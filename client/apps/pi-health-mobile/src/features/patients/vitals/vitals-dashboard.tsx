import { Box, Center, Fade, Heading, useDisclosure } from "@chakra-ui/react";
import Vital from "../../../assets/Vital Icon 36 x 36.svg";
import { DeleteVitalsDataFn } from "../../../query-mutation-services/vital-data-fn";
import { useAtom } from "jotai";
import { useState } from "react";
import {
	headerText,
	addImage,
	addPath,
	addValue,
	dashboardValue,
	formValue,
} from "../../../atoms/header";
import { useParams } from "@tanstack/react-router";
import VitalsPopup from "./vitals-popup";
import DashboardCard from "../../../components/dashboard-card";
import SearchBar from "../../../components/search-bar";
import DeleteModal from "../../../components/delete-modal";
import { db } from "../../../db/db";
import { useLiveQuery } from "dexie-react-hooks";
import { useMountEffect } from "@react-hookz/web";
import { motion, AnimatePresence } from "framer-motion";
const MotionBox = motion(Box);

const VitalsDashboard = () => {
	const [, setHeaderText] = useAtom(headerText);
	const [, setaddImage] = useAtom(addImage);
	const [, setaddPath] = useAtom(addPath);
	const [, setAddValue] = useAtom(addValue);
	const [, setFormValue] = useAtom(formValue);
	const [, setDashboardValue] = useAtom(dashboardValue);
	const { isOpen: isDeleteModalOpen, onOpen, onClose } = useDisclosure();

	const [searchQuery, setSearchQuery] = useState("");
	const [deletedata, setDeletedata] = useState("");

	useMountEffect(() => {
		setHeaderText("Add Vitals");
		setaddImage(Vital);
		setaddPath(`/vitals/add/${patientId}`);
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
	const patientId = useParams({
		from: "/vitals/list/$patientId",
		select: (params) => params.patientId,
	});

	const openModal = (id: string) => {
		setActiveId(id);
		openModalBase();
	};
	const vitalsData = useLiveQuery(() => db.vitals.toArray());
	const deleteVitals = DeleteVitalsDataFn();
	const headerData = useLiveQuery(() => db.patients.toArray());
	const headerFilterData = headerData?.find((item) => item.id === patientId);

	const handleDelete = async (id: string) => {
		try {
			await db.vitals.where({ id }).delete();
			await deleteVitals.mutateAsync({
				id,
				patient_id: patientId,
				last_updated_input: await db.getLastUpdated(),
			});
		} catch (error) {
			console.error("Error deleting vital:", error);
		}
		onClose();
	};
	const vitalsDataById = vitalsData?.filter(
		(item) => item.patient_id === patientId,
	);
	const filteredData = (vitalsDataById || []).filter((item) =>
		item.bmi?.toString().includes(searchQuery),
	);

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
						top={0}
						zIndex={10}
						bgColor={"#F7F7F9"}
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
									<span style={{ color: "black" }}>
										{`${headerFilterData?.user.first_name}'s `}
									</span>
									Vitals List
								</Heading>
								<SearchBar
									value={searchQuery}
									onchange={(e) => setSearchQuery(e.target.value)}
									placeholder={"Search by BMI Name"}
								/>
							</Box>
						</Center>
					</Box>
					<Box zIndex={15} mt={5} bgColor={"#F7F7F9"}>
						{sortedData.map((items) => (
							<div key={items.created_at}>
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
												heading_1={"BMI :"}
												result_1={items.bmi?.toString() || ""}
												heading_2={"Comments :"}
												result_2={items.comments}
												open_model={() => openModal(items.id)}
												handleDelete={() => {
													setDeletedata(items.id);
													onOpen();
												}}
												editpath={`/vitals/edit/${items.patient_id}/${items.id}`}
											/>
										</MotionBox>
									</AnimatePresence>
								</Center>
								<VitalsPopup
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

export default VitalsDashboard;
