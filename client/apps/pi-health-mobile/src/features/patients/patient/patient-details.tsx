import {
	Box,
	Button,
	Center,
	Fade,
	Heading,
	useDisclosure,
} from "@chakra-ui/react";
import VitalIcon from "../../../assets/Vital Icon 24 x 24.svg";
import NoteIcon from "../../../assets/Note Icon.svg";
import Allergies from "../../../assets/Allergies icon 24 x 24.svg";
import Problems from "../../../assets/Problems Icon 24 x 24.svg";
import Immunizations from "../../../assets/Immunizations Icon 24 x 24.svg";
import Medication from "../../../assets/Medication Icon 24 x 24.svg";
import history from "../../../assets/history.svg";
import PatientDetailImmunizationsCard from "../../../components/patient-details-immunizations-card";
import PatientPopup from "./patient-popup";
import { useAtom } from "jotai";
import { useMountEffect } from "@react-hookz/web";
import {
	headerText,
	addValue,
	formValue,
	dashboardValue,
	displayMenu,
} from "../../../atoms/header";
import { useParams } from "@tanstack/react-router";
import { motion, AnimatePresence } from "framer-motion";
const MotionBox = motion(Box);

const PatientDetails = () => {
	const [, setHeaderText] = useAtom(headerText);
	const [, setAddValue] = useAtom(addValue);
	const [, setFormValue] = useAtom(formValue);
	const [, setmenu] = useAtom(displayMenu);
	const [, setDashboardValue] = useAtom(dashboardValue);

	const patientId = useParams({
		from: "/patient/details/$patientId",
		select: (params) => params.patientId,
	});
	useMountEffect(() => {
		setHeaderText("Patient Details");
		setAddValue(true);
		setFormValue(true);
		setDashboardValue(false);
		setmenu(false);
	});

	const {
		isOpen: isModalOpen,
		onOpen: openModal,
		onClose: closeModal,
	} = useDisclosure();
	const patientDetailsFormData = [
		{
			headings: "Notes",
			icon: NoteIcon,
			path: "/notes/list",
			id: 1,
		},
		{
			headings: "Vitals",
			icon: VitalIcon,
			path: "/vitals/list",
			id: 1,
		},
		{
			headings: "Allergies",
			path: "/allergy/list",
			icon: Allergies,
			id: 3,
		},
		{
			headings: "Problems",
			icon: Problems,
			path: "/problem/list",
			id: 3,
		},
		{
			headings: "Medications",
			icon: Medication,
			path: "/medication/list",
			id: 4,
		},
		{
			headings: "History",
			icon: history,
			path: "/history",
			id: 5,
		},
		{
			headings: "Immunizations",
			icon: Immunizations,
			path: "/immunization",
			id: 6,
		},
	];
	return (
		<div>
			<Fade in={true}>
				<Box top={0} height={0} minHeight={"100vh"} bgColor={"#F7F7F9"}>
					<Center>
						<Box width={{ md: "80%", base: "90%", lg: "70%" }}>
							{patientDetailsFormData.map((items) => (
								<div key={items.id}>
									<Center>
										<AnimatePresence>
											<MotionBox
												initial={{ opacity: 0, x: -50 }}
												animate={{ opacity: 1, x: 0 }}
												exit={{ opacity: 0, x: -50 }}
												transition={{ duration: 0.65 }}
												width={"100%"}
											>
												<PatientDetailImmunizationsCard
													heading={items.headings}
													paths={
														items.headings === "History"
															? "/history"
															: items.headings === "Immunizations"
																? "/immunization"
																: `${items.path}/${patientId}`
													}
													img_src={items.icon}
												/>
											</MotionBox>
										</AnimatePresence>
									</Center>
								</div>
							))}
							<Box
								bgColor={"#FFFFFF"}
								style={{
									height: "85px",
									border: "15px solid #F7F7F9",
								}}
								borderRadius={"100px"}
							>
								<Box
									border="2px solid #095FBA"
									height="100%"
									width="100%"
									borderRadius={"10px"}
								>
									<Button
										onClick={openModal}
										width="100%"
										height="100%"
										bgColor={"#FFFFFF"}
										color={"#095FBA"}
										borderRadius={"10px"}
									>
										<Heading as="h4" size="md" color="#095FBA" ml="2">
											Patient Details
										</Heading>
									</Button>
								</Box>
							</Box>
						</Box>
						<PatientPopup
							open={isModalOpen}
							close={closeModal}
							patientId={patientId}
						/>
					</Center>
				</Box>
			</Fade>
		</div>
	);
};
export default PatientDetails;
