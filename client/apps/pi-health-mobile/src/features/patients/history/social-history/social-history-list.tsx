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
import DeleteModal from "../../../../components/delete-modal";
import { DeleteSocialHistoryFn } from "../../../../query-mutation-services/social-history";
import SocialHistoryCard from "./social-history-card";
const socialHistoryList = () => {
	const [, setHeaderText] = useAtom(headerText);
	const [, setaddImage] = useAtom(addImage);
	const [, setaddPath] = useAtom(addPath);
	const [, setAddValue] = useAtom(addValue);
	const [, setFormValue] = useAtom(formValue);
	const [, setDashboardValue] = useAtom(dashboardValue);

	useMountEffect(() => {
		setHeaderText("Add social History");
		setaddImage(Icon);
		setaddPath(`/socialhistory/add/${patientId}`);
		setAddValue(false);
		setFormValue(false);
		setDashboardValue(false);
	});
	const [searchQuery, setSearchQuery] = useState("");

	const { patientId } = useParams({ from: "patientId" });
	const { isOpen: isDeleteModalOpen, onOpen, onClose } = useDisclosure();
	const [deletedata, setDeletedata] = useState("");

	const deletesocialHistory = DeleteSocialHistoryFn();
	const handleDelete = async (id: string) => {
		try {
			await db.socialhistory.where({ id }).delete();
			await deletesocialHistory.mutateAsync({
				id,
				patient_id: patientId,
				last_updated_input: await db.getLastUpdated(),
			});
		} catch (error) {
			console.error("Error deleting medication:", error);
		}
		onClose();
	};

	const socialhistoryData = useLiveQuery(() => db.socialhistory.toArray());
	const socialhistoryDataById = socialhistoryData?.filter(
		(item) => item.patient_id === patientId,
	);

	const filteredData = (socialhistoryDataById || []).filter((item) =>
		item.created_by
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
									placeholder={"social History Details"}
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
											<SocialHistoryCard
												birth_gender={items.birth_gender}
												tobacco={items.tobacco}
												alcohol={items.alcohol}
												drug_abuse={items.drug_abuse}
												cardiovascular={items.cardiovascular}
												safety={items.safety}
												sexual_activity={items.sexual_activity}
												comments={items.comments}
												editpath={`/socialhistory/edit/${items.patient_id}/${items.id}`}
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

export default socialHistoryList;
