import { Box, Center, Fade, Flex } from "@chakra-ui/react";
import { Heading } from "@chakra-ui/react";
import Allergy from "../../../assets/AllergyIcon.svg";
import SearchBar from "../../../components/search-bar";
import { useState } from "react";
import { useAtom } from "jotai";
import {
	headerText,
	addImage,
	addPath,
	addValue,
	dashboardValue,
	formValue,
} from "../../../atoms/header";
import { useParams } from "@tanstack/react-router";
import { db } from "../../../db/db";
import { useLiveQuery } from "dexie-react-hooks";
import { useMountEffect } from "@react-hookz/web";
import AllergyListTabs from "./allergy-list-tabs";

const AllergyList = () => {
	const [, setHeaderText] = useAtom(headerText);
	const [, setaddImage] = useAtom(addImage);
	const [, setaddPath] = useAtom(addPath);
	const [, setAddValue] = useAtom(addValue);
	const [, setFormValue] = useAtom(formValue);
	const [, setDashboardValue] = useAtom(dashboardValue);

	const [searchQuery, setSearchQuery] = useState("");
	useMountEffect(() => {
		setHeaderText("Add Allergy");
		setaddImage(Allergy);
		setaddPath(`/allergy/add/${patientId}`);
		setAddValue(false);
		setFormValue(false);
		setDashboardValue(false);
	});
	const patientId = useParams({
		from: "/allergy/list/$patientId",
		select: (params) => params.patientId,
	});

	const allergyData = useLiveQuery(() => db.allergy.toArray());
	const allergyDataById = allergyData?.filter(
		(item) => item.patient_id === patientId,
	);
	const filteredData = (allergyDataById || []).filter((item) =>
		item.allergen.toString().toLowerCase().includes(searchQuery.toLowerCase()),
	);
	const headerData = useLiveQuery(() => db.patients.toArray());
	const headerFilterData = headerData?.find((item) => item.id === patientId);

	return (
		<Fade in={true}>
			<Flex direction="column" minHeight={"100vh"} bgColor={"#F7F7F9"}>
				<Box position="sticky" top={0} zIndex={10} bgColor={"#F7F7F9"}>
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
								Allergy List
							</Heading>
							<SearchBar
								value={searchQuery}
								onchange={(e) => setSearchQuery(e.target.value)}
								placeholder={"Search by Allergen Name"}
							/>
						</Box>
					</Center>
				</Box>
				<Box position={"sticky"}>
					<AllergyListTabs filterData={filteredData} />
				</Box>
			</Flex>
		</Fade>
	);
};
export default AllergyList;
