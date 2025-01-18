import { Box, useDisclosure, Heading, Center, Fade } from "@chakra-ui/react";
import Minister from "../../../../assets/New Administered Icon.svg";
import DashboardCard from "../../../../components/dashboard-card";
import SearchBar from "../../../../components/search-bar";
import AdministerPopup from "./administer-popup";

import { useAtom } from "jotai";
import { DeleteAdministerDataFn } from "../../../../query-mutation-services/administer-data-fn";
import { useState } from "react";
import {
	addImage,
	addPath,
	addValue,
	dashboardValue,
	formValue,
	headerText,
} from "../../../../atoms/header";
import { useParams } from "@tanstack/react-router";
import DeleteModal from "../../../../components/delete-modal";
import { db } from "../../../../db/db";
import { useLiveQuery } from "dexie-react-hooks";
import { useMountEffect } from "@react-hookz/web";
const AdministerList = () => {
	const [, setHeaderText] = useAtom(headerText);
	const [, setaddImage] = useAtom(addImage);
	const [, setaddPath] = useAtom(addPath);
	const [, setAddValue] = useAtom(addValue);
	const [, setFormValue] = useAtom(formValue);
	const [, setDashboardValue] = useAtom(dashboardValue);
	const { patientId } = useParams({ from: "patientId" });

	const { isOpen: isDeleteModalOpen, onOpen, onClose } = useDisclosure();

	const [searchQuery, setSearchQuery] = useState("");
	const [deletedata, setDeletedata] = useState("");
	const deleteAdminister = DeleteAdministerDataFn();
	const administerData = useLiveQuery(() => db.administer.toArray());

	const handleDelete = async (id: string) => {
		try {
			await db.administer.where({ id }).delete();

			await deleteAdminister.mutateAsync({
				id,
				patient_id: patientId,
				last_updated_input: await db.getLastUpdated(),
			});
		} catch (error) {
			console.error("Error deleting administer:", error);
		}
		onClose();
	};

	useMountEffect(() => {
		setHeaderText("Add New Administer");
		setaddImage(Minister);
		setaddPath(`/administer/add/${patientId}`);
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
	const administerDataById = administerData?.filter(
		(item) => item.patient_id === patientId,
	);
	const filteredData = (administerDataById || []).filter((item) =>
		item.vaccine.toString().toLowerCase().includes(searchQuery.toLowerCase()),
	);

	const sortedData = filteredData.slice().sort((a, b) => {
		const dateA = new Date(a.created_at);
		const dateB = new Date(b.created_at);
		return dateB.getTime() - dateA.getTime();
	});

	return (
		<Fade in={true}>
			<Box>
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
									Administer List
								</Heading>
								<SearchBar
									value={searchQuery}
									onchange={(e) => setSearchQuery(e.target.value)}
									placeholder={"Search By Vaccine"}
								/>
							</Box>
						</Center>
					</Box>
					<Box zIndex={15} mt={5} bgColor={"#F7F7F9"}>
						{sortedData?.map((item) => (
							<Box key={item.created_at}>
								<Center>
									<Box
										cursor={"pointer"}
										width={{ md: "80%", base: "90%", lg: "70%" }}
									>
										<DashboardCard
											heading_1={"Vaccine:"}
											result_1={item.vaccine}
											heading_2={"Type:"}
											result_2={item.types}
											open_model={() => openModal(item.id)}
											handleDelete={() => {
												setDeletedata(item.id);
												onOpen();
											}}
											editpath={`/administer/edit/${patientId}/${item.id}`}
										/>
									</Box>
								</Center>
								<AdministerPopup
									open={isModalOpen}
									close={closeModal}
									id={activeId}
									patientId={patientId}
								/>
								<DeleteModal
									open={isDeleteModalOpen}
									close={onClose}
									handle_delete={() => handleDelete(deletedata)}
								/>
							</Box>
						))}
					</Box>
				</Box>
			</Box>
		</Fade>
	);
};

export default AdministerList;
