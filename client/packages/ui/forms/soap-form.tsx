import {
	Box,
	Center,
	FormErrorMessage,
	FormLabel,
	Textarea,
} from "@chakra-ui/react";
import { SubmitButton } from "../component";
import { useForm } from "@felte/react";
import type { z } from "zod";
import type { soapAddSchema } from "@repo/types/validation";
import type { LastUpdatedInput, SoapAdd } from "@repo/types/dto";
import type {
	AllergiesStateExtend,
	MedicationsStateExtend,
	VitalsStateExtend,
} from "@repo/types/dexie-state";

import AccordianComp, { type AccordianData } from "../component/accordian-comp";
import type { CurrentNoteState } from "../../types/dto/CurrentNoteState";
import NotesAddIcon from "../component/notes-add-icon";
export type AddSoapFormProps = {
	onSubmit: (p: SoapAdd) => void;
	lastUpdatedInput: () => Promise<LastUpdatedInput>;
	addicon?: string;
	patient?: string | undefined;
	vitals: VitalsStateExtend[] | undefined;
	allergy: AllergiesStateExtend[] | undefined;
	medication: MedicationsStateExtend[] | undefined;
	Link: (path: string) => void;
};
const SoapForm = ({
	lastUpdatedInput,
	onSubmit,
	vitals,
	medication,
	allergy,
	patient,
	Link,
}: AddSoapFormProps) => {
	const isPropsReady = vitals && medication && allergy && patient;
	if (!isPropsReady) {
		return <div>Loading...</div>;
	}

	const vitalsSortedData =
		vitals?.slice().sort((a, b) => {
			const dateA = new Date(a.last_updated);
			const dateB = new Date(b.last_updated);
			return dateB.getTime() - dateA.getTime();
		}) ?? [];
	const firstVital = vitalsSortedData[0];
	const { form, errors, isSubmitting } = useForm<z.infer<typeof soapAddSchema>>(
		{
			onSubmit: async (values) => {
				const modifiedValues = {
					...values,
					patient_id: patient,
					note_state: "Open" as CurrentNoteState,
					medications_id: medication.map((med) => med.id),
					allergies_id: allergy.map((allergy) => allergy.id),
					vitals_id: firstVital ? firstVital.id : "g234876543",
					last_updated_input: await lastUpdatedInput(),
				};
				onSubmit(modifiedValues);
			},
			//   extend: [validator({ schema: soapAddSchema }), reporterDom()],
		},
	);
	return (
		<div>
			<form ref={form}>
				<FormLabel mb={2} my={3} color={"#095FBA"}>
					{"Chief Complaint"}
				</FormLabel>
				<Textarea
					name="chief_complaint"
					_focus={{
						borderColor: "#095FBA",
					}}
					focusBorderColor="#095FBA"
					borderColor="#095FBA"
					placeholder={"Chief Complaint"}
				/>
				<FormLabel mb={2} my={3} color={"#095FBA"}>
					{"Subjective"}
				</FormLabel>
				<Textarea
					name="subjective"
					_focus={{
						borderColor: "#095FBA",
					}}
					focusBorderColor="#095FBA"
					borderColor="#095FBA"
					placeholder={"Subjective"}
				/>
				<FormLabel
					mb={2}
					my={3}
					color={"#095FBA"}
					display={"flex"}
					justifyContent={"space-between"}
					flexDirection={"row"}
				>
					<Center>Medications</Center>

					<Box
						display={"flex"}
						flexDirection={"row"}
						//justifyContent={"space-between"}
						width={"70px"}
					>
						<Box onClick={() => Link(`/medication/add/${patient}`)}>
							<NotesAddIcon />
						</Box>
					</Box>
				</FormLabel>
				<AccordianComp
					data={
						medication.map((med, index) => ({
							id: index + 1,
							title: `Medication ${index + 1} Details`,
							content: [
								{ title: "Medication Status", content: med.status.toString() },
								{ title: "Drugs", content: med.drug },
								{ title: "Instruction", content: med.instruction },
								{ title: "Comments", content: med.comments },
							],
						})) as unknown as AccordianData[]
					}
					title={"Medications"}
				/>
				<FormLabel
					mb={2}
					my={3}
					color={"#095FBA"}
					display={"flex"}
					justifyContent={"space-between"}
					flexDirection={"row"}
				>
					<Center>Allergies</Center>

					<Box
						display={"flex"}
						flexDirection={"row"}
						justifyContent={"space-around"}
						width={"70px"}
					>
						<Box onClick={() => Link(`/allergy/add/${patient}`)} mr="12px">
							<NotesAddIcon />
						</Box>
					</Box>
				</FormLabel>
				<AccordianComp
					data={
						allergy.map((allergy, index) => ({
							id: index + 1,
							title: `Allergy ${index + 1} Details`,
							content: [
								{ title: "Allergy Status", content: allergy.stauts.toString() },
								{ title: "Allergy Name", content: allergy.allergen },
								{ title: "Reactions", content: allergy.reaction },
								{
									title: "VFC Financial Class",
									content: allergy.allergy_severities,
								},
								{ title: "Comments", content: allergy.comments },
							],
						})) as unknown as AccordianData[]
					}
					title={"Allergies"}
				/>
				<FormLabel
					mb={2}
					my={3}
					color={"#095FBA"}
					display={"flex"}
					justifyContent={"space-between"}
					flexDirection={"row"}
				>
					<Center>Vitals</Center>

					<Box
						display={"flex"}
						flexDirection={"row"}
						justifyContent={"space-around"}
						width={"70px"}
					>
						<Box onClick={() => Link(`/vitals/add/${patient}`)} mr="12px">
							<NotesAddIcon />
						</Box>
					</Box>
				</FormLabel>
				<AccordianComp
					data={[
						{
							id: 1,
							title: "Heart Rate",
							content: firstVital?.heart_rate?.toString(),
						},
						{ id: 2, title: "Comments", content: firstVital?.comments },
						{ id: 3, title: "Height", content: firstVital?.height?.toString() },
						{ id: 4, title: "Weight", content: firstVital?.weight?.toString() },
						{ id: 5, title: "BMI", content: firstVital?.bmi?.toString() },
						{
							id: 6,
							title: "Temperature",
							content: firstVital?.temperature?.toString(),
						},
					]}
					title={"Vitals"}
				/>
				<FormLabel mb={2} my={3} color={"#095FBA"}>
					{"Mental Function"}
				</FormLabel>
				<Textarea
					name="mental_or_functional"
					bgColor="#FFFFFF"
					borderColor="#095FBA"
					_focus={{
						borderColor: "#095FBA",
					}}
					focusBorderColor="#095FBA"
					placeholder="Mental Function"
				/>
				<FormLabel mb={2} my={3} color={"#095FBA"}>
					{"Objective"}
				</FormLabel>
				<Textarea
					name="objective"
					bgColor="#FFFFFF"
					borderColor="#095FBA"
					_focus={{
						borderColor: "#095FBA",
					}}
					focusBorderColor="#095FBA"
					placeholder="Objective"
				/>
				{errors().objective && (
					<FormErrorMessage>{errors().objective}</FormErrorMessage>
				)}
				<FormLabel mb={2} my={3} color={"#095FBA"}>
					{"Assessment"}
				</FormLabel>
				<Textarea
					name="assessment"
					bgColor="#FFFFFF"
					borderColor="#095FBA"
					_focus={{
						borderColor: "#095FBA",
					}}
					focusBorderColor="#095FBA"
					placeholder="Assessment"
				/>
				<FormLabel mb={2} my={3} color={"#095FBA"}>
					{"Plan"}
				</FormLabel>
				<Textarea
					name="plan"
					bgColor="#FFFFFF"
					borderColor="#095FBA"
					_focus={{
						borderColor: "#095FBA",
					}}
					focusBorderColor="#095FBA"
					placeholder="Plan"
				/>
				<Box>
					<SubmitButton loading={isSubmitting()} />
				</Box>
			</form>
		</div>
	);
};
export default SoapForm;
