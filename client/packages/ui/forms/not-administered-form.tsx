import { useForm } from "@felte/react";
import { SubmitButton } from "../component/submit-button";
import type { z } from "zod";
import { validator } from "@felte/validator-zod";
import reporterDom from "@felte/reporter-dom";
import { convertToUTC } from "../component/convert-to-utc";
import {
	FormLabel,
	FormControl,
	FormErrorMessage,
	Select,
	Textarea,
} from "@chakra-ui/react";
import type {
	LastUpdatedInput,
	NotAdministeredAdd,
	Types,
} from "@repo/types/dto";
import { notAdministeredAddSchema } from "@repo/types/validation";
import type { NotAdministeredStateExtend } from "@repo/types/dexie-state";
import { convertUTCtoLocal } from "../component/utc-date-to-normal-date";
import DateComponent from "../component/date-comp";

export type AddNotAdministeredFormProps = {
	patientId: string;
	edit?: boolean;
	vaccine?: string[];
	lastUpdatedInput: () => Promise<LastUpdatedInput>;
	initialValues: NotAdministeredStateExtend | undefined;
	onSubmit: (p: NotAdministeredAdd) => void;
};

export const NotAdministeredForm = (props: AddNotAdministeredFormProps) => {
	if (props.initialValues === undefined && props.edit === true) {
		return <p>Loading...</p>;
	}
	const { form, errors, isSubmitting } = useForm<
		z.infer<typeof notAdministeredAddSchema>
	>({
		onSubmit: async (values) => {
			const utcDate = convertToUTC(values.recorded);
			const modifiedValues = {
				...values,
				last_updated_input: await props.lastUpdatedInput(),
				recorded: utcDate,
			};
			props.onSubmit(modifiedValues);
		},
		initialValues: {
			patient_id: props.patientId,
			vaccine: props.initialValues?.vaccine,
			types: props.initialValues?.types as Types,
			recorded:
				props.initialValues?.recorded &&
				convertUTCtoLocal(props.initialValues.recorded),
			reason_for_non_administration:
				props.initialValues?.reason_for_non_administration,
			comments: props.initialValues?.comments,
		},
		extend: [validator({ schema: notAdministeredAddSchema }), reporterDom()],
	});
	return (
		<div>
			<form ref={form}>
				<FormLabel mb={2} my={3} color={"#095FBA"}>
					Vaccine
					<span style={{ color: "red" }}>*</span>
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
					Type
					<span style={{ color: "red" }}>*</span>
				</FormLabel>
				<FormControl mt={2} isInvalid={(errors().types || []).length !== 0}>
					<Select
						placeholder="Select Type"
						name="types"
						bgColor="#FFFFFF"
						borderColor="#095FBA"
					>
						<option value="Type1">Type1</option>
						<option value="Type2">Type2</option>
					</Select>
					{errors().types && (
						<FormErrorMessage>{errors().types}</FormErrorMessage>
					)}
				</FormControl>

				<FormLabel mb={2} my={3} color={"#095FBA"}>
					Recorded Date & Time
					<span style={{ color: "red" }}>*</span>
				</FormLabel>
				<FormControl mt={2} isInvalid={(errors().recorded || []).length !== 0}>
					<DateComponent
						type={"datetime-local"}
						name={"recorded"}
						placeholder={"Recorded Date & Time"}
						min={new Date().toISOString().slice(0, 16)}
					/>
					{errors().recorded && (
						<FormErrorMessage>{errors().recorded}</FormErrorMessage>
					)}
				</FormControl>

				<FormLabel mb={2} my={3} color={"#095FBA"}>
					Reason
					<span style={{ color: "red" }}>*</span>
				</FormLabel>
				<FormControl
					mt={2}
					isInvalid={
						(errors().reason_for_non_administration || []).length !== 0
					}
				>
					<Textarea
						bgColor="#FFFFFF"
						name="reason_for_non_administration"
						borderColor="#095FBA"
						placeholder="Enter your reason"
					/>
					{errors().reason_for_non_administration && (
						<FormErrorMessage>
							{errors().reason_for_non_administration}
						</FormErrorMessage>
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
