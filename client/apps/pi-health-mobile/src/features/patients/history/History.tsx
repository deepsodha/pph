import { Fade, Center, Box } from "@chakra-ui/react";
import { useMountEffect } from "@react-hookz/web";
import { useParams } from "@tanstack/react-router";
import { useAtom } from "jotai";
import {
	headerText,
	addValue,
	formValue,
	dashboardValue,
} from "../../../atoms/header";
import PatientDetailHistoryCard from "../../../components/patient-details-history-card";

const history = () => {
	const [, setHeaderText] = useAtom(headerText);
	const [, setAddValue] = useAtom(addValue);
	const [, setFormValue] = useAtom(formValue);
	const [, setDashboardValue] = useAtom(dashboardValue);
	const { patientId } = useParams({ from: "patientId" });

	useMountEffect(() => {
		setHeaderText("History");
		setAddValue(true);
		setFormValue(true);
		setDashboardValue(false);
	});

	const HistoryData = [
		{
			paths: "/familyhistory/list",
			headings: "Family History",
			id: 1,
		},
		{
			paths: "/pastmedicalhistory/list",
			headings: "Past Medical History",
			id: 2,
		},
		{
			paths: "/pregnancyhistory/list",
			headings: "Ob & Pregnancy History",
			id: 3,
		},
		{
			paths: "/implantabledevice/list",
			headings: "Implantable Devices",
			id: 4,
		},
		{
			paths: "/hospitalization/list/",
			headings: "Hospitalizations & Procedures",
			id: 5,
		},
		{
			paths: "/socialhistory/list",
			headings: "Social History",
			id: 6,
		},
		{
			paths: "/pastsurgicalhistory/list",
			headings: "Past Surgical History",
			id: 7,
		},
	];
	return (
		<Box bgColor="#F7F7F9">
			<Fade in={true}>
				<Center>
					<Box width={{ md: "80%", base: "90%", lg: "70%" }}>
						{HistoryData.map((items) => (
							<Box cursor={"pointer"} key={items.id}>
								<PatientDetailHistoryCard
									heading={items.headings}
									paths={`/${items.paths}/${patientId}`}
								/>
							</Box>
						))}
					</Box>
				</Center>
			</Fade>
		</Box>
	);
};

export default history;
