import { Box, Center, Fade, Flex, Select } from "@chakra-ui/react";
import AddAppointment from "../../assets/Add Appointment Icon.svg";
import SearchBar from "../../components/search-bar";
import AppointmentTabs from "./appointment-tabs";
import { useState } from "react";
import { useMountEffect } from "@react-hookz/web";
import {
	headerText,
	addImage,
	addPath,
	addValue,
	dashboardValue,
	formValue,
	servicelocationid,
	displayMenu,
} from "../../atoms/header";

import { useAtom } from "jotai";
import { useLiveQuery } from "dexie-react-hooks";
import { db } from "../../db/db";
import CalenderBar from "../../components/calender-bar";
const AppointmentList = () => {
	const [, setHeaderText] = useAtom(headerText);
	const [, setaddImage] = useAtom(addImage);
	const [, setaddPath] = useAtom(addPath);
	const [, setAddValue] = useAtom(addValue);
	const [, setFormValue] = useAtom(formValue);
	const [, setmenu] = useAtom(displayMenu);
	const [servicelocationId, setServiceLocationId] = useAtom(servicelocationid);
	const [, setDashboardValue] = useAtom(dashboardValue);

	useMountEffect(() => {
		setHeaderText("Add Appointment");
		setaddImage(AddAppointment);
		setaddPath("/appointment/add");
		setAddValue(true);
		setFormValue(false);
		setDashboardValue(false);
		setmenu(true);
	});

	const [searchQuery, setSearchQuery] = useState("");
	const [selectedDate, setSelectedDate] = useState<Date | null>(null);

	const handleDateSelect = (date: Date) => {
		setSelectedDate(date);
	};

	const handleServiceLocationChange = (
		e: React.ChangeEvent<HTMLSelectElement>,
	) => {
		console.log("id", e.target.value.split(":")[0]);
		const selectedLocationId = e.target.value.split(":")[0];
		setServiceLocationId(selectedLocationId);
	};

	const servicelocation = useLiveQuery(() => db.servicelocation.toArray());
	const appointmentData = useLiveQuery(() => db.appointments.toArray());

	const filteredByServiceLocation = (appointmentData || []).filter(
		(item) => item.service_location_id === servicelocationId,
	);

	const filteredData = filteredByServiceLocation.filter((item) =>
		item.patient_name
			.toString()
			.toLowerCase()
			.includes(searchQuery.toLowerCase()),
	);

	let data = filteredData || [];

	if (selectedDate) {
		data = data.filter((item) => {
			const itemDate = new Date(item.date);
			return itemDate.toDateString() === selectedDate.toDateString();
		});
	}

	return (
		<Fade in={true}>
			<Flex direction="column" minHeight={"100vh"} bgColor={"#F7F7F9"}>
				<Box position="relative" top={0} zIndex={10} bgColor={"#F7F7F9"}>
					<Center>
						<Box width={{ md: "80%", base: "90%", lg: "70%" }}>
							<CalenderBar
								heading={"Appointment List"}
								onSelectDate={handleDateSelect}
							/>
							<SearchBar
								value={searchQuery}
								onchange={(e) => setSearchQuery(e.target.value)}
								placeholder={"Search Patient by Name"}
							/>
							<Select
								mt="3%"
								border="2px"
								borderColor={"#095FBA"}
								placeholder="Select service location"
								onChange={handleServiceLocationChange}
							>
								{servicelocation?.map((items) => (
									<option
										key={items.service_location_name}
										value={`${items.id}:${items.service_location_name}`}
										selected={servicelocationId === items.id}
									>
										{items.service_location_name}
									</option>
								))}
							</Select>
						</Box>
					</Center>
				</Box>
				<Box flex="1">
					<AppointmentTabs filterData={data} />
				</Box>
			</Flex>
		</Fade>
	);
};

export default AppointmentList;
