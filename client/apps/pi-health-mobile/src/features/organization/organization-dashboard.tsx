import { Box, Center, Fade, SimpleGrid } from "@chakra-ui/react";
import Patient from "../../assets/Patient Icon.svg";
import Doctor from "../../assets/Doctor Icon.svg";
import Appointment from "../../assets/Appointment Icon.svg";
import Prescription from "../../assets/Prescription Icon.svg";
import Setting from "../../assets/Settings Icon.svg";
import DashboardIconCard from "../../components/dashboard-icon-card";
import SearchBar from "../../components/search-bar";

import { useAtom } from "jotai";
import { useState } from "react";
import {
	headerText,
	addValue,
	formValue,
	dashboardValue,
} from "../../atoms/header";
import { useMountEffect } from "@react-hookz/web";
import LogoutButton from "../../components/logout-button";

const OrganizationDashboardList = () => {
	const [searchQuery, setSearchQuery] = useState("");
	const [, setHeaderText] = useAtom(headerText);
	const [, setAddValue] = useAtom(addValue);
	const [, setFormValue] = useAtom(formValue);
	const [, setDashboardValue] = useAtom(dashboardValue);

	useMountEffect(() => {
		setHeaderText("Pi Health");
		setAddValue(true);
		setFormValue(false);
		setDashboardValue(true);
	});
	const data = [
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
			title: "settings",
			heading: "Settings",
			icon: Setting,
		},
	];

	const filteredData = (data || []).filter((item) =>
		item.title.toString().toLowerCase().includes(searchQuery.toLowerCase()),
	);

	return (
		<div>
			<Fade in={true} style={{ transitionDuration: "200ms" }}>
				<Box minHeight={"100vh"} bgColor={"#F7F7F9"}>
					<Box bgColor={"#F7F7F9"} minHeight="100vh" mt={3}>
						<Center>
							<Box width={{ md: "80%", base: "90%", lg: "70%" }}>
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
										<SimpleGrid
											columns={3}
											width="100%"
											mt={5}
											spacing={{ lg: "8", base: "5" }}
										>
											{filteredData?.map((item) => (
												<div key={item.title}>
													<DashboardIconCard
														title={item.title}
														icon={item.icon}
														heading={item.heading}
													/>
												</div>
											))}
										</SimpleGrid>
									</Center>
								</Box>
							</Box>
						</Center>
						<Box width={"100%"}>
							<Center>
								<LogoutButton />
							</Center>
						</Box>
					</Box>
				</Box>
			</Fade>
		</div>
	);
};

export default OrganizationDashboardList;
