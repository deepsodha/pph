import { Box, Center, Fade, Heading, useDisclosure } from "@chakra-ui/react";
import AddPatient from "../../assets/New Prescription Icon.svg";
import CalenderBar from "../../components/calender-bar";
import SearchBar from "../../components/search-bar";
import PrescriptionPopup from "./prescription-popup";
import { useState } from "react";
import { useAtom } from "jotai";
import { deletePrescriptionFn } from "../../query-mutation-services/prescription-data-fn";
import {
	headerText,
	addImage,
	addPath,
	addValue,
	dashboardValue,
	formValue,
	displayMenu,
} from "../../atoms/header";
import DashboardCard from "../../components/dashboard-card";
import DeleteModal from "../../components/delete-modal";
import { db } from "../../db/db";
import { useLiveQuery } from "dexie-react-hooks";
import { useMountEffect } from "@react-hookz/web";

import { motion, AnimatePresence } from "framer-motion";
const MotionBox = motion(Box);
const PrescriptionList = () => {
	const [, setHeaderText] = useAtom(headerText);
	const [, setaddImage] = useAtom(addImage);
	const [, setaddPath] = useAtom(addPath);
	const [, setAddValue] = useAtom(addValue);
	const [, setFormValue] = useAtom(formValue);
	const [, setmenu] = useAtom(displayMenu);

	const [, setDashboardValue] = useAtom(dashboardValue);

	const [deletedata, setDeletedata] = useState("");
	const { isOpen: isDeleteModalOpen, onOpen, onClose } = useDisclosure();
	useMountEffect(() => {
		setHeaderText("Add Prescription");
		setaddImage(AddPatient);
		setaddPath("/prescription/add");
		setAddValue(true);
		setFormValue(false);
		setDashboardValue(false);
		setmenu(true);
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
	const patientId = "244c780c-beed-4da3-85e4-9f46625e1866";
	const deletePatient = deletePrescriptionFn();
	const handleDelete = async (id: string) => {
		try {
			await db.prescription.where({ id }).delete();

			await deletePatient.mutateAsync({
				id,
				patient_id: patientId,
				last_updated_input: await db.getLastUpdated(),
			});
		} catch (error) {
			console.error("Error deleting complaint:", error);
		}
		onClose();
	};

	const [searchQuery, setSearchQuery] = useState("");
	const prescriptionData = useLiveQuery(() => db.prescription.toArray());
	const [selectedDate, setSelectedDate] = useState<Date | null>(null);

	const handleDateSelect = (date: Date) => {
		setSelectedDate(date);
	};

	let filteredData = prescriptionData || [];

	if (selectedDate) {
		filteredData = filteredData.filter((item) => {
			const itemDate = new Date(item.date);
			return itemDate.toDateString() === selectedDate.toDateString();
		});
	}

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
									Prescription List
								</Heading>
								<CalenderBar
									heading={"Select Date	"}
									onSelectDate={handleDateSelect}
								/>
								<SearchBar
									value={searchQuery}
									onchange={(e) => setSearchQuery(e.target.value)}
									placeholder={"Search by Patient Name"}
								/>
							</Box>
						</Center>
					</Box>
					<Box zIndex={15} mt={5} bgColor={"#F7F7F9"} pb="10%">
						{sortedData?.map((items) => (
							<Box key={items.id}>
								<Center>
									<AnimatePresence>
										<MotionBox
											key={items.created_at}
											initial={{ opacity: 0, y: -50 }}
											animate={{ opacity: 1, y: 0 }}
											exit={{ opacity: 0, y: -50 }}
											transition={{ duration: 0.65 }}
											width={{ md: "80%", base: "90%", lg: "70%" }}
										>
											<DashboardCard
												heading_1={"Patient Name:"}
												result_1={items.patient_name}
												heading_2={"Prescription Name:"}
												result_2={items.presecription_name}
												open_model={() => openModal(items.id)}
												editpath={`/prescription/edit/${items.id}`}
												handleDelete={() => {
													setDeletedata(items.id);
													onOpen();
												}}
											/>
										</MotionBox>
									</AnimatePresence>
								</Center>
								<PrescriptionPopup
									close={closeModal}
									id={activeId}
									open={isModalOpen}
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
			</Fade>
		</div>
	);
};

export default PrescriptionList;
