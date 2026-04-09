import { Box, Center, useDisclosure } from "@chakra-ui/react";
import DashboardCard from "../../../components/dashboard-card";
import Problem_popup from "./problem-popup";
import { useState } from "react";
import { deleteProblemFn } from "../../../query-mutation-services/problem-data-fn";
import { useParams } from "@tanstack/react-router";
import DeleteModal from "../../../components/delete-modal";
import { db } from "../../../db/db";
import type { ProblemStateExtend } from "@repo/types/dexie-state";
import { motion, AnimatePresence } from "framer-motion";

const MotionBox = motion(Box);

type ActiveInactiveProblemListTabProps = {
	filterData: ProblemStateExtend[];
	active?: boolean;
};

const ActiveInactiveProblemListTab = (
	props: ActiveInactiveProblemListTabProps,
) => {
	const patientId = useParams({
		from: "/problem/list/$patientId",
		select: (params) => params.patientId,
	});
	const [activeId, setActiveId] = useState<string>("");
	const deleteProblem = deleteProblemFn();
	const ActiveProblemList = props.filterData
		.filter(
			props.active
				? (item) => item.status === "Active"
				: (item) => item.status === "InActive",
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
			await db.problems.where({ id }).delete();

			await deleteProblem.mutateAsync({
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
				{ActiveProblemList?.map((items) => (
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
										heading_1={"Problem Name :"}
										result_1={items.issue}
										heading_2={"Type :"}
										result_2={items.issue_type}
										open_model={() => openModal(items.id)}
										handleDelete={() => {
											setDeletedata(items.id);
											onOpen();
										}}
										editpath={`/problem/edit/${patientId}/${items.id}`}
									/>
								</MotionBox>
							</AnimatePresence>
						</Center>
						<Problem_popup
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
		</div>
	);
};

export default ActiveInactiveProblemListTab;
