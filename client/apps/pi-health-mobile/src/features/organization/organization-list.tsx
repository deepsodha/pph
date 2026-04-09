import { Box, Center, Fade, Heading, useDisclosure } from "@chakra-ui/react";
import Organization from "../../assets/Organization Icon.svg";
import DashboardCard from "../../components/dashboard-card";
import SearchBar from "../../components/search-bar";
import {
	DeleteOrganizationDataFn,
	GetOrganizationDataFn,
} from "../../query-mutation-services/organization-data-fn";
import { useAtom } from "jotai";
import OrganizationPopup from "./organization-popup";
import { useState } from "react";
import {
	headerText,
	addImage,
	addPath,
	addValue,
	dashboardValue,
	formValue,
} from "../../atoms/header";
import DeleteModal from "../../components/delete-modal";
import { useMountEffect } from "@react-hookz/web";
const OrganizationList = () => {
	const [, setHeaderText] = useAtom(headerText);
	const [, setaddImage] = useAtom(addImage);
	const [, setaddPath] = useAtom(addPath);
	const [, setAddValue] = useAtom(addValue);
	const [, setFormValue] = useAtom(formValue);
	const [, setDashboardValue] = useAtom(dashboardValue);

	const { isOpen: isDeleteModalOpen, onOpen, onClose } = useDisclosure();

	const [deletedata, setDeletedata] = useState("");
	const [searchQuery, setSearchQuery] = useState("");

	useMountEffect(() => {
		setHeaderText("Add New Organization");
		setaddImage(Organization);
		setaddPath("/organization/add");
		setAddValue(false);
		setFormValue(false);
		setDashboardValue(false);
	});
	const {
		isOpen: isModalOpen,
		onOpen: openModalBase,
		onClose: closeModal,
	} = useDisclosure();

	const [activeId, setActiveId] = useState<string>("");

	const openModal = (id: string) => {
		setActiveId(id);
		openModalBase();
	};
	const organization = GetOrganizationDataFn();
	const deleteOrganization = DeleteOrganizationDataFn();

	const filteredData = (organization.data?.result || []).filter((item) =>
		item.mobile_number.toString().includes(searchQuery),
	);

	const handleDelete = async (id: string) => {
		try {
			await deleteOrganization.mutateAsync({ id });
			organization.refetch();
		} catch (error) {
			console.error("Error deleting organization:", error);
		}
		onClose();
	};
	return (
		<div>
			<Fade in={true}>
				<Box minHeight={"100vh"} bgColor={"#F7F7F9"}>
					<Box
						position="sticky"
						width={"100%"}
						zIndex={10}
						bgColor={"#F7F7F9"}
						top={0}
						left={0}
						right={0}
					>
						<Center>
							<Box width={{ md: "80%", base: "90%", lg: "70%" }}>
								<Heading
									color="#095FBA"
									pt="1"
									fontSize="20px"
									mt="16px"
									mb="10px"
								>
									Organization List
								</Heading>
								<SearchBar
									value={searchQuery}
									onchange={(e) => setSearchQuery(e.target.value)}
									placeholder={"Search by Mobile Number"}
								/>
							</Box>
						</Center>
					</Box>
					<Box zIndex={15} mt={5} bgColor={"#F7F7F9"}>
						{filteredData.map((item) => (
							<div key={item.id}>
								<Center>
									<Box
										cursor={"pointer"}
										width={{ md: "80%", base: "90%", lg: "70%" }}
									>
										<DashboardCard
											heading_1={"Organization Name:"}
											result_1={item.name}
											heading_2={"Phone No:"}
											result_2={item.mobile_number}
											open_model={() => openModal(item.id)}
											handleDelete={() => {
												setDeletedata(item.id);
												onOpen();
											}}
											editpath={`/organization/edit/${item.id}`}
										/>
									</Box>
								</Center>
								<OrganizationPopup
									open={isModalOpen}
									close={closeModal}
									id={activeId}
								/>
								<DeleteModal
									open={isDeleteModalOpen}
									close={onClose}
									handle_delete={() => handleDelete(deletedata)}
								/>
							</div>
						))}
					</Box>
				</Box>
			</Fade>
		</div>
	);
};

export default OrganizationList;
