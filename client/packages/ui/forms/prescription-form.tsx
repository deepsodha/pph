import {
	FormLabel,
	Box,
	Input,
	FormControl,
	FormErrorMessage,
	Select as ChakraSelect,
	Textarea,
	Center,
	Heading,
} from "@chakra-ui/react";
import Select, { type MultiValue } from "react-select";
import { SubmitButton } from "../component/submit-button";
import type { z } from "zod";
import { useForm } from "@felte/react";
import { validator } from "@felte/validator-zod";
import reporterDom from "@felte/reporter-dom";
import { convertToUTC } from "../component/convert-to-utc";
import type { LastUpdatedInput, PrescriptionAdd } from "@repo/types/dto";
import { prescriptionInputSchema } from "@repo/types/validation";
import type {
	PatientStateExtend,
	PrescriptionStateExtend,
} from "@repo/types/dexie-state";
import { convertUTCtoLocal } from "../component/utc-date-to-normal-date";
import { useState, type ChangeEvent } from "react";
import DateComponent from "../component/date-comp";

export type drugData = {
	Brand: string;
	Generic: string;
	Details: string;
	Category: string;
	SideEffects: string;
	DrugDoseInfo: string;
	Precaution: string;
	ManufacturerName: string;
	Medicines: string;
	ContraIndications: string;
	Diseases: string;
	Interactions: string;
	Contains: string;
	id: number;
};

interface OptionType {
	value: string;
	label: string;
}
export type NewPrescriptionProps = {
	onSubmit: (p: PrescriptionAdd) => void;
	patient: PatientStateExtend[] | undefined;
	prescriptionId?: string;
	edit?: boolean;
	initialValues?: PrescriptionStateExtend | undefined;
	lastUpdatedInput: () => Promise<LastUpdatedInput>;
	drug_name?: drugData[] | undefined;
};

export const PrescriptionFrom = (props: NewPrescriptionProps) => {
	const patientInitialValue = props.initialValues?.patient_id
		? `${props.initialValues.patient_id}:${props.initialValues.patient_name}`
		: "";
	function handleSelectChange(e: ChangeEvent<HTMLSelectElement>) {
		const [patientId = "", patient_name = ""] = e.target.value.split(":");
		setData("patient_id", patientId);
		setData("patient_name", patient_name);
	}
	if (props.initialValues === undefined && props.edit === true) {
		return <p>Loading...</p>;
	}

	const [, setSelectedRoles] = useState<string[]>(
		props.initialValues?.drug_name || ["dummy"],
	);

	const handleRoleChange = (newValue: MultiValue<OptionType>) => {
		if (newValue) {
			const selectedRoles: Array<string> = newValue.map(
				(option) => option.value,
			);
			setSelectedRoles(selectedRoles);
			setData("drug_name", selectedRoles);
		} else {
			setSelectedRoles([]);
		}
	};
	const { form, errors, isDirty, isSubmitting, setData, data } = useForm<
		z.infer<typeof prescriptionInputSchema>
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
			patient_id: props.initialValues?.patient_id || "",
			patient_name: props.initialValues?.patient_name || "",
			presecription_name: props.initialValues?.presecription_name || "",
			instruction: props.initialValues?.instruction || "",
			date:
				(props.initialValues?.date &&
					convertUTCtoLocal(props.initialValues.date)) ||
				"",
			drug_name: (props.initialValues?.drug_name as Array<string>) || ["dummy"],
		},
		extend: [validator({ schema: prescriptionInputSchema }), reporterDom()],
	});
	const today = new Date().toISOString().slice(0, 16);

	return (
		<Center>
			<Box width={{ md: "80%", base: "90%", lg: "70%" }}>
				<form ref={form}>
					<FormLabel mb={2} my={3} color={"#095FBA"}>
						{"Patient Name"}
						<span style={{ color: "red" }}>*</span>
					</FormLabel>
					<FormControl
						mt={2}
						isInvalid={(errors().patient_name || []).length !== 0}
					>
						<ChakraSelect
							placeholder="Patient Name"
							name="patient_name"
							bgColor="#FFFFFF"
							borderColor="#095FBA"
							value={isDirty() ? undefined : patientInitialValue}
							onChange={(e) => handleSelectChange(e)}
						>
							{props.patient?.map((items) => (
								<option
									key={items.id}
									value={`${items.id}:${items.user.first_name}`}
								>
									{items.user.first_name}
								</option>
							))}
						</ChakraSelect>
						{errors().patient_name && (
							<FormErrorMessage>{errors().patient_name}</FormErrorMessage>
						)}
					</FormControl>

					<FormLabel mb={2} my={3} color={"#095FBA"}>
						{"Prescription Name"}
						<span style={{ color: "red" }}>*</span>
					</FormLabel>
					<FormControl
						mt={2}
						isInvalid={(errors().presecription_name || []).length !== 0}
					>
						<Input
							type="text"
							name="presecription_name"
							bgColor="#FFFFFF"
							borderColor="#095FBA"
							placeholder={"Prescription Name"}
						/>
						{errors().presecription_name && (
							<FormErrorMessage>{errors().presecription_name}</FormErrorMessage>
						)}
					</FormControl>

					<FormLabel mb={2} my={3} color={"#095FBA"}>
						{"Instruction"}
						<span style={{ color: "red" }}>*</span>
					</FormLabel>
					<FormControl
						mt={2}
						isInvalid={(errors().instruction || []).length !== 0}
					>
						<Textarea
							bgColor={"#FFFFFF"}
							name={"instruction"}
							borderColor={"#095FBA"}
							placeholder={"Instruction"}
							maxLength={200}
						/>
						{errors().instruction && (
							<FormErrorMessage>{errors().instruction}</FormErrorMessage>
						)}
					</FormControl>

					<FormLabel mb={2} my={3} color={"#095FBA"}>
						Date
						<span style={{ color: "red" }}>*</span>
					</FormLabel>
					<FormControl mt={2} isInvalid={(errors().date || []).length !== 0}>
						<DateComponent
							type={"datetime-local"}
							name={"date"}
							placeholder={"Date"}
							min={today}
						/>
						{errors().date && (
							<FormErrorMessage>{errors().date}</FormErrorMessage>
						)}
					</FormControl>

					{/* TODO SearchBar to be refactored below */}
					<FormLabel mb={2} my={3} color={"#095FBA"}>
						Drug Name
						<span style={{ color: "red" }}>*</span>
					</FormLabel>
					<FormControl
						mt={2}
						isInvalid={(errors().drug_name || []).length !== 0}
					>
						<Select
							name="drug_name"
							placeholder="Drugs"
							isMulti
							onChange={handleRoleChange}
							defaultValue={
								Array.isArray(data().drug_name)
									? data().drug_name.map((drug_name) => ({
											value: drug_name,
											label: drug_name,
										}))
									: []
							}
							closeMenuOnSelect={false}
							options={props.drug_name?.map((option) => ({
								value: option.Brand,
								label: option.Brand,
							}))}
							styles={{
								option: (provided) => ({
									...provided,
									background: "white",
									borderRadius: "3px",
								}),
							}}
						/>
						{errors().drug_name && (
							<FormErrorMessage>{errors().drug_name}</FormErrorMessage>
						)}
					</FormControl>
					<Heading fontSize={"sm"} color="red">
						{errors().drug_name}
					</Heading>
					<SubmitButton loading={isSubmitting()} />
				</form>
			</Box>
		</Center>
	);
};
