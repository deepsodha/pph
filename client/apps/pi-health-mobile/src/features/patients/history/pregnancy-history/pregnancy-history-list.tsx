import { Fade, Center, Box, useDisclosure } from "@chakra-ui/react";
import { useMountEffect } from "@react-hookz/web";
import { useParams } from "@tanstack/react-router";
import { useLiveQuery } from "dexie-react-hooks";
import { useAtom } from "jotai";
import { useState } from "react";
import {
	headerText,
	addImage,
	addPath,
	addValue,
	formValue,
	dashboardValue,
} from "../../../../atoms/header";
import SearchBar from "../../../../components/search-bar";
import { db } from "../../../../db/db";

import Icon from "../../../../assets/Add Service Location Icon.svg";
import PregnancyHistoryCard from "./pregnancy-history-card";
import { DeleteObAndPregnancyFn } from "../../../../query-mutation-services/ob-and-pregnancy";
import DeleteModal from "../../../../components/delete-modal";

const pregnancyHistoryList = () => {
	const [, setHeaderText] = useAtom(headerText);
	const [, setaddImage] = useAtom(addImage);
	const [, setaddPath] = useAtom(addPath);
	const [, setAddValue] = useAtom(addValue);
	const [, setFormValue] = useAtom(formValue);
	const [, setDashboardValue] = useAtom(dashboardValue);

	useMountEffect(() => {
		setHeaderText("Add OB & Pregnancy History");
		setaddImage(Icon);
		setaddPath(`/pregnancyhistory/add/${patientId}`);
		setAddValue(false);
		setFormValue(false);
		setDashboardValue(false);
	});
	const [searchQuery, setSearchQuery] = useState("");

	const { patientId } = useParams({ from: "patientId" });
	const { isOpen: isDeleteModalOpen, onOpen, onClose } = useDisclosure();
	const [deletedata, setDeletedata] = useState("");

	const deletePregnancyHistory = DeleteObAndPregnancyFn();
	const handleDelete = async (id: string) => {
		try {
			await db.obandpregnanacy.where({ id }).delete();

			await deletePregnancyHistory.mutateAsync({
				id,
				patient_id: patientId,
				last_updated_input: await db.getLastUpdated(),
			});
		} catch (error) {
			console.error("Error deleting medication:", error);
		}
		onClose();
	};

	const pregnancyhistoryData = useLiveQuery(() => db.obandpregnanacy.toArray());
	const pregnancyhistoryDataById = pregnancyhistoryData?.filter(
		(item) => item.patient_id === patientId,
	);

	const filteredData = (pregnancyhistoryDataById || []).filter((item) =>
		item.age_onset_of_menses.toString().includes(searchQuery),
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
								<SearchBar
									value={searchQuery}
									onchange={(e) => setSearchQuery(e.target.value)}
									placeholder={"Family History Details"}
								/>
							</Box>
						</Center>
					</Box>
					<Center>
						<Box
							zIndex={15}
							mt={5}
							bgColor={"#F7F7F9"}
							width={{ md: "80%", base: "90%", lg: "70%" }}
						>
							{sortedData?.map((items) => (
								<div key={items.created_at}>
									<Center>
										<Box cursor={"pointer"} width={"100%"}>
											<PregnancyHistoryCard
												age_onset_of_menses={items.age_onset_of_menses}
												age_at_menopause={items.age_at_menopause}
												comments_ob={items.comments_ob}
												total_pregnancy={items.total_pregnancy}
												full_term={items.full_term}
												pre_term={items.pre_term}
												miscarriages={items.miscarriages}
												living={items.living}
												comments_pregnancy={items.comments_pregnancy}
												editpath={`/pregnancyhistory/edit/${items.patient_id}/${items.id}`}
												handleDelete={() => {
													setDeletedata(items.id);
													onOpen();
												}}
											/>
										</Box>
									</Center>
									<DeleteModal
										open={isDeleteModalOpen}
										close={onClose}
										handle_delete={() => handleDelete(deletedata)}
									/>
								</div>
							))}
						</Box>
					</Center>
				</Box>
			</Fade>
		</div>
	);
};

export default pregnancyHistoryList;
