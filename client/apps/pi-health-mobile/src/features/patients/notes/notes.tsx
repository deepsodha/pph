import type React from "react";
import { useState } from "react";
import { Box, Fade, Select } from "@chakra-ui/react";
import { useMountEffect } from "@react-hookz/web";
import { useAtom } from "jotai";
import {
	headerText,
	addValue,
	formValue,
	dashboardValue,
} from "../../../atoms/header";
import Soap from "./soap/soap";
import HistoryAndPhysical from "./history-and-physical/history-and-physical";
import { useRouterState } from "@tanstack/react-router";

const Notes = () => {
	const [, setHeaderText] = useAtom(headerText);
	const [, setAddValue] = useAtom(addValue);
	const [, setFormValue] = useAtom(formValue);
	const [, setDashboardValue] = useAtom(dashboardValue);

	const [selectedNoteType, setSelectedNoteType] = useState("historicalForm"); // Set default value to "historicalForm"
	const router = useRouterState();
	const role = router.location.pathname.split("/")[2];
	useMountEffect(() => {
		setHeaderText(role === "edit" ? "Edit Notes" : "Add Notes");
		setAddValue(true);
		setFormValue(true);
		setDashboardValue(false);
	});

	const handleNoteTypeChange = (
		event: React.ChangeEvent<HTMLSelectElement>,
	) => {
		setSelectedNoteType(event.target.value);
	};

	return (
		<div>
			<Fade in={true}>
				<Box minHeight={"100vh"} bgColor={"#F7F7F9"} overflowY="auto">
					<Box m="4%">
						<Select
							bgColor="#FFFFFF"
							borderColor="#095FBA"
							placeholder="Select Note Type"
							value={selectedNoteType} // Set the value attribute to maintain the selected value
							onChange={handleNoteTypeChange}
						>
							<option value="historicalForm">History and Physical</option>
							<option value="soap">Soap</option>
							<option value="acupunctureFollowUp">Acupuncture Follow Up</option>
						</Select>

						{selectedNoteType === "historicalForm" && <HistoryAndPhysical />}
						{selectedNoteType === "soap" && <Soap />}
					</Box>
				</Box>
			</Fade>
		</div>
	);
};

export default Notes;
