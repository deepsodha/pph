import { Box, useDisclosure, Heading, Center, Fade } from "@chakra-ui/react";
import Historical from "../../../../assets/Add Historical Icon 36 x 36.svg";
import DashboardCard from "../../../../components/dashboard-card";
import { useAtom } from "jotai";
import SearchBar from "../../../../components/search-bar";
import HistoricalPopup from "./historical-popup";
import { useState } from "react";
import { DeleteHistoricalDataFn } from "../../../../query-mutation-services/historical-data-fn";
import { useLiveQuery } from "dexie-react-hooks";
import {
	headerText,
	addImage,
	addPath,
	addValue,
	dashboardValue,
	formValue,
} from "../../../../atoms/header";
import { useParams } from "@tanstack/react-router";
import DeleteModal from "../../../../components/delete-modal";
import { db } from "../../../../db/db";
import { useMountEffect } from "@react-hookz/web";
const HistoricalList = () => {
	const { isOpen: isDeleteModalOpen, onOpen, onClose } = useDisclosure();

	const [searchQuery, setSearchQuery] = useState("");
	const [deletedata, setDeletedata] = useState("");
	const [, setHeaderText] = useAtom(headerText);
	const [, setaddImage] = useAtom(addImage);
	const [, setaddPath] = useAtom(addPath);
	const [, setAddValue] = useAtom(addValue);
	const [, setFormValue] = useAtom(formValue);
	const [, setDashboardValue] = useAtom(dashboardValue);
	useMountEffect(() => {
		setHeaderText("Add Historical");
		setaddImage(Historical);
		setaddPath(`/historical/add/${patientId}`);
		setAddValue(false);
		setFormValue(false);
		setDashboardValue(false);
		setDashboardValue(false);
	});
	const { patientId } = useParams({ from: "patientId" });
	const {
		isOpen: isModalOpen,
		onOpen: openModalBase,
		onClose: closeModal,
	} = useDisclosure();
	const [activeId, setActiveId] = useState<string>("");
	const openModal = (id: string) => {
		setActiveId(id);
		openModalBase();
	};
	const deleteHistorical = DeleteHistoricalDataFn();
	const historicalData = useLiveQuery(() => db.historical.toArray());
	const handleDelete = async (id: string) => {
		try {
			await db.historical.where({ id }).delete();

			await deleteHistorical.mutateAsync({
				id,
				patient_id: patientId,
				last_updated_input: await db.getLastUpdated(),
			});
		} catch (error) {
			console.error("Error deleting historical:", error);
		}
		onClose();
	};
	const historicalDataById = historicalData?.filter(
		(item) => item.patient_id === patientId,
	);
	const filteredData = (historicalDataById || []).filter((item) =>
		item.vaccine.toString().toLowerCase().includes(searchQuery.toLowerCase()),
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
						zIndex={10}
						bgColor={"#F7F7F9"}
						top={0}
						left={0}
						right={0}
					>
						<Center>
							<Box width={{ md: "80%", base: "90%", lg: "70%" }}>
								<Heading
									color="#095FBA"
									pt="1"
									fontSize="20px"
									mt="16px"
									mb="10x"
								>
									Historical List
								</Heading>
								<SearchBar
									value={searchQuery}
									onchange={(e) => setSearchQuery(e.target.value)}
									placeholder={"Search by Vaccine"}
								/>
							</Box>
						</Center>
					</Box>
					<Box zIndex={15} mt={5} bgColor={"#F7F7F9"}>
						{sortedData?.map((item) => (
							<div key={item.created_at}>
								<Center>
									<Box
										key={item.id}
										width={{ md: "80%", base: "90%", lg: "70%" }}
									>
										<DashboardCard
											heading_1={"Vaccine:"}
											result_1={item.vaccine}
											heading_2={"Type:"}
											result_2={item.types}
											open_model={() => openModal(item.id)}
											editpath={`/historical/edit/${patientId}/${item.id}`}
											handleDelete={() => {
												setDeletedata(item.id);
												onOpen();
											}}
										/>
									</Box>
								</Center>
								<HistoricalPopup
									open={isModalOpen}
									close={closeModal}
									id={activeId}
									patientId={patientId}
								/>
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
export default HistoricalList;
