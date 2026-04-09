import { useForm } from "@felte/react";
import { SubmitButton } from "../component/submit-button";
import type { z } from "zod";
import { validator } from "@felte/validator-zod";
import reporterDom from "@felte/reporter-dom";
import {
	FormLabel,
	FormControl,
	FormErrorMessage,
	Select,
	Textarea,
} from "@chakra-ui/react";
import { convertToUTC } from "../component/convert-to-utc";
import type { LastUpdatedInput, OrderAdd } from "@repo/types/dto";
import type { Types } from "@repo/types/dto";
import { orderAddSchema } from "@repo/types/validation";
import type { OrderStateExtend } from "@repo/types/dexie-state";
import { convertUTCtoLocal } from "../component/utc-date-to-normal-date";
import DateComponent from "../component/date-comp";
export type OrderFormProps = {
	lastUpdatedInput: () => Promise<LastUpdatedInput>;
	onSubmit: (p: OrderAdd) => void;
	initialValues: OrderStateExtend | undefined;
	vaccine?: string[] | [" "];
	edit?: boolean;
	doctorList?: string[] | [" "];
	patientId?: string;
};
export const OrderForm = (props: OrderFormProps) => {
	if (props.initialValues === undefined && props.edit === true) {
		return <p>Loading...</p>;
	}
	const { form, errors, isSubmitting } = useForm<
		z.infer<typeof orderAddSchema>
	>({
		onSubmit: async (values) => {
			const utcDate = convertToUTC(values.ordered);
			const modifiedValues = {
				...values,
				last_updated_input: await props.lastUpdatedInput(),
				ordered: utcDate,
			};
			props.onSubmit(modifiedValues);
		},
		initialValues: {
			patient_id: props.patientId,
			vaccine: props.initialValues?.vaccine || "",
			types: (props.initialValues?.types as Types) || "",
			ordered:
				(props.initialValues?.ordered &&
					convertUTCtoLocal(props.initialValues.ordered)) ||
				"",
			provider: props.initialValues?.provider || "",
			comments: props.initialValues?.comments || "",
		},
		extend: [validator({ schema: orderAddSchema }), reporterDom()],
	});
	return (
		<div>
			<form ref={form}>
				<FormLabel mb={2} my={3} color={"#095FBA"}>
					{"Vaccine"}
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
					{"Type"}
					<span style={{ color: "red" }}>*</span>
				</FormLabel>
				<FormControl mt={2} isInvalid={(errors().types || []).length !== 0}>
					<Select
						placeholder="Select Type"
						name="types"
						bgColor="#FFFFFF"
						borderColor="#095FBA"
					>
						<option value="Type1">Type 1</option>
						<option value="Type2">Type 2</option>
					</Select>
					{errors().types && (
						<FormErrorMessage>{errors().types}</FormErrorMessage>
					)}
				</FormControl>

				<FormLabel mb={2} my={3} color={"#095FBA"}>
					{"Ordered Date & Time"}
					<span style={{ color: "red" }}>*</span>
				</FormLabel>
				<FormControl mt={2} isInvalid={(errors().ordered || []).length !== 0}>
					<DateComponent
						type={"datetime-local"}
						name={"ordered"}
						placeholder={"Ordered Date & Time"}
						min={new Date().toISOString().slice(0, 16)}
					/>
					{errors().ordered && (
						<FormErrorMessage>{errors().ordered}</FormErrorMessage>
					)}
				</FormControl>

				<FormLabel mb={2} my={3} color={"#095FBA"}>
					Provider
					<span style={{ color: "red" }}>*</span>
				</FormLabel>
				<FormControl mt={2} isInvalid={(errors().provider || []).length !== 0}>
					<Select
						placeholder="Select Provider"
						name="provider"
						bgColor="#FFFFFF"
						borderColor="#095FBA"
					>
						{props.doctorList?.map((doctor) => (
							<option key={doctor} value={doctor}>
								{doctor}
							</option>
						))}
					</Select>
					{errors().provider && (
						<FormErrorMessage>{errors().provider}</FormErrorMessage>
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
