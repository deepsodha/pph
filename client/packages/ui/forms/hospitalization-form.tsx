import {
	Box,
	Center,
	FormControl,
	FormErrorMessage,
	FormLabel,
	Input,
	Select,
	Text,
} from "@chakra-ui/react";
import { useForm } from "@felte/react";
import type { z } from "zod";
import { hospitalizationAddSchema } from "@repo/types/validation";
import { convertToUTC } from "../component/convert-to-utc";
import type {
	HospitalizationAdd,
	HospitalizationState,
	LastUpdatedInput,
} from "@repo/types/dto";
import { SubmitButton } from "../component/submit-button";
import { validator } from "@felte/validator-zod";
import reporterDom from "@felte/reporter-dom";
import { convertUTCtoLocal } from "../component/utc-date-to-normal-date";
import DateComponent from "../component/date-comp";

export type HospitalizationAddFormProps = {
	onSubmit: (v: HospitalizationAdd) => void;
	patientId?: string;
	hospitalizationId?: string;
	edit?: boolean;
	initialValues: HospitalizationState | undefined;
	lastUpdatedInput: () => Promise<LastUpdatedInput>;
};
export const HospitalizationForm = (props: HospitalizationAddFormProps) => {
	if (props.initialValues === undefined && props.edit === true) {
		return <p>Loading...</p>;
	}
	const today = new Date();
	const current = today.toISOString().slice(0, 16);
	const { form, errors, isSubmitting } = useForm<
		z.infer<typeof hospitalizationAddSchema>
	>({
		onSubmit: async (values) => {
			const utcDate = convertToUTC(values.admission_date ?? "");
			const modifiedValues = {
				...values,
				last_updated_input: await props.lastUpdatedInput(),
				admission_date: utcDate,
			};

			props.onSubmit(modifiedValues);
		},
		initialValues: {
			patient_id: props.patientId,
			admission_date:
				(props.initialValues?.admission_date &&
					convertUTCtoLocal(props.initialValues.admission_date)) ||
				"",
			related_to: props.initialValues?.related_to || "",
			status: props.initialValues?.status || "",
			length_of_stay: props.initialValues?.length_of_stay || 0,
			procedure: props.initialValues?.procedure || "",
			comments: props.initialValues?.comments || "",
		},
		extend: [validator({ schema: hospitalizationAddSchema }), reporterDom()],
	});
	return (
		<form ref={form}>
			<FormLabel mb={2} my={3} color={"#095FBA"}>
				{"Admission Date"}
				<span style={{ color: "red" }}>*</span>
			</FormLabel>
			<FormControl
				mt={2}
				isInvalid={(errors().admission_date || []).length !== 0}
			>
				<DateComponent
					type={"datetime-local"}
					name={"admission_date"}
					placeholder={"Date (DD/MM/YYYY)"}
					max={current}
				/>
				{errors().admission_date && (
					<FormErrorMessage>{errors().admission_date}</FormErrorMessage>
				)}
			</FormControl>

			<FormLabel mb={2} my={3} color={"#095FBA"}>
				{"Related To"}
				<span style={{ color: "red" }}>*</span>
			</FormLabel>
			<FormControl mt={2} isInvalid={(errors().related_to || []).length !== 0}>
				<Select
					placeholder="Status"
					name="related_to"
					bgColor="#FFFFFF"
					borderColor="#095FBA"
				>
					<option value="Mild">Mild</option>
					<option value="Moderate">Moderate</option>
					<option value="Severe">Severe</option>
				</Select>
				{errors().related_to && (
					<FormErrorMessage>{errors().related_to}</FormErrorMessage>
				)}
			</FormControl>
			<FormLabel mb={2} my={3} color={"#095FBA"}>
				{"Status"}
				<span style={{ color: "red" }}>*</span>
			</FormLabel>
			<FormControl mt={2} isInvalid={(errors().status || []).length !== 0}>
				<Select
					placeholder="Status"
					name="status"
					bgColor="#FFFFFF"
					borderColor="#095FBA"
				>
					<option value="Mild">Mild</option>
					<option value="Moderate">Moderate</option>
					<option value="Severe">Severe</option>
				</Select>
				{errors().status && (
					<FormErrorMessage>{errors().status}</FormErrorMessage>
				)}
			</FormControl>
			<FormLabel mb={2} my={3} color={"#095FBA"}>
				{"Length Of Stay"}
				<span style={{ color: "red" }}>*</span>
			</FormLabel>
			<Box width="100%" display="flex">
				<FormControl isInvalid={(errors().length_of_stay || []).length !== 0}>
					<Input
						type="number"
						name="length_of_stay"
						bgColor="#FFFFFF"
						borderColor="#095FBA"
						placeholder={"Length Of Stay"}
					/>
					{errors().length_of_stay && (
						<FormErrorMessage>{errors().length_of_stay}</FormErrorMessage>
					)}
				</FormControl>
				<Box
					width="20%"
					ml="2%"
					border="1px"
					borderColor="#095FBA"
					bgColor="#FFFFFF"
					height="40px"
					borderRadius="md"
					alignItems="center"
				>
					<Center>
						<Text mt={1}>Days</Text>
					</Center>
				</Box>
			</Box>
			<FormLabel mb={2} my={3} color={"#095FBA"}>
				{"Procedure"}
				<span style={{ color: "red" }}>*</span>
			</FormLabel>
			<FormControl mt={2} isInvalid={(errors().procedure || []).length !== 0}>
				<Input
					type="Text"
					name="procedure"
					bgColor="#FFFFFF"
					borderColor="#095FBA"
					placeholder={"Type CPT Procedure Code"}
				/>
				{errors().procedure && (
					<FormErrorMessage>{errors().procedure}</FormErrorMessage>
				)}
			</FormControl>
			<FormLabel mb={2} my={3} color={"#095FBA"}>
				{"Comments"}
			</FormLabel>
			<FormControl mt={2} isInvalid={(errors().comments || []).length !== 0}>
				<Input
					type="Text"
					name="comments"
					bgColor="#FFFFFF"
					borderColor="#095FBA"
					placeholder={"Comments"}
					maxLength={100}
				/>
				{errors().comments && (
					<FormErrorMessage>{errors().comments}</FormErrorMessage>
				)}
			</FormControl>
			<SubmitButton loading={isSubmitting()} />
		</form>
	);
};
