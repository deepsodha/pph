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
import { DeleteFamilyHistoryDataFn } from "../../../../query-mutation-services/family-history-data-fn";
import DeleteModal from "../../../../components/delete-modal";
import FamilyHistoryCard from "./family-history-card";
const familyHistoryList = () => {
	const [, setHeaderText] = useAtom(headerText);
	const [, setaddImage] = useAtom(addImage);
	const [, setaddPath] = useAtom(addPath);
	const [, setAddValue] = useAtom(addValue);
	const [, setFormValue] = useAtom(formValue);
	const [, setDashboardValue] = useAtom(dashboardValue);

	useMountEffect(() => {
		setHeaderText("Add Family History");
		setaddImage(Icon);
		setaddPath(`/familyhistory/add/${patientId}`);
		setAddValue(false);
		setFormValue(false);
		setDashboardValue(false);
	});
	const [searchQuery, setSearchQuery] = useState("");

	const { patientId } = useParams({ from: "patientId" });
	const { isOpen: isDeleteModalOpen, onOpen, onClose } = useDisclosure();
	const [deletedata, setDeletedata] = useState("");

	const deleteFamilyHistory = DeleteFamilyHistoryDataFn();
	const handleDelete = async (id: string) => {
		try {
			await db.familyhistory.where({ id }).delete();

			await deleteFamilyHistory.mutateAsync({
				id,
				patient_id: patientId,
				last_updated_input: await db.getLastUpdated(),
			});
		} catch (error) {
			console.error("Error deleting medication:", error);
		}
		onClose();
	};

	const familyhistoryData = useLiveQuery(() => db.familyhistory.toArray());
	const familyhistoryDataById = familyhistoryData?.filter(
		(item) => item.patient_id === patientId,
	);

	const filteredData = (familyhistoryDataById || []).filter((item) =>
		item.family_member
			.toString()
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
											<FamilyHistoryCard
												family_member={items.family_member}
												health_status={items.health_status}
												general={items.general}
												cancer={items.cancer}
												comments={items.comments}
												editpath={`/familyhistory/edit/${items.patient_id}/${items.id}`}
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

export default familyHistoryList;
