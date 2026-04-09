import { Fade, Center, Box, useDisclosure } from "@chakra-ui/react";
import { useMountEffect } from "@react-hookz/web";
import { useParams } from "@tanstack/react-router";
import { useAtom } from "jotai";
import {
	headerText,
	addImage,
	addPath,
	addValue,
	formValue,
	dashboardValue,
} from "../../../../atoms/header";
import Icon from "../../../../assets/Add Service Location Icon.svg";
import SearchBar from "../../../../components/search-bar";
import { useState } from "react";
import { useLiveQuery } from "dexie-react-hooks";
import { db } from "../../../../db/db";
import PastSurgicalHistoryCard from "./past-surgical-history-card";
import { DeletePastSurgicalHistoryFn } from "../../../../query-mutation-services/past-surgical-history";
import DeleteModal from "../../../../components/delete-modal";
const familyHistoryList = () => {
	const [, setHeaderText] = useAtom(headerText);
	const [, setaddImage] = useAtom(addImage);
	const [, setaddPath] = useAtom(addPath);
	const [, setAddValue] = useAtom(addValue);
	const [, setFormValue] = useAtom(formValue);
	const [, setDashboardValue] = useAtom(dashboardValue);

	useMountEffect(() => {
		setHeaderText("Add Past Surgical History");
		setaddImage(Icon);
		setaddPath(`/pastsurgicalhistory/add/${patientId}`);
		setAddValue(false);
		setFormValue(false);
		setDashboardValue(false);
	});
	const [searchQuery, setSearchQuery] = useState("");

	const { patientId } = useParams({ from: "patientId" });
	const { isOpen: isDeleteModalOpen, onOpen, onClose } = useDisclosure();
	const [deletedata, setDeletedata] = useState("");

	const deletePastSurgicalHistory = DeletePastSurgicalHistoryFn();
	const handleDelete = async (id: string) => {
		try {
			await db.pastsurgicalhistory.where({ id }).delete();

			await deletePastSurgicalHistory.mutateAsync({
				id,
				patient_id: patientId,
				last_updated_input: await db.getLastUpdated(),
			});
		} catch (error) {
			console.error("Error deleting medication:", error);
		}
		onClose();
	};

	const pastsurgicalHistory = useLiveQuery(() =>
		db.pastsurgicalhistory.toArray(),
	);
	const pastSurgicalhistoryDataById = pastsurgicalHistory?.filter(
		(item) => item.patient_id === patientId,
	);

	const filteredData = (pastSurgicalhistoryDataById || []).filter((item) =>
		item?.common_surgeries
			?.toString()
			.toLowerCase()
			.includes(searchQuery.toLowerCase()),
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
									placeholder={"Past Surgical History Details"}
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
											<PastSurgicalHistoryCard
												editpath={`/pastsurgicalhistory/edit/${items.id}/${items.patient_id}`}
												handleDelete={() => {
													setDeletedata(items.id);
													onOpen();
												}}
												common_surgeries={items.common_surgeries}
												comments={items.comments}
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

export default familyHistoryList;
