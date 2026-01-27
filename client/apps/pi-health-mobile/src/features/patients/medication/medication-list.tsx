import { Box, Heading, Flex, Center, Progress, Fade } from "@chakra-ui/react";
import Problem from "../../../assets/Problem Icon 36 x 36.svg";
import { useAtom } from "jotai";
import { useState } from "react";
import { useMountEffect } from "@react-hookz/web";
import {
	headerText,
	addImage,
	addPath,
	addValue,
	dashboardValue,
	formValue,
} from "../../../atoms/header";
import { useParams } from "@tanstack/react-router";
import { useLiveQuery } from "dexie-react-hooks";
import { db } from "../../../db/db";
import MedicationListTabs from "./medication-list-tabs";
import SearchBar from "../../../components/search-bar";
import CalenderBar from "../../../components/calender-bar";
const MedicationList = () => {
	const [, setHeaderText] = useAtom(headerText);
	const [, setaddImage] = useAtom(addImage);
	const [, setaddPath] = useAtom(addPath);
	const [, setAddValue] = useAtom(addValue);
	const [, setFormValue] = useAtom(formValue);
	const [, setDashboardValue] = useAtom(dashboardValue);
	const patientId = useParams({
		from: "/medication/list/$patientId",
		select: (params) => params.patientId,
	});
	const [selectedDate, setSelectedDate] = useState<Date | null>(new Date());
	const [isLoading] = useState<boolean>(false);
	useMountEffect(() => {
		setHeaderText("Add Medication");
		setaddImage(Problem);
		setaddPath(`/medication/add/${patientId}`);
		setAddValue(false);
		setFormValue(false);
		setDashboardValue(false);
	});
	const [searchQuery, setSearchQuery] = useState("");
	const medicationData = useLiveQuery(() => db.medications.toArray());
	const data = medicationData?.filter((item) => item.patient_id === patientId);
	const filteredData = data || [];
	const headerData = useLiveQuery(() => db.patients.toArray());
	const headerFilterData = headerData?.find((item) => item.id === patientId);
	const handleDateSelect = (date: Date) => {
		setSelectedDate(date);
	};
	return (
		<Fade in={true}>
			<Flex direction="column" minHeight={"100vh"} bgColor={"#F7F7F9"}>
				<Box position="sticky" top={0} zIndex={10} bgColor={"#F7F7F9"}>
					<Progress size="xs" isIndeterminate={isLoading} />
					<Center>
						<Box width={{ md: "80%", base: "90%", lg: "70%" }}>
							<Heading
								color="#095FBA"
								pt="1"
								fontSize="20px"
								mt="16px"
								mb="10px"
							>
								<span style={{ color: "black" }}>
									{`${headerFilterData?.user.first_name}'s `}
								</span>
								Medication List
							</Heading>
							<CalenderBar
								heading={"Select Date "}
								onSelectDate={handleDateSelect}
							/>
							<SearchBar
								value={searchQuery}
								onchange={(e) => setSearchQuery(e.target.value)}
								placeholder={"Search by Drug Name"}
							/>
						</Box>
					</Center>
				</Box>
				<Box flex="1" position={"sticky"}>
					<MedicationListTabs
						filterData={filteredData}
						selectedDate={selectedDate}
					/>
				</Box>
			</Flex>
		</Fade>
	);
};
export default MedicationList;
