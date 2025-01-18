import { SubmitButton } from "../component/submit-button";
import {
	Switch,
	FormLabel,
	Text,
	FormControl,
	FormErrorMessage,
	Box,
	Center,
	Select as ChakraSelect,
} from "@chakra-ui/react";
import { useForm } from "@felte/react";
import type { z } from "zod";
import { validator } from "@felte/validator-zod";
import reporterDom from "@felte/reporter-dom";
import type { StaffAdd, StaffType, LastUpdatedInput } from "@repo/types/dto";
import { useState } from "react";
import { staffInputSchema } from "@repo/types/validation";
import type {
	StaffStateExtend,
	UserStateExtend,
} from "@repo/types/dexie-state";
import Select, { type MultiValue } from "react-select";

export type AddStaffDetailsProps = {
	onSubmit: (p: StaffAdd) => void;
	lastUpdatedInput: () => Promise<LastUpdatedInput>;
	StaffId?: string;
	edit?: boolean;
	user: UserStateExtend[] | undefined;
	image_url1: string;
	image_url2: string;
	initialValues?: StaffStateExtend | undefined;
};
interface OptionType {
	value: StaffType;
	label: StaffType;
}

const options: OptionType[] = [
	{ value: "Consultation", label: "Consultation" },
	{ value: "Examination", label: "Examination" },
	{ value: "Diagnosis", label: "Diagnosis" },
	{ value: "Treatment", label: "Treatment" },
	{ value: "Procedure", label: "Procedure" },
	{ value: "Surgery", label: "Surgery" },
	{ value: "Therapy", label: "Therapy" },
	{ value: "Counseling", label: "Counseling" },
	{ value: "MedicationManagement", label: "MedicationManagement" },
	{ value: "Imaging", label: "Imaging" },
	{ value: "LaboratoryTest", label: "LaboratoryTest" },
	{ value: "Rehabilitation", label: "Rehabilitation" },
	{ value: "HomeCare", label: "HomeCare" },
	{ value: "Telemedicine", label: "Telemedicine" },
	{ value: "WellnessProgram", label: "WellnessProgram" },
];

export const StaffDetailsForm = (props: AddStaffDetailsProps) => {
	const today = new Date();
	today.setDate(today.getDate() - 1);
	// const yesterday = today.toISOString().slice(0, 10);
	if (props.initialValues === undefined && props.edit === true) {
		return <p>Loading...</p>;
	}

	const [selectedRoles, setSelectedRoles] = useState<StaffType[]>(
		props.initialValues?.staff_department || ["Consultation"],
	);
	const userInitialValue = props.initialValues?.user.id
		? `${props.initialValues.user.id}:${props.initialValues.user.user.first_name}`
		: "";

	const handleRoleChange = (newValue: MultiValue<OptionType>) => {
		if (newValue) {
			const selectedRoles: StaffType[] = newValue.map(
				(option) => option.value as StaffType,
			);
			setSelectedRoles(selectedRoles);
			setData("staff_department", selectedRoles);
		} else {
			setSelectedRoles([]);
		}
	};
	const { form, errors, isSubmitting, setData, isDirty } = useForm<
		z.infer<typeof staffInputSchema>
	>({
		onSubmit: async (values) => {
			const [userId = ""] = values.user_id.split(":");
			const modifiedValues = {
				...values,
				user_id: userId,
				last_updated_input: await props.lastUpdatedInput(),
			};
			props.onSubmit(modifiedValues);
		},

		initialValues: {
			user_id: props.initialValues?.user.id,
			staff_department: (props.initialValues
				?.staff_department as Array<StaffType>) || ["Consultation"],
			emergency: props.initialValues?.emergency,
		},

		extend: [validator({ schema: staffInputSchema }), reporterDom()],
	});
	return (
		<Center>
			<Box width={{ md: "80%", base: "90%", lg: "70%" }}>
				<form ref={form}>
					<FormLabel mb={2} my={3} color={"#095FBA"}>
						User
						<span style={{ color: "red" }}>*</span>
					</FormLabel>
					<FormControl mt={2} isInvalid={(errors().user_id || []).length !== 0}>
						<ChakraSelect
							placeholder="User Name"
							name="user_id"
							bgColor="#FFFFFF"
							borderColor="#095FBA"
							value={isDirty() ? undefined : userInitialValue}
						>
							{props.user?.map((items) => (
								<option
									key={items.id}
									value={`${items.id}:${items.user.first_name}`}
								>
									{items.user.first_name} {items.phone.number}
								</option>
							))}
						</ChakraSelect>
						{errors().user_id && (
							<FormErrorMessage>{errors().user_id}</FormErrorMessage>
						)}
					</FormControl>
					<FormLabel mb={2} my={3} color={"#095FBA"}>
						Staff Department
						<span style={{ color: "red" }}>*</span>
					</FormLabel>
					<FormControl
						mt={2}
						isInvalid={(errors().staff_department || []).length !== 0}
					>
						<Select
							name="staff_department"
							defaultValue={selectedRoles.map((role) => ({
								value: role,
								label: role,
							}))}
							onChange={handleRoleChange}
							options={options.map((option) => ({
								value: option.value,
								label: option.label,
							}))}
							isMulti
							closeMenuOnSelect={false}
							styles={{
								option: (provided) => ({
									...provided,
									background: "white",
									borderRadius: "3px",
								}),
							}}
						/>

						{errors().staff_department && (
							<FormErrorMessage>{errors().staff_department}</FormErrorMessage>
						)}
					</FormControl>
					<FormControl display="flex" alignItems="center">
						<FormLabel htmlFor="email-alerts" mb="0" mt={4} color={"#095FBA"}>
							Emergency
						</FormLabel>
						<Switch id="email-alerts" size={"lg"} mt={4} name="emergency" />
					</FormControl>
					<Text color={"red"} fontSize="sm">
						{errors().emergency}
					</Text>
					<SubmitButton loading={isSubmitting()} />
				</form>
			</Box>
		</Center>
	);
};
