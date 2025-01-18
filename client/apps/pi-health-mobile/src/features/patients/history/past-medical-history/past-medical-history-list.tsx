import { Box, Center, Fade, Text, useDisclosure } from "@chakra-ui/react";
import Icon from "../../../../assets/Add Service Location Icon.svg";
import SearchBar from "../../../../components/search-bar";
import { useAtom } from "jotai";
import {
	headerText,
	addImage,
	addPath,
	addValue,
	dashboardValue,
	formValue,
} from "../../../../atoms/header";
import { useMountEffect } from "@react-hookz/web";
import { useParams } from "@tanstack/react-router";
import { useLiveQuery } from "dexie-react-hooks";
import { db } from "../../../../db/db";
import { useState } from "react";
import PastMedicalHistoryCard from "./past-medical-history-card";
import { DeletePastMedicalHistoryDataFn } from "../../../../query-mutation-services/past-medical-history";
import DeleteModal from "../../../../components/delete-modal";

const ServiceLocationDetailsList = () => {
	const [, setHeaderText] = useAtom(headerText);
	const [, setaddImage] = useAtom(addImage);
	const [, setaddPath] = useAtom(addPath);
	const [, setAddValue] = useAtom(addValue);
	const [, setFormValue] = useAtom(formValue);
	const [, setDashboardValue] = useAtom(dashboardValue);

	const [searchQuery, setSearchQuery] = useState("");
	const { patientId } = useParams({ from: "patientId" });
	const { isOpen: isDeleteModalOpen, onOpen, onClose } = useDisclosure();
	const [deletedata, setDeletedata] = useState("");

	const pastmedicalhistoryData = useLiveQuery(() =>
		db.pastmedicalhistory.toArray(),
	);
	useMountEffect(() => {
		setHeaderText("Add Past Medical History");
		setaddImage(Icon);
		setaddPath(`/pastmedicalhistory/add/${patientId}`);
		setAddValue(false);
		setFormValue(false);
		setDashboardValue(false);
	});

	const deletPastMedicalHistory = DeletePastMedicalHistoryDataFn();
	const handleDelete = async (id: string) => {
		try {
			await db.pastmedicalhistory.where({ id }).delete();

			await deletPastMedicalHistory.mutateAsync({
				id,
				patient_id: patientId,
				last_updated_input: await db.getLastUpdated(),
			});
		} catch (error) {
			console.error("Error deleting past Medical History:", error);
		}
		onClose();
	};
	const filteredData = (pastmedicalhistoryData || []).filter((item) =>
		item.comments?.toString().toLowerCase().includes(searchQuery.toLowerCase()),
	);
	const sortedData = filteredData.slice().sort((a, b) => {
		const dateA = new Date(a.created_at);
		const dateB = new Date(b.created_at);
		return dateB.getTime() - dateA.getTime();
	});

	return (
		<>
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
								<Text
									mb={"2%"}
									color={"#095FBA"}
									mt={"5%"}
									fontSize={"20px"}
									fontWeight={"600"}
								>
									Past Medical History List
								</Text>
								<SearchBar
									value={searchQuery}
									onchange={(e) => setSearchQuery(e.target.value)}
									placeholder={"Search by Comments"}
								/>
							</Box>
						</Center>
					</Box>

					<Box zIndex={15} mt={5} bgColor={"#F7F7F9"} pb="10%">
						{sortedData?.map((item) => (
							<div key={item.created_at}>
								<Center>
									<Box width={"100%"}>
										<PastMedicalHistoryCard
											blood_type={item?.blood_type}
											head={item?.head}
											respiratory={item?.respiratory}
											musculoskeletal={item.musculoskeletal}
											endocrine={item.endocrine}
											eyes={item.eyes}
											gastrointestinal={item.gastrointestinal}
											skin={item.skin}
											ears={item.ears}
											noses={item.noses}
											neurological={item.neurological}
											heme={item.heme}
											mouth={item.mouth}
											infectious={item.infectious}
											cardiovascular={item.cardiovascular}
											genitourinary={item.genitourinary}
											psychiatric={item.psychiatric}
											comments={item.comments}
											editpath={`/pastmedicalhistory/edit/${item.patient_id}/${item.id}`}
											handleDelete={() => {
												setDeletedata(item.id);
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
				</Box>
			</Fade>
		</>
	);
};

export default ServiceLocationDetailsList;
