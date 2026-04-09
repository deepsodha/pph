import { FormLabel, Textarea, Box, Center } from "@chakra-ui/react";
import AccordianComp, { type AccordianData } from "../component/accordian-comp";
import { useForm } from "@felte/react";

import type {
	VitalsStateExtend,
	AllergiesStateExtend,
	MedicationsStateExtend,
} from "@repo/types/dexie-state";
import type { HistoryAndPhysicalAdd, LastUpdatedInput } from "@repo/types/dto";
import type { historyAndPhysicalAddSchema } from "@repo/types/validation";
import type { z } from "zod";
import { SubmitButton } from "../component";

import NotesAddIcon from "../component/notes-add-icon";
import type { CurrentNoteState } from "../../types/dto/CurrentNoteState";
export type AddHistoryAndPhysicalFormProps = {
	onSubmit: (p: HistoryAndPhysicalAdd) => void;
	lastUpdatedInput: () => Promise<LastUpdatedInput>;
	patientId: string;
	addicon?: string;
	vitals: VitalsStateExtend[] | undefined;
	allergy: AllergiesStateExtend[] | undefined;
	medication: MedicationsStateExtend[] | undefined;
	// initialValues: HistoryAndPhysicalNoteStateExtend | undefined;
	// edit?: boolean;
	// HandPId?: string;
	Link: (path: string) => void;
	//patient: PatientStateExtend[] | undefined;
};

const HistoryAndPhysicalForm = ({
	lastUpdatedInput,
	onSubmit,
	vitals,
	medication,
	allergy,
	patientId,
	// initialValues,
	// edit,
	Link,
}: AddHistoryAndPhysicalFormProps) => {
	const isPropsReady =
		(vitals && medication && allergy) ||
		(vitals && medication && allergy === undefined);

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
	//   const medicationSortedData =
	//     medication?.slice().sort((a, b) => {
	//       const dateA = new Date(a.last_updated);
	//       const dateB = new Date(b.last_updated);
	//       return dateB.getTime() - dateA.getTime();
	//     }) ?? [];
	//  // const firstmedication = medicationSortedData[0];
	//   const allergySortedData =
	//     allergy?.slice().sort((a, b) => {
	//       const dateA = new Date(a.last_updated);
	//       const dateB = new Date(b.last_updated);
	//       return dateB.getTime() - dateA.getTime();
	//     }) ?? [];

	//const firstallergy = allergySortedData[0];
	//   console.log(firstmedication?.id, "medicationId");
	const { form, isSubmitting } = useForm<
		z.infer<typeof historyAndPhysicalAddSchema>
	>({
		onSubmit: async (values) => {
			const modifiedValues = {
				...values,
				patient_id: patientId,
				note_state: "Open" as CurrentNoteState,
				medications_id: medication.map((med) => med.id),
				allergies_id: allergy ? allergy.map((allergy) => allergy.id) : [],
				vitals_id: firstVital ? firstVital.id : "g234876543",
				last_updated_input: await lastUpdatedInput(),
			};
			onSubmit(modifiedValues);
		},

		// extend: [validator({ schema: historyAndPhysicalAddSchema }), reporterDom()],
	});

	return (
		<div>
			<form ref={form}>
				<Box
					display={"flex"}
					flexDirection={"column"}
					justifyContent={"center"}
					alignItems={"left"}
				>
					<FormLabel mb={2} my={3} color={"#095FBA"}>
						CC
					</FormLabel>
					<Textarea
						borderColor={"#095FBA"}
						placeholder="CC"
						bgColor={"#FFFFFF"}
						name="chief_complaint"
						mr={4}
					/>

					<FormLabel mb={2} my={3} color={"#095FBA"}>
						HPI
					</FormLabel>
					<Textarea
						borderColor={"#095FBA"}
						placeholder="HPI"
						bgColor={"#FFFFFF"}
						name="history_of_present_illness"
						mr={4}
					/>
					<FormLabel mb={2} my={3} color={"#095FBA"}>
						PMHx
					</FormLabel>
					<Textarea
						borderColor={"#095FBA"}
						placeholder="PMHx"
						bgColor={"#FFFFFF"}
						name="past_medical_history"
						mr={4}
					/>

					<FormLabel mb={2} my={3} color={"#095FBA"}>
						PSHx
					</FormLabel>
					<Textarea
						borderColor={"#095FBA"}
						placeholder="PSHx"
						bgColor={"#FFFFFF"}
						name="past_surgical_history"
						mr={4}
					/>

					<FormLabel mb={2} my={3} color={"#095FBA"}>
						FHx
					</FormLabel>
					<Textarea
						borderColor={"#095FBA"}
						placeholder="FHx"
						bgColor={"#FFFFFF"}
						name="family_history"
						mr={4}
					/>

					<FormLabel mb={2} my={3} color={"#095FBA"}>
						Soc Hx
					</FormLabel>
					<Textarea
						borderColor={"#095FBA"}
						placeholder="Soc Hx"
						bgColor={"#FFFFFF"}
						name="social_history"
						mr={4}
					/>

					<FormLabel mb={2} my={3} color={"#095FBA"}>
						Ob Preg Hx
					</FormLabel>
					<Textarea
						borderColor={"#095FBA"}
						placeholder="Ob Preg Hx"
						bgColor={"#FFFFFF"}
						name="obstetric_and_pregnancy_history"
						mr={4}
					/>

					<FormLabel mb={2} my={3} color={"#095FBA"}>
						Hospitalizations
					</FormLabel>
					<Textarea
						borderColor={"#095FBA"}
						placeholder="Hospitalizations"
						bgColor={"#FFFFFF"}
						name="hospitalizations"
						mr={4}
					/>

					<FormLabel mb={2} my={3} color={"#095FBA"}>
						Implantable Devices
					</FormLabel>
					<Textarea
						borderColor={"#095FBA"}
						placeholder="Implantable Devices"
						bgColor={"#FFFFFF"}
						name="implantable_devices"
						mr={4}
					/>

					<FormLabel mb={2} my={3} color={"#095FBA"}>
						ROS
					</FormLabel>
					<Textarea
						borderColor={"#095FBA"}
						placeholder="ROS"
						bgColor={"#FFFFFF"}
						name="review_of_systems"
						mr={4}
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
							<Box onClick={() => Link(`/medication/add/${patientId}`)}>
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
									{
										title: "Medication Status",
										content: med.status.toString(),
									},
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
							<Box onClick={() => Link(`/allergy/add/${patientId}`)} mr="12px">
								<NotesAddIcon />
							</Box>
						</Box>
					</FormLabel>
					<AccordianComp
						data={
							allergy?.map((allergy, index) => ({
								id: index + 1,
								title: `Allergy ${index + 1} Details`,
								content: [
									{
										title: "Allergy Status",
										content: allergy.stauts.toString(),
									},
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

					<FormLabel mb={2} my={3} color={"#095FBA"}>
						Mental/Functional
					</FormLabel>
					<Textarea
						borderColor={"#095FBA"}
						placeholder="Mental/Functional"
						bgColor={"#FFFFFF"}
						name="mental_or_functional"
						mr={4}
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
							<Box onClick={() => Link(`/vitals/add/${patientId}`)} mr="12px">
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
							{
								id: 3,
								title: "Height",
								content: firstVital?.height?.toString(),
							},
							{
								id: 4,
								title: "Weight",
								content: firstVital?.weight?.toString(),
							},
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
						EXAM
					</FormLabel>
					<Textarea
						borderColor={"#095FBA"}
						placeholder="EXAM"
						bgColor={"#FFFFFF"}
						name="exam"
						mr={4}
					/>

					<FormLabel mb={2} my={3} color={"#095FBA"}>
						Assessment
					</FormLabel>
					<Textarea
						borderColor={"#095FBA"}
						placeholder="Assessment"
						bgColor={"#FFFFFF"}
						name="assessment"
						mr={4}
					/>

					<FormLabel mb={2} my={3} color={"#095FBA"}>
						Plan
					</FormLabel>
					<Textarea
						borderColor={"#095FBA"}
						placeholder="Plan"
						bgColor={"#FFFFFF"}
						name="plan"
						mr={4}
					/>

					<FormLabel mb={2} my={3} color={"#095FBA"}>
						Minor Procedures
					</FormLabel>
					<Textarea
						borderColor={"#095FBA"}
						placeholder="Minor Procedures"
						bgColor={"#FFFFFF"}
						name="minor_procedures"
						mr={4}
					/>

					<FormLabel mb={2} my={3} color={"#095FBA"}>
						Goals
					</FormLabel>
					<Textarea
						borderColor={"#095FBA"}
						placeholder="Goals"
						bgColor={"#FFFFFF"}
						name="goals"
						mr={4}
					/>

					<FormLabel mb={2} my={3} color={"#095FBA"}>
						Health Concerns
					</FormLabel>
					<Textarea
						borderColor={"#095FBA"}
						placeholder="Health Concerns"
						bgColor={"#FFFFFF"}
						name="health_concerns"
						mr={4}
					/>
					<Box>
						{/* <Button type="submit">submit</Button> */}
						<SubmitButton loading={isSubmitting()} />
					</Box>
				</Box>
			</form>
		</div>
	);
};
export default HistoryAndPhysicalForm;
