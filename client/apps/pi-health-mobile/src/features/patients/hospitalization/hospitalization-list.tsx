import { Box, Center, Fade, Heading, useDisclosure } from "@chakra-ui/react";
import SearchBar from "../../../components/search-bar";
//import {hospitalizationProcedure} from "../../../assets/hospitalizationProcedures.svg";
import hospital from "../../../assets/hospitalizationProcedures.svg";
import HospitalizationCard from "../../../components/dashboard-card-hospitalization";
import { useMountEffect } from "@react-hookz/web";
import { useAtom } from "jotai";
import {
	headerText,
	addImage,
	addPath,
	addValue,
	formValue,
	dashboardValue,
} from "../../../atoms/header";
import { useParams } from "@tanstack/react-router";
import { useLiveQuery } from "dexie-react-hooks";
import { db } from "../../../db/db";
import { useState } from "react";
import DeleteModal from "../../../components/delete-modal";
import { DeleteHospitalizationDataFn } from "../../../query-mutation-services/hospitalization-data-fn";

const hospitalizationDashboard = () => {
	const [, setHeaderText] = useAtom(headerText);
	const [, setaddImage] = useAtom(addImage);
	const [, setaddPath] = useAtom(addPath);
	const [, setAddValue] = useAtom(addValue);
	const [, setFormValue] = useAtom(formValue);
	const [, setDashboardValue] = useAtom(dashboardValue);
	const { isOpen: isDeleteModalOpen, onOpen, onClose } = useDisclosure();
	const [searchQuery, setSearchQuery] = useState("");
	const { patientId } = useParams({ from: "patientId" });
	const [deletedata, setDeletedata] = useState("");
	useMountEffect(() => {
		setHeaderText("Hospitalizations & Procedures");
		setaddImage(hospital);
		setaddPath(`/hospitalization/add/${patientId}`);
		setAddValue(false);
		setFormValue(false);
		setDashboardValue(false);
	});
	const deleteHospitalization = DeleteHospitalizationDataFn();
	const handleDelete = async (id: string) => {
		try {
			await db.hospitalization.where({ id }).delete();

			await deleteHospitalization.mutateAsync({
				id,
				patient_id: patientId,
				last_updated_input: await db.getLastUpdated(),
			});
		} catch (error) {
			console.error("Error deleting vital:", error);
		}
		onClose();
	};
	const hospitlizationData = useLiveQuery(() => db.hospitalization.toArray());
	const hospitalizationDataById = hospitlizationData?.filter(
		(item) => item.patient_id === patientId,
	);
	const filteredData = (hospitalizationDataById || []).filter((item) =>
		item.admission_date?.toString().includes(searchQuery),
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
									Hospitalization List
								</Heading>
								<SearchBar
									value={searchQuery}
									onchange={(e) => setSearchQuery(e.target.value)}
									placeholder={"Search by Admission Date"}
								/>
							</Box>
						</Center>
					</Box>
					<Box zIndex={15} mt={5} bgColor={"#F7F7F9"}>
						{sortedData.map((item) => (
							<div key={item.created_at}>
								<Center>
									<Box
										cursor={"pointer"}
										width={{ md: "80%", base: "90%", lg: "70%" }}
									>
										<HospitalizationCard
											result_1={item.admission_date}
											result_2={item.related_to}
											result_3={item.status}
											result_4={item.length_of_stay}
											result_5={item.procedure}
											result_6={item.comments}
											handleDelete={() => {
												setDeletedata(item.id);
												onOpen();
											}}
											editpath={`/hospitalization/edit/${item.patient_id}/${item.id}`}
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
		</div>
	);
};

export default hospitalizationDashboard;
