import { useForm } from "@felte/react";
import { SubmitButton } from "../component/submit-button";
import type { z } from "zod";
import { validator } from "@felte/validator-zod";
import reporterDom from "@felte/reporter-dom";
import { convertToUTC } from "../component/convert-to-utc";
import {
	FormLabel,
	FormControl,
	Input,
	FormErrorMessage,
	Select,
	Textarea,
} from "@chakra-ui/react";
import type {
	DoctorType,
	HistoricalAdd,
	LastUpdatedInput,
} from "@repo/types/dto";
import { historicalAddSchema } from "@repo/types/validation";
import type { HistoricalStateExtend } from "@repo/types/dexie-state";
import { convertUTCtoLocal } from "../component/utc-date-to-normal-date";
import DateComponent from "../component/date-comp";

export type HistoricalFormProps = {
	initialValues: HistoricalStateExtend | undefined;
	onSubmit: (p: HistoricalAdd) => void;
	lastUpdatedInput: () => Promise<LastUpdatedInput>;
	edit?: boolean;
	vaccine?: string[];
	patientId?: string;
};

export const HistoricalForm = (props: HistoricalFormProps) => {
	if (props.initialValues === undefined && props.edit === true) {
		return <p>Loading...</p>;
	}
	const { form, errors, isSubmitting } = useForm<
		z.infer<typeof historicalAddSchema>
	>({
		onSubmit: async (values) => {
			const utcDate = convertToUTC(values.date);
			const modifiedValues = {
				...values,
				last_updated_input: await props.lastUpdatedInput(),
				date: utcDate,
			};
			props.onSubmit(modifiedValues);
		},
		initialValues: {
			patient_id: props.patientId,
			vaccine: props.initialValues?.vaccine,
			types: props.initialValues?.types,
			date:
				(props.initialValues?.date &&
					convertUTCtoLocal(props.initialValues.date)) ||
				"",
			number_in_series: props.initialValues?.number_in_series,
			provider: (props.initialValues?.provider as DoctorType) || "Doctor",
			source_of_information: props.initialValues?.source_of_information,
			comments: props.initialValues?.comments || "",
		},
		extend: [validator({ schema: historicalAddSchema }), reporterDom()],
	});
	return (
		<div>
			<form ref={form}>
				<FormLabel mb={2} my={3} color={"#095FBA"}>
					{"Vaccine"}
				</FormLabel>
				<FormControl mt={2} isInvalid={(errors().vaccine || []).length !== 0}>
					<Select
						placeholder="vaccine"
						name="vaccine"
						bgColor="#FFFFFF"
						borderColor="#095FBA"
					>
						{props.vaccine?.map((vaccine) => (
							<option key={vaccine} value={vaccine}>
								{vaccine}
							</option>
						))}
					</Select>
					{errors().vaccine && (
						<FormErrorMessage>{errors().vaccine}</FormErrorMessage>
					)}
				</FormControl>

				<FormLabel mb={2} my={3} color={"#095FBA"}>
					{"Type"}
				</FormLabel>
				<FormControl mt={2} isInvalid={(errors().types || []).length !== 0}>
					<Select
						placeholder={"Type"}
						name={"types"}
						bgColor={"#FFFFFF"}
						borderColor={"#095FBA"}
					>
						<option value="Type A">Type A</option>
						<option value="Type B">Type B</option>
					</Select>
					{errors().types && (
						<FormErrorMessage>{errors().types}</FormErrorMessage>
					)}
				</FormControl>

				<FormLabel mb={2} my={3} color={"#095FBA"}>
					{"Date"}
				</FormLabel>
				<FormControl mt={2} isInvalid={(errors().date || []).length !== 0}>
					<DateComponent
						type={"datetime-local"}
						name={"date"}
						placeholder={"Date"}
						min={new Date().toISOString().slice(0, 16)}
					/>
					{errors().date && (
						<FormErrorMessage>{errors().date}</FormErrorMessage>
					)}
				</FormControl>

				<FormLabel mb={2} my={3} color={"#095FBA"}>
					{"Number in Series"}
				</FormLabel>
				<FormControl
					mt={2}
					isInvalid={(errors().number_in_series || []).length !== 0}
				>
					<Input
						type={"text"}
						name={"number_in_series"}
						bgColor={"#FFFFFF"}
						borderColor={"#095FBA"}
						placeholder={"Enter Number"}
					/>
					{errors().number_in_series && (
						<FormErrorMessage>{errors().number_in_series}</FormErrorMessage>
					)}
				</FormControl>

				<FormLabel mb={2} my={3} color={"#095FBA"}>
					{"Enter Provider"}
				</FormLabel>
				<FormControl mt={2} isInvalid={(errors().provider || []).length !== 0}>
					<Input
						type={"text"}
						name={"provider"}
						bgColor={"#FFFFFF"}
						borderColor={"#095FBA"}
						placeholder={"Enter Provider"}
					/>
					{errors().provider && (
						<FormErrorMessage>{errors().provider}</FormErrorMessage>
					)}
				</FormControl>

				<FormLabel mb={2} my={3} color={"#095FBA"}>
					{"Select Source of Information"}
				</FormLabel>
				<FormControl
					mt={2}
					isInvalid={(errors().source_of_information || []).length !== 0}
				>
					<Select
						placeholder={"Select Source of Information"}
						name={"source_of_information"}
						bgColor={"#FFFFFF"}
						borderColor={"#095FBA"}
					>
						<option value="Type A">Type A</option>
						<option value="Type B">Type B</option>
					</Select>
					{errors().source_of_information && (
						<FormErrorMessage>
							{errors().source_of_information}
						</FormErrorMessage>
					)}
				</FormControl>

				<FormLabel mb={2} my={3} color={"#095FBA"}>
					{"Comments"}
				</FormLabel>
				<FormControl mt={2} isInvalid={(errors().comments || []).length !== 0}>
					<Textarea
						bgColor={"#FFFFFF"}
						name={"comments"}
						borderColor={"#095FBA"}
						placeholder={"Comments"}
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
