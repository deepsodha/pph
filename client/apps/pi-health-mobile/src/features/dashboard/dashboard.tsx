import { Box, Center, SimpleGrid } from "@chakra-ui/react";
import Patient from "../../assets/Patient Icon.svg";
import Doctor from "../../assets/Doctor Icon.svg";
import Appointment from "../../assets/Appointment Icon.svg";
import Prescription from "../../assets/Prescription Icon.svg";
import DashboardIconCard from "../../components/dashboard-icon-card";
import SearchBar from "../../components/search-bar";
import Staff from "../../assets/Add_staff_icon.svg";
import Settings from "../../assets/Settings Icon.svg";
import User from "../../assets/userLogo.svg";
import Admin from "../../assets/System Admin Icon.svg";
import { useAtom } from "jotai";
import { useState } from "react";
import {
	addValue,
	formValue,
	dashboardValue,
	addPath,
	profileValue,
	displayMenu,
} from "../../atoms/header";
import { useMountEffect } from "@react-hookz/web";
import {
	isdoctor,
	isofficestaff,
	issuperadmin,
	issystemadmin,
} from "../../atoms/roles";
import { motion, AnimatePresence } from "framer-motion";

const MotionBox = motion(Box);
const Dashboard = () => {
	const [superadmin] = useAtom(issuperadmin);
	const [systemadmin] = useAtom(issystemadmin);
	const [doctor] = useAtom(isdoctor);
	const [officestaff] = useAtom(isofficestaff);
	const [, setAddValue] = useAtom(addValue);
	const [, setFormValue] = useAtom(formValue);
	const [, setDashboardValue] = useAtom(dashboardValue);
	const [, setmenu] = useAtom(displayMenu);
	const [, setPath] = useAtom(addPath);
	const [, setProfile] = useAtom(profileValue);
	let data: { title: string; heading: string; icon: string }[] = [];
	if (systemadmin && !doctor && !officestaff && !superadmin) {
		data = [
			{
				title: "doctor/list",
				heading: "Doctor",
				icon: Doctor,
			},
			{
				title: "staff/list",
				heading: "Staff",
				icon: Staff,
			},
			{
				title: "settings/list",
				heading: "Settings",
				icon: Settings,
			},
			{
				title: "user/list",
				heading: "Users",
				icon: User,
			},
		];
	} else if (superadmin && !doctor && !officestaff && !systemadmin) {
		data = [
			{
				title: "systemadmin/list",
				heading: "System Admin",
				icon: Admin,
			},
		];
	} else if (doctor && !superadmin && !officestaff && !systemadmin) {
		data = [
			{
				title: "doctor/list",
				heading: "Doctor",
				icon: Doctor,
			},
			{
				title: "patient/list",
				heading: "Patient",
				icon: Patient,
			},
			{
				title: "appointment/list",
				heading: "Appointment",
				icon: Appointment,
			},
			{
				title: "prescription/list",
				heading: "Prescription",
				icon: Prescription,
			},
			{
				title: "staff/list",
				heading: "Staff",
				icon: Staff,
			},
			{
				title: "settings/list",
				heading: "Settings",
				icon: Settings,
			},
		];
	} else if (officestaff && !doctor && !systemadmin && !superadmin) {
		data = [
			{
				title: "patient/list",
				heading: "Patient",
				icon: Patient,
			},
			{
				title: "appointment/list",
				heading: "Appointment",
				icon: Appointment,
			},
			{
				title: "prescription/list",
				heading: "Prescription",
				icon: Prescription,
			},
			{
				title: "staff/list",
				heading: "Staff",
				icon: Staff,
			},
		];
	} else if (systemadmin && doctor && !officestaff && !superadmin) {
		data = [
			{
				title: "doctor/list",
				heading: "Doctor",
				icon: Doctor,
			},
			{
				title: "patient/list",
				heading: "Patient",
				icon: Patient,
			},
			{
				title: "appointment/list",
				heading: "Appointment",
				icon: Appointment,
			},
			{
				title: "prescription/list",
				heading: "Prescription",
				icon: Prescription,
			},
			{
				title: "staff/list",
				heading: "Staff",
				icon: Staff,
			},
			{
				title: "settings/list",
				heading: "Settings",
				icon: Settings,
			},
			{
				title: "user/list",
				heading: "Users",
				icon: User,
			},
		];
	} else if (systemadmin && officestaff && !doctor && !superadmin) {
		data = [
			{
				title: "doctor/list",
				heading: "Doctor",
				icon: Doctor,
			},
			{
				title: "patient/list",
				heading: "Patient",
				icon: Patient,
			},
			{
				title: "appointment/list",
				heading: "Appointment",
				icon: Appointment,
			},
			{
				title: "prescription/list",
				heading: "Prescription",
				icon: Prescription,
			},
			{
				title: "staff/list",
				heading: "Staff",
				icon: Staff,
			},
			{
				title: "settings/list",
				heading: "Settings",
				icon: Settings,
			},
			{
				title: "user/list",
				heading: "Users",
				icon: User,
			},
		];
	} else if (doctor && officestaff && !systemadmin && !superadmin) {
		data = [
			{
				title: "doctor/list",
				heading: "Doctor",
				icon: Doctor,
			},
			{
				title: "patient/list",
				heading: "Patient",
				icon: Patient,
			},
			{
				title: "appointment/list",
				heading: "Appointment",
				icon: Appointment,
			},
			{
				title: "prescription/list",
				heading: "Prescription",
				icon: Prescription,
			},
			{
				title: "staff/list",
				heading: "Staff",
				icon: Staff,
			},
			{
				title: "settings/list",
				heading: "Settings",
				icon: Settings,
			},
		];
	} else if (systemadmin && doctor && officestaff && !superadmin) {
		data = [
			{
				title: "doctor/list",
				heading: "Doctor",
				icon: Doctor,
			},
			{
				title: "patient/list",
				heading: "Patient",
				icon: Patient,
			},
			{
				title: "appointment/list",
				heading: "Appointment",
				icon: Appointment,
			},
			{
				title: "prescription/list",
				heading: "Prescription",
				icon: Prescription,
			},
			{
				title: "staff/list",
				heading: "Staff",
				icon: Staff,
			},
			{
				title: "settings/list",
				heading: "Settings",
				icon: Settings,
			},
			{
				title: "user/list",
				heading: "Users",
				icon: User,
			},
		];
	}
	const [searchQuery, setSearchQuery] = useState("");
	const filteredData = (data || []).filter((item) =>
		item.title.toString().toLowerCase().includes(searchQuery.toLowerCase()),
	);

	useMountEffect(() => {
		setAddValue(true);
		setFormValue(false);
		setDashboardValue(true);
		setProfile(true);
		setPath("/profile/doctor");
		setmenu(false);
	});

	return (
		<Box minHeight={"100vh"} bgColor={"#F7F7F9"}>
			<Box bgColor={"#F7F7F9"} minHeight="100vh" mt={3}>
				<Center>
					<Box width={{ md: "80%", base: "90%", lg: "70%" }} zIndex={10}>
						<SearchBar
							value={searchQuery}
							onchange={(e) => setSearchQuery(e.target.value)}
							placeholder={"Search by Category"}
						/>
					</Box>
				</Center>
				<Center>
					<Box display="flex" width={{ md: "80%", base: "90%", lg: "70%" }}>
						<Box
							width={"100%"}
							display="flex-row"
							justifyItems={"center"}
							justifyContent={"space-between"}
						>
							<Center>
								<AnimatePresence>
									<SimpleGrid
										columns={3}
										width="100%"
										mt={5}
										spacing={{ lg: "8", base: "5" }}
									>
										{filteredData?.map((item) => (
											<MotionBox
												key={item.title}
												initial={{ opacity: 0, y: 50 }}
												animate={{ opacity: 1, y: 0 }}
												exit={{ opacity: 0, y: 50 }}
												transition={{ duration: 0.65 }}
											>
												<DashboardIconCard
													title={item.title}
													icon={item.icon}
													heading={item.heading}
												/>
											</MotionBox>
										))}
									</SimpleGrid>
								</AnimatePresence>
							</Center>
						</Box>
					</Box>
				</Center>
			</Box>
		</Box>
	);
};

export default Dashboard;
