import { SubmitButton } from "../component/submit-button";
import { SaveAddAnotherButton } from "../component/save-add-another-button";
import { useForm } from "@felte/react";
import { validator } from "@felte/validator-zod";
import reporterDom from "@felte/reporter-dom";
import type { z } from "zod";

import {
	FormControl,
	FormErrorMessage,
	FormLabel,
	Input,
	Radio,
	RadioGroup,
	Select,
	Stack,
	Textarea,
} from "@chakra-ui/react";
import { convertToUTC } from "../component/convert-to-utc";
import { problemsAddSchema } from "@repo/types/validation";
import type {
	LastUpdatedInput,
	ProblemTypes,
	ProblemsAdd,
	Status,
} from "@repo/types/dto";
import type { ProblemStateExtend } from "@repo/types/dexie-state";
import { convertUTCtoLocal } from "../component/utc-date-to-normal-date";
import DateComponent from "../component/date-comp";
export type icd10data = {
	category: string;
	code: string;
	description: string;
	icd_code: string;
	id: number;
	long_description: string;
};

export type AddProblemProps = {
	patientId?: string;
	icd_data: icd10data[] | undefined;
	edit?: boolean;
	lastUpdatedInput: () => Promise<LastUpdatedInput>;
	initialValues?: ProblemStateExtend | undefined;
	onSubmit: (p: ProblemsAdd) => void;
};
export const ProblemForm = (props: AddProblemProps) => {
	if (props.initialValues === undefined && props.edit === true) {
		return <p>Loading...</p>;
	}
	const today = new Date();
	today.setDate(today.getDate());
	const yesterday = today.toISOString().slice(0, 16);
	const { form, errors, isSubmitting, data } = useForm<
		z.infer<typeof problemsAddSchema>
	>({
		onSubmit: async (values) => {
			const changedDates = {
				utcDate1: convertToUTC(values.start_date),
				utcDate2: convertToUTC(values.end_date),
			};

			const modifiedValues = {
				...values,
				last_updated_input: await props.lastUpdatedInput(),
			};
			modifiedValues.start_date = changedDates.utcDate1;
			modifiedValues.end_date = changedDates.utcDate2;
			props.onSubmit(modifiedValues);
		},
		initialValues: {
			patient_id: props.patientId,
			status: props.initialValues?.status as Status,
			issue: props.initialValues?.issue,
			icd_10_problem: props.initialValues?.icd_10_problem || undefined,
			issue_type: props.initialValues?.issue_type as ProblemTypes,
			start_date:
				props.initialValues?.start_date &&
				convertUTCtoLocal(props.initialValues.start_date),
			end_date:
				props.initialValues?.end_date &&
				convertUTCtoLocal(props.initialValues.end_date),
			comment: props.initialValues?.comment || "",
		},
		extend: [validator({ schema: problemsAddSchema }), reporterDom()],
	});

	return (
		<form ref={form}>
			<FormLabel mb={2} my={3} color={"#095FBA"}>
				{"Status"}
				<span style={{ color: "red" }}>*</span>
			</FormLabel>
			<FormControl mt={2} isInvalid={(errors().status || []).length !== 0}>
				<RadioGroup
					defaultValue={props.initialValues?.status}
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
				{"Problem/Issue Name"}
				<span style={{ color: "red" }}>*</span>
			</FormLabel>
			<FormControl mt={2} isInvalid={(errors().issue || []).length !== 0}>
				<Input
					type="text"
					name="issue"
					bgColor="#FFFFFF"
					borderColor="#095FBA"
					placeholder={"Problem/Issue Name"}
				/>
				{errors().issue && (
					<FormErrorMessage>{errors().issue}</FormErrorMessage>
				)}
			</FormControl>
			<FormLabel mb={2} my={3} color={"#095FBA"}>
				{"ICD-10"}
				<span style={{ color: "red" }}>*</span>
			</FormLabel>
			<FormControl
				mt={2}
				isInvalid={(errors().icd_10_problem || []).length !== 0}
			>
				<Select
					name="icd_10_problem"
					bgColor="#FFFFFF"
					borderColor="#095FBA"
					placeholder={"ICD-10"}
					value={data().icd_10_problem || ""}
				>
					{props.icd_data?.map((items) => (
						<option
							key={items.id}
							// value={`${items.id}:${items.code.first_name}`}
						>
							{items.description}
						</option>
					))}
				</Select>
				{errors().icd_10_problem && (
					<FormErrorMessage>{errors().icd_10_problem}</FormErrorMessage>
				)}
			</FormControl>

			<FormLabel mb={2} my={3} color={"#095FBA"}>
				{"Type"}
				<span style={{ color: "red" }}>*</span>
			</FormLabel>
			<FormControl mt={2} isInvalid={(errors().issue_type || []).length !== 0}>
				<Select
					placeholder={"Type"}
					name={"issue_type"}
					bgColor={"#FFFFFF"}
					borderColor={"#095FBA"}
				>
					<option value="Acute">Acute</option>
					<option value="Chronic">Chronic</option>
				</Select>
				{errors().issue_type && (
					<FormErrorMessage>{errors().issue_type}</FormErrorMessage>
				)}
			</FormControl>

			<FormLabel mb={2} my={3} color={"#095FBA"}>
				{"Start Date"}
				<span style={{ color: "red" }}>*</span>
			</FormLabel>
			<FormControl mt={2} isInvalid={(errors().start_date || []).length !== 0}>
				<DateComponent
					type={"datetime-local"}
					name={"start_date"}
					placeholder={"Start Date"}
					max={yesterday}
				/>
				{errors().start_date && (
					<FormErrorMessage>{errors().start_date}</FormErrorMessage>
				)}
			</FormControl>

			<FormLabel mb={2} my={3} color={"#095FBA"}>
				{"End Date"}
				<span style={{ color: "red" }}>*</span>
			</FormLabel>
			<FormControl mt={2} isInvalid={(errors().end_date || []).length !== 0}>
				<DateComponent
					type={"datetime-local"}
					name={"end_date"}
					placeholder={"End Date"}
					max={yesterday}
				/>
				{errors().end_date && (
					<FormErrorMessage>{errors().end_date}</FormErrorMessage>
				)}
			</FormControl>
			<FormLabel mb={2} my={3} color={"#095FBA"}>
				{"Comments"}
			</FormLabel>
			<FormControl mt={2} isInvalid={(errors().comment || []).length !== 0}>
				<Textarea
					bgColor={"#FFFFFF"}
					name={"comment"}
					borderColor={"#095FBA"}
					placeholder={"Comments"}
					maxLength={100}
				/>
				{errors().comment && (
					<FormErrorMessage>{errors().comment}</FormErrorMessage>
				)}
			</FormControl>

			<SaveAddAnotherButton path={`/problem/add/${props.patientId}`} />
			<SubmitButton loading={isSubmitting()} />
		</form>
	);
};
