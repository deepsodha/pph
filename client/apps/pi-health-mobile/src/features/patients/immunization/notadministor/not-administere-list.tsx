import { Box, Center, Fade, useDisclosure } from "@chakra-ui/react";
import { Heading } from "@chakra-ui/react";
import NotAdministeredIcon from "../../../../assets/Not administered icon 36 x 36.svg";
import DashboardCard from "../../../../components/dashboard-card";
import { useAtom } from "jotai";
import SearchBar from "../../../../components/search-bar";
import NotAdministeredPopUp from "./not-administere-popup";
import { useState } from "react";
import { DeleteNotAdministerDataFn } from "../../../../query-mutation-services/not-administer-data-fn";
import {
	headerText,
	addImage,
	addPath,
	addValue,
	dashboardValue,
	formValue,
} from "../../../../atoms/header";
import { useParams } from "@tanstack/react-router";
import DeleteModal from "../../../../components/delete-modal";
import { db } from "../../../../db/db";
import { useLiveQuery } from "dexie-react-hooks";
import { useMountEffect } from "@react-hookz/web";

const NotAdministeredList = () => {
	const [, setHeaderText] = useAtom(headerText);
	const [, setaddImage] = useAtom(addImage);
	const [, setaddPath] = useAtom(addPath);
	const [, setAddValue] = useAtom(addValue);
	const [, setFormValue] = useAtom(formValue);
	const [, setDashboardValue] = useAtom(dashboardValue);
	useMountEffect(() => {
		setHeaderText("Add Not Administer");
		setaddImage(NotAdministeredIcon);
		setaddPath(`/notAdminister/add/${patientId}`);
		setAddValue(false);
		setFormValue(false);
		setDashboardValue(false);
	});

	const { patientId } = useParams({ from: "patientId" });
	const [searchQuery, setSearchQuery] = useState("");

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

	const { isOpen: isDeleteModalOpen, onOpen, onClose } = useDisclosure();
	const [deletedata, setDeletedata] = useState("");
	const deleteNotAdminister = DeleteNotAdministerDataFn();
	const notAdministerList = useLiveQuery(() => db.notadminister.toArray());
	const notAdministerListById = notAdministerList?.filter(
		(item) => item.patient_id === patientId,
	);

	const filteredData = (notAdministerListById || []).filter((item) =>
		item.vaccine.toString().toLowerCase().includes(searchQuery.toLowerCase()),
	);
	const handleDelete = async (id: string) => {
		try {
			await db.notadminister.where({ id }).delete();

			await deleteNotAdminister.mutateAsync({
				id,
				patient_id: patientId,
				last_updated_input: await db.getLastUpdated(),
			});
		} catch (error) {
			console.error("Error deleting Administer:", error);
		}
		onClose();
	};

	const sortedData = filteredData.slice().sort((a, b) => {
		const dateA = new Date(a.created_at);
		const dateB = new Date(b.created_at);
		return dateB.getTime() - dateA.getTime();
	});

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
						{/* <Progress size="xs" isIndeterminate={notAdminister.isLoading} /> */}
						<Center>
							<Box width={{ md: "80%", base: "90%", lg: "70%" }}>
								<Heading
									color="#095FBA"
									pt="1"
									fontSize="20px"
									mt="16px"
									mb="10px"
								>
									Not Administered List
								</Heading>
								<SearchBar
									value={searchQuery}
									onchange={(e) => setSearchQuery(e.target.value)}
									placeholder={"Search by Vaccine"}
								/>
							</Box>
						</Center>
					</Box>
					<Box zIndex={15} mt={5} bgColor={"#F7F7F9"}>
						{sortedData.map((items) => (
							<div key={items.created_at}>
								<Center>
									<Box
										cursor={"pointer"}
										width={{ md: "80%", base: "90%", lg: "70%" }}
									>
										<DashboardCard
											heading_1={" Patient Name :"}
											result_1={items.vaccine}
											heading_2={"Type :"}
											result_2={items.types}
											open_model={() => openModal(items.id)}
											handleDelete={() => {
												setDeletedata(items.id);
												onOpen();
											}}
											editpath={`/notAdminister/edit/${patientId}/${items.id}`}
										/>
									</Box>
								</Center>
								<NotAdministeredPopUp
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
							</div>
						))}
					</Box>
				</Box>
			</Fade>
		</div>
	);
};
export default NotAdministeredList;
