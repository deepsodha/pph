import { useForm } from "@felte/react";
import { SubmitButton } from "../component/submit-button";
import { validator } from "@felte/validator-zod";
import reporterDom from "@felte/reporter-dom";
import type z from "zod";
import {
	FormLabel,
	FormControl,
	Select,
	FormErrorMessage,
	Textarea,
} from "@chakra-ui/react";

import type { LastUpdatedInput, MedicationsAdd, Status } from "@repo/types/dto";
import { medicationsAddSchema } from "@repo/types/validation";
import type {
	MedicationsStateExtend,
	PatientStateExtend,
} from "@repo/types/dexie-state";
import type { drugData } from "./prescription-form";
export type AddMedicationFormProps = {
	onSubmit: (p: MedicationsAdd) => void;
	lastUpdatedInput: () => Promise<LastUpdatedInput>;
	patient: PatientStateExtend[] | undefined;
	initialValues?: MedicationsStateExtend | undefined;
	edit?: boolean;
	drug?: string[];
	drug_name?: drugData[] | undefined;
	patientId: string;
};

export const MedicationForm = (props: AddMedicationFormProps) => {
	if (props.initialValues === undefined && props.edit === true) {
		return <p>Loading...</p>;
	}

	const patientId = props.patientId;

	const { form, errors, isSubmitting, data } = useForm<
		z.infer<typeof medicationsAddSchema>
	>({
		onSubmit: async (values) => {
			{
				const modifiedValues = {
					...values,
					last_updated_input: await props.lastUpdatedInput(),
					patient_id: patientId,
					// patient_name: patient_name,
				};

				props.onSubmit(modifiedValues);
			}
		},
		initialValues: {
			patient_id: patientId,
			drug: props.initialValues?.drug || "",
			instruction: props.initialValues?.instruction || "",
			status: (props.initialValues?.status as Status) || "",
			comments: props.initialValues?.comments || "",
		},
		extend: [validator({ schema: medicationsAddSchema }), reporterDom()],
	});

	return (
		<div>
			<form ref={form}>
				<FormLabel mb={2} my={3} color={"#095FBA"}>
					Status
					<span style={{ color: "red" }}>*</span>
				</FormLabel>
				<FormControl mt={2} isInvalid={(errors().status || []).length !== 0}>
					<Select
						placeholder="Select Status"
						name="status"
						bgColor="#FFFFFF"
						borderColor="#095FBA"
					>
						<option value="Active">Active</option>
						<option value="InActive">InActive</option>
					</Select>
					{errors().status && (
						<FormErrorMessage>{errors().status}</FormErrorMessage>
					)}
				</FormControl>

				<FormLabel mb={2} my={3} color={"#095FBA"}>
					Drug
					<span style={{ color: "red" }}>*</span>
				</FormLabel>
				<FormControl mt={2} isInvalid={(errors().drug || []).length !== 0}>
					<Select
						placeholder="Drugs"
						name="drug"
						bgColor="#FFFFFF"
						borderColor="#095FBA"
						value={data().drug || ""}
					>
						{props.drug_name?.map((items) => (
							<option
								key={items.id}
								// value={`${items.id}:${items.code.first_name}`}
							>
								{items.Brand}
							</option>
						))}
					</Select>
					{errors().drug && (
						<FormErrorMessage>{errors().drug}</FormErrorMessage>
					)}
				</FormControl>

				<FormLabel mb={2} my={3} color={"#095FBA"}>
					Instruction
					<span style={{ color: "red" }}>*</span>
				</FormLabel>
				<FormControl
					mt={2}
					isInvalid={(errors().instruction || []).length !== 0}
				>
					<Textarea
						bgColor="#FFFFFF"
						name="instruction"
						borderColor="#095FBA"
						placeholder="Instructions"
					/>
					{errors().instruction && (
						<FormErrorMessage>{errors().instruction}</FormErrorMessage>
					)}
				</FormControl>

				<FormLabel mb={2} my={3} color={"#095FBA"}>
					Comments
				</FormLabel>
				<FormControl mt={2} isInvalid={(errors().comments || []).length !== 0}>
					<Textarea
						bgColor="#FFFFFF"
						name="comments"
						borderColor="#095FBA"
						placeholder="Comments"
						maxLength={100}
					/>
					{errors().comments && (
						<FormErrorMessage>{errors().comments}</FormErrorMessage>
					)}
				</FormControl>
				<SubmitButton loading={isSubmitting()} />
			</form>
		</div>
	);
};
