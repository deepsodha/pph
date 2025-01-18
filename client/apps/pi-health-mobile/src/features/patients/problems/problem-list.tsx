import { Box, Heading, Flex, Center, Progress, Fade } from "@chakra-ui/react";
import Problem from "../../../assets/Problem Icon 36 x 36.svg";
import { useAtom } from "jotai";
import SearchBar from "../../../components/search-bar";
import ProblemListTabs from "./problem-list-tabs";
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

const ProblemList = () => {
	const [, setHeaderText] = useAtom(headerText);
	const [, setaddImage] = useAtom(addImage);
	const [, setaddPath] = useAtom(addPath);
	const [, setAddValue] = useAtom(addValue);
	const [, setFormValue] = useAtom(formValue);
	const [, setDashboardValue] = useAtom(dashboardValue);

	const patientId = useParams({
		from: "/problem/list/$patientId",
		select: (params) => params.patientId,
	});

	useMountEffect(() => {
		setHeaderText("Add Problem");
		setaddImage(Problem);
		setaddPath(`/problem/add/${patientId}`);
		setAddValue(false);
		setFormValue(false);
		setDashboardValue(false);
	});
	const [isLoading] = useState<boolean>();

	const [searchQuery, setSearchQuery] = useState("");
	const problemData = useLiveQuery(() => db.problems.toArray());

	const data = problemData?.filter((item) => item.patient_id === patientId);
	const filteredData = (data || []).filter((item) =>
		item.issue.toString().toLowerCase().includes(searchQuery.toLowerCase()),
	);
	const headerData = useLiveQuery(() => db.patients.toArray());
	const headerFilterData = headerData?.find((item) => item.id === patientId);

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
								Problem List
							</Heading>
							<SearchBar
								value={searchQuery}
								onchange={(e) => setSearchQuery(e.target.value)}
								placeholder={"Search by Problem Name"}
							/>
						</Box>
					</Center>
				</Box>
				<Box position={"sticky"}>
					<ProblemListTabs filterData={filteredData} />
				</Box>
			</Flex>
		</Fade>
	);
};

export default ProblemList;
