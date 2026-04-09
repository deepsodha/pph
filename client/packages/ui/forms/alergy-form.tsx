import {
	FormControl,
	FormErrorMessage,
	FormLabel,
	Input,
	Radio,
	RadioGroup,
	Stack,
	Select,
	Textarea,
} from "@chakra-ui/react";

import { SubmitButton } from "../component/submit-button";
import { useForm } from "@felte/react";
import type { z } from "zod";
import reporterDom from "@felte/reporter-dom";
import { validator } from "@felte/validator-zod";
import { SaveAddAnotherButton } from "../component/save-add-another-button";

import type {
	AllergyAdd,
	AllergySeveritiesType,
	LastUpdatedInput,
	Status,
} from "@repo/types/dto";
import { allergyAddSchema } from "@repo/types/validation";
import type { AllergiesStateExtend } from "@repo/types/dexie-state";
import { dobDate } from "../component/utc-date-to-normal-date";
import DateComponent from "../component/date-comp";

export type AddAllergyFormProps = {
	onSubmit: (p: AllergyAdd) => void;
	patientId?: string;
	allergen?: string[];
	allergyId?: string;
	edit?: boolean;
	lastUpdatedInput: () => Promise<LastUpdatedInput>;
	initialValues?: AllergiesStateExtend | undefined;
};

export const AlergyForm = (props: AddAllergyFormProps) => {
	if (props.initialValues === undefined && props.edit === true) {
		return <p>Loading...</p>;
	}
	const today = new Date();
	const current = today.toISOString().slice(0, 16);

	const { form, errors, isSubmitting } = useForm<
		z.infer<typeof allergyAddSchema>
	>({
		onSubmit: async (values) => {
			const utcDate = dobDate(values.input_date);
			const modifiedValues = {
				...values,
				last_updated_input: await props.lastUpdatedInput(),
			};

			modifiedValues.input_date = utcDate;
			props.onSubmit(modifiedValues);
		},
		initialValues: {
			patient_id: props.patientId,
			allergen: props.initialValues?.allergen || "",
			reaction: props.initialValues?.reaction || "",
			allergy_severities:
				(props.initialValues?.allergy_severities as AllergySeveritiesType) ||
				"",
			status: props.initialValues?.stauts as Status,
			comments: props.initialValues?.comments || "",
			input_date:
				(props.initialValues?.input_date &&
					dobDate(props.initialValues.input_date)) ||
				"",
		},
		extend: [validator({ schema: allergyAddSchema }), reporterDom()],
	});

	return (
		<form ref={form}>
			<FormLabel mb={2} my={3} color={"#095FBA"}>
				{"Status"}
				<span style={{ color: "red" }}>*</span>
			</FormLabel>
			<FormControl mt={2} isInvalid={(errors().status || []).length !== 0}>
				<RadioGroup
					defaultValue={props.initialValues?.stauts}
					name="status"
					mt={"4"}
					color={"#095FBA"}
				>
					<Stack direction="row">
						<Radio value="Active">Active</Radio>
						<Radio value="InActive">InActive</Radio>
					</Stack>
				</RadioGroup>
				{errors().status && (
					<FormErrorMessage>{errors().status}</FormErrorMessage>
				)}
			</FormControl>
			<FormLabel mb={2} my={3} color={"#095FBA"}>
				Allergen Name
				<span style={{ color: "red" }}>*</span>
			</FormLabel>
			<FormControl mt={2} isInvalid={(errors().allergen || []).length !== 0}>
				<Select
					placeholder="Allergen"
					name="allergen"
					bgColor="#FFFFFF"
					borderColor="#095FBA"
				>
					{props.allergen?.map((allergen) => (
						<option key={allergen} value={allergen}>
							{allergen}
						</option>
					))}
				</Select>

				{errors().allergen && (
					<FormErrorMessage>{errors().allergen}</FormErrorMessage>
				)}
			</FormControl>
			<FormLabel mb={2} my={3} color={"#095FBA"}>
				Reactions
				<span style={{ color: "red" }}>*</span>
			</FormLabel>
			<FormControl mt={2} isInvalid={(errors().reaction || []).length !== 0}>
				<Input
					type="text"
					name="reaction"
					bgColor={"#FFFFFF"}
					borderColor={"#095FBA"}
					placeholder="Reactions"
				/>
				{errors().reaction && (
					<FormErrorMessage>{errors().reaction}</FormErrorMessage>
				)}
			</FormControl>

			<FormLabel mb={2} my={3} color={"#095FBA"}>
				Date of Onset (Starting Date)
				<span style={{ color: "red" }}>*</span>
			</FormLabel>
			<FormControl mt={2} isInvalid={(errors().input_date || []).length !== 0}>
				<DateComponent
					type={"date"}
					name={"input_date"}
					placeholder={"Date of Onset (Starting Date)"}
					max={current}
				/>
				{errors().input_date && (
					<FormErrorMessage>{errors().input_date}</FormErrorMessage>
				)}
			</FormControl>
			<FormLabel mb={2} my={3} color={"#095FBA"}>
				VFC Financial Class
				<span style={{ color: "red" }}>*</span>
			</FormLabel>
			<FormControl
				mt={2}
				isInvalid={(errors().allergy_severities || []).length !== 0}
			>
				<Select
					placeholder="Select VFC Financial Class"
					name="allergy_severities"
					bgColor="#FFFFFF"
					borderColor="#095FBA"
				>
					<option value="Mild">Mild</option>
					<option value="Moderate">Moderate</option>
					<option value="Severe">Severe</option>
				</Select>
				{errors().allergy_severities && (
					<FormErrorMessage>{errors().allergy_severities}</FormErrorMessage>
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
			<SaveAddAnotherButton path="/" />
			<SubmitButton loading={isSubmitting()} />
		</form>
	);
};
