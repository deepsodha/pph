import type { AllergiesStateExtend } from "@repo/types/dexie-state";
import { DeleteAllergyDataFn } from "../../../query-mutation-services/allergy-data-fn";
import { Box, Center, useDisclosure } from "@chakra-ui/react";
import { useParams } from "@tanstack/react-router";
import { useState } from "react";
import { db } from "../../../db/db";
import AllergyPopup from "./allergy-popup";
import DashboardCard from "../../../components/dashboard-card";
import DeleteModal from "../../../components/delete-modal";
import { motion, AnimatePresence } from "framer-motion";
const MotionBox = motion(Box);
type ActiveInactiveAllergyListProps = {
	filterData: AllergiesStateExtend[];
	active?: boolean;
};
const ActiveInactiveAllergyListTab = (
	props: ActiveInactiveAllergyListProps,
) => {
	const [activeId, setActiveId] = useState<string>("");
	const patientId = useParams({
		from: "/allergy/list/$patientId",
		select: (params) => params.patientId,
	});
	const deleteAllergy = DeleteAllergyDataFn();
	const ActiveAllergyList = props.filterData
		.filter(
			props.active
				? (item) => item.stauts === "Active"
				: (item) => item.stauts === "InActive",
		)
		.slice()
		.sort((a, b) => {
			const dateA = new Date(a.created_at);
			const dateB = new Date(b.created_at);
			return dateB.getTime() - dateA.getTime();
		});
	const { isOpen: isDeleteModalOpen, onOpen, onClose } = useDisclosure();

	const [deletedata, setDeletedata] = useState("");

	const handleDelete = async (id: string) => {
		try {
			await db.allergy.where({ id }).delete();

			await deleteAllergy.mutateAsync({
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
				{ActiveAllergyList?.map((items) => (
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
										heading_1={"Allergen Name :"}
										result_1={items.allergen}
										heading_2={"Reaction :"}
										result_2={items.reaction}
										open_model={() => openModal(items.id)}
										handleDelete={() => {
											setDeletedata(items.id);
											onOpen();
										}}
										editpath={`/allergy/edit/${patientId}/${items.id}`}
									/>
								</MotionBox>
							</AnimatePresence>
						</Center>
						<AllergyPopup open={isModalOpen} close={closeModal} id={activeId} />
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

export default ActiveInactiveAllergyListTab;
