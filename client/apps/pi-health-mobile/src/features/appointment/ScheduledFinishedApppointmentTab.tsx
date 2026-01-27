import { Box, Center, useDisclosure } from "@chakra-ui/react";
import DashboardCard from "../../components/dashboard-card";
import AppointmentPopup from "./appointment-popup";
import { deleteAppointmentFn } from "../../query-mutation-services/appointment-data-fn";
import { useState } from "react";
import DeleteModal from "../../components/delete-modal";
import { db } from "../../db/db";
import type { AppointmentStateExtend } from "@repo/types/dexie-state";
import { motion, AnimatePresence } from "framer-motion";

const MotionBox = motion(Box);

export type ScheduledFinishedApppointmentTabProps = {
	filterData: AppointmentStateExtend[];
	scheduled?: boolean;
};

const ScheduledFinishedApppointmentTab = (
	props: ScheduledFinishedApppointmentTabProps,
) => {
	const deleteAppointment = deleteAppointmentFn();
	const [activeId, setActiveId] = useState<string>("");
	const [deletedata, setDeletedata] = useState({
		id: "",
		patient_id: "",
		doctor_id: "",
	});
	const today = new Date();
	const appointmentList = props.filterData
		.filter(
			props.scheduled
				? (item) => new Date(item.date) > today
				: (item) => new Date(item.date) < today,
		)
		.sort((a, b) => {
			const dateA = new Date(a.created_at);
			const dateB = new Date(b.created_at);
			return dateB.getTime() - dateA.getTime();
		});
	const { isOpen: isDeleteModalOpen, onOpen, onClose } = useDisclosure();
	const {
		isOpen: isModalOpen,
		onOpen: openModalBase,
		onClose: closeModal,
	} = useDisclosure();

	const openModal = (id: string) => {
		setActiveId(id);
		openModalBase();
	};

	const handleDelete = async (
		id: string,
		patient_id: string,
		doctor_id: string,
	) => {
		try {
			await db.appointments.where({ id }).delete();
			await deleteAppointment.mutateAsync({
				id: id,
				patient_id: patient_id,
				doctor_id: doctor_id,
				last_updated_input: await db.getLastUpdated(),
			});
		} catch (error) {
			console.error("Error deleting complaint:", error);
		}
		onClose();
	};

	return (
		<div>
			<Box>
				{appointmentList?.map((item) => (
					<div key={item.created_at}>
						<Center>
							<AnimatePresence>
								<MotionBox
									key={item.created_at}
									initial={{ opacity: 0, y: -50 }}
									animate={{ opacity: 1, y: 0 }}
									exit={{ opacity: 0, y: -50 }}
									transition={{ duration: 0.65 }}
									width={{ md: "80%", base: "90%", lg: "70%" }}
								>
									<DashboardCard
										heading_1={"Patient Name:"}
										result_1={item.patient_name}
										heading_2={"Visit Type:"}
										result_2={item.visit}
										open_model={() => openModal(item.id)}
										editpath={`/appointment/edit/${item.id}`}
										handleDelete={() => {
											setDeletedata({
												id: item.id,
												patient_id: item.patient_id,
												doctor_id: item.doctor_id,
											});
											onOpen();
										}}
									/>
								</MotionBox>
							</AnimatePresence>
						</Center>
						<AppointmentPopup
							open={isModalOpen && activeId === item.id}
							close={closeModal}
							id={activeId}
						/>
						<DeleteModal
							open={isDeleteModalOpen}
							close={onClose}
							handle_delete={() =>
								handleDelete(
									deletedata.id,
									deletedata.patient_id,
									deletedata.doctor_id,
								)
							}
						/>
					</div>
				))}
			</Box>
		</div>
	);
};

export default ScheduledFinishedApppointmentTab;
