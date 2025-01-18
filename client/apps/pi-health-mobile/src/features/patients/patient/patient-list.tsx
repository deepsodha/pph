import { Box, Center, Fade, Heading, useDisclosure } from "@chakra-ui/react";
import AddPatient from "../../../assets/Add Patient Icon.svg";
import DashboardCardWithImage from "../../../components/dashboard-card-with-image";
import SearchBar from "../../../components/search-bar";
import { deletePatientFn } from "../../../query-mutation-services/patient-data-fn";
import { useLiveQuery } from "dexie-react-hooks";
import { useState } from "react";
import { useMountEffect } from "@react-hookz/web";
import { useAtom } from "jotai";
import {
	headerText,
	addImage,
	addPath,
	addValue,
	dashboardValue,
	formValue,
	displayMenu,
} from "../../../atoms/header";
import DeleteModal from "../../../components/delete-modal";
import { db } from "../../../db/db";
import { GetVaccineDataFn } from "../../../query-mutation-services/organization-data-fn";
import { VaccineData } from "../../../atoms/vaccine-data";
import { motion, AnimatePresence } from "framer-motion";
const MotionBox = motion(Box);
const PatientList = () => {
	const [, setHeaderText] = useAtom(headerText);
	const [, setaddImage] = useAtom(addImage);
	const [, setaddPath] = useAtom(addPath);
	const [, setAddValue] = useAtom(addValue);
	const [, setFormValue] = useAtom(formValue);
	const [, setDashboardValue] = useAtom(dashboardValue);
	const [, setmenu] = useAtom(displayMenu);

	const [searchQuery, setSearchQuery] = useState("");
	const [deletedata, setDeletedata] = useState("");
	const { isOpen: isDeleteModalOpen, onOpen, onClose } = useDisclosure();
	const { data } = GetVaccineDataFn();
	const { UpdateVaccineData } = VaccineData();

	UpdateVaccineData(data?.result);
	useMountEffect(() => {
		setHeaderText("Add Patient");
		setaddImage(AddPatient);
		setaddPath("/patient/add");
		setAddValue(true);
		setFormValue(false);
		setDashboardValue(false);
		setmenu(true);
	});

	const deletePatient = deletePatientFn();
	const patientData = useLiveQuery(() => db.patients.toArray());

	const filteredData = ((patientData && patientData) || [])
		.reverse()
		.filter((item) => item?.phone?.number.toString().includes(searchQuery));

	const handleDelete = async (id: string) => {
		try {
			await db.patients.where({ id }).delete();

			await deletePatient.mutateAsync({
				id,
				last_updated_input: await db.getLastUpdated(),
			});
		} catch (error) {
			console.error("Error deleting complaint:", error);
		}
		onClose();
	};
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
						zIndex={10}
						bgColor={"#F7F7F9"}
						top={0}
						left={0}
						right={0}
					>
						{/* <Progress size="xs" isIndeterminate={patients.isLoading} /> */}
						<Center>
							<Box width={{ md: "80%", base: "90%", lg: "70%" }}>
								<Heading
									color="#095FBA"
									pt="1"
									fontSize="20px"
									mt="16px"
									mb="10px"
								>
									Patient List
								</Heading>
								<SearchBar
									value={searchQuery}
									onchange={(e) => setSearchQuery(e.target.value)}
									placeholder={"Search Patient by Mobile Number"}
								/>
							</Box>
						</Center>{" "}
					</Box>
					<Box zIndex={15} mt={5} bgColor={"#F7F7F9"} pb="10%">
						{sortedData?.map((item) => (
							<div key={item.created_at}>
								<Center>
									<AnimatePresence>
										<MotionBox
											key={item.created_at}
											initial={{ opacity: 0, y: -50 }}
											animate={{ opacity: 1, y: 0 }}
											exit={{ opacity: 0, y: -50 }}
											transition={{ duration: 0.65 }}
											width={{ md: "80%", base: "90%", lg: "70%" }}
										>
											<DashboardCardWithImage
												profile_img={item.user.photo_url}
												name={item.user.first_name}
												cardPath={`/patient/details/${item.id}`}
												result={item.phone.number}
												heading={"Mobile No:"}
												editpath={`/patient/edit/${item.id}`}
												handleDelete={() => {
													setDeletedata(item.id);
													onOpen();
												}}
												gender={item.user.gender}
											/>
										</MotionBox>
									</AnimatePresence>
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

export default PatientList;
