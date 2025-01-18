import { Box, Center, useDisclosure } from "@chakra-ui/react";
import DashboardCard from "../../../components/dashboard-card";
import { useState } from "react";
import { DeleteMedicationDataFn } from "../../../query-mutation-services/medication-data-fn";
import { useParams } from "@tanstack/react-router";
import DeleteModal from "../../../components/delete-modal";
import { db } from "../../../db/db";
import type { MedicationsStateExtend } from "@repo/types/dexie-state";
import Medication_popup from "../medication/medication-popup";
import { motion, AnimatePresence } from "framer-motion";
const MotionBox = motion(Box);

type ActiveInactiveProblemListTabProps = {
	filterData: MedicationsStateExtend[];
	active?: boolean;
	selectedDate: Date | null;
};

const ActiveInactiveMedicationListTab = (
	props: ActiveInactiveProblemListTabProps,
) => {
	const patientId = useParams({
		from: "/medication/list/$patientId",
		select: (params) => params.patientId,
	});
	const [activeId, setActiveId] = useState<string>("");
	const deleteMedication = DeleteMedicationDataFn();
	let ActiveProblemList = props.filterData.filter(
		props.active
			? (item) => item.status === "Active"
			: (item) => item.status === "InActive",
	);
	if (props.selectedDate) {
		ActiveProblemList = ActiveProblemList.filter((item) => {
			const itemDate = new Date(item.created_at);
			return itemDate.toDateString() === props.selectedDate?.toDateString();
		});
	}

	const sortedData = ActiveProblemList.slice().sort((a, b) => {
		const dateA = new Date(a.created_at);
		const dateB = new Date(b.created_at);
		return dateB.getTime() - dateA.getTime();
	});
	const { isOpen: isDeleteModalOpen, onOpen, onClose } = useDisclosure();

	const [deletedata, setDeletedata] = useState("");

	const handleDelete = async (id: string) => {
		try {
			await db.medications.clear();
			await deleteMedication.mutateAsync({
				id,
				patient_id: patientId,
				last_updated_input: await db.getLastUpdated(),
			});
		} catch (error) {
			console.error("Error deleting complaint:", error);
		}
		onClose();
	};
	const {
		isOpen: isModalOpen,
		onOpen: openModalBase,
		onClose: closeModal,
	} = useDisclosure();
	const openModal = (id: string) => {
		setActiveId(id);
		openModalBase();
	};
	return (
		<div>
			<Box>
				{sortedData?.map((items) => (
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
										heading_1={"Drug :"}
										result_1={items.drug}
										heading_2={"Instruction :"}
										result_2={items.instruction}
										open_model={() => openModal(items.id)}
										handleDelete={() => {
											setDeletedata(items.id);
											onOpen();
										}}
										editpath={`/medication/edit/${patientId}/${items.id}`}
									/>
								</MotionBox>
							</AnimatePresence>
						</Center>
						<Medication_popup
							open={isModalOpen}
							close={closeModal}
							id={activeId}
							patientId={""}
						/>
						<DeleteModal
							open={isDeleteModalOpen}
							close={onClose}
							handle_delete={() => handleDelete(deletedata)}
						/>
					</div>
				))}
			</Box>
		</div>
	);
};

export default ActiveInactiveMedicationListTab;
