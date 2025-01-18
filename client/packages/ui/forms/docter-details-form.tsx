import { SubmitButton } from "../component/submit-button";
import {
	Switch,
	Input,
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
import type { DoctorAdd, DoctorType, LastUpdatedInput } from "@repo/types/dto";
import { useState } from "react";
import { doctorInputSchema } from "@repo/types/validation";
import type {
	DoctorStateExtend,
	UserStateExtend,
} from "@repo/types/dexie-state";
import Select, { type MultiValue } from "react-select";

export type AddDocterDetailsProps = {
	onSubmit: (p: DoctorAdd) => void;
	lastUpdatedInput: () => Promise<LastUpdatedInput>;
	doctorId?: string;
	edit?: boolean;
	user: UserStateExtend[] | undefined;
	image_url1: string;
	image_url2: string;
	initialValues?: DoctorStateExtend | undefined;
};
interface OptionType {
	value: DoctorType;
	label: DoctorType;
}

const options: OptionType[] = [
	{ value: "FamilyMedicinePhysician", label: "FamilyMedicinePhysician" },
	{ value: "Pediatrician", label: "Pediatrician" },
	{ value: "Gynecologist", label: "Gynecologist" },
	{ value: "Cardiologist", label: "Cardiologist" },
	{ value: "Pharmacist", label: "Pharmacist" },
	{ value: "Dermatologist", label: "Dermatologist" },
	{ value: "Psychiatrist", label: "Psychiatrist" },
	{ value: "Surgeon", label: "Surgeon" },
];

export const DocterDetailsForm = (props: AddDocterDetailsProps) => {
	if (props.initialValues === undefined && props.edit === true) {
		return <p>Loading...</p>;
	}

	const [selectedRoles, setSelectedRoles] = useState<DoctorType[]>(
		props.initialValues?.doctor_role || ["Cardiologist"],
	);
	const handleRoleChange = (newValue: MultiValue<OptionType>) => {
		if (newValue) {
			const selectedRoles: DoctorType[] = newValue.map(
				(option) => option.value as DoctorType,
			);
			setSelectedRoles(selectedRoles);
			setData("doctor_role", selectedRoles);
		} else {
			setSelectedRoles([]);
		}
	};
	const userInitialValue = props.initialValues?.user.id
		? `${props.initialValues.user.id}:${props.initialValues.user.user.first_name}`
		: "";

	const { form, errors, isSubmitting, setData, isDirty } = useForm<
		z.infer<typeof doctorInputSchema>
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
			doctor_role: (props.initialValues?.doctor_role as Array<DoctorType>) || [
				"Cardiologist",
			],
			docter_register_number: props.initialValues?.doctor_register_number || "",
			doctor_department: props.initialValues?.doctor_department || "",
			doctor_speciality: props.initialValues?.doctor_speciality || "",

			emergency: props.initialValues?.emergency,
		},

		extend: [validator({ schema: doctorInputSchema }), reporterDom()],
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
						Doctor Role
						<span style={{ color: "red" }}>*</span>
					</FormLabel>
					<FormControl
						mt={2}
						isInvalid={(errors().doctor_role || []).length !== 0}
					>
						<Select
							name="doctor_role"
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
						{errors().doctor_role && (
							<FormErrorMessage>{errors().doctor_role}</FormErrorMessage>
						)}
					</FormControl>
					<FormLabel mb={2} my={3} color={"#095FBA"}>
						Doctor Registration Number
						<span style={{ color: "red" }}>*</span>
					</FormLabel>
					<FormControl
						mt={2}
						isInvalid={(errors().docter_register_number || []).length !== 0}
					>
						<Input
							type="text"
							name="docter_register_number"
							bgColor={"#FFFFFF"}
							borderColor={"#095FBA"}
							placeholder="Doctor Registration Number"
						/>
						{errors().docter_register_number && (
							<FormErrorMessage>
								{errors().docter_register_number}
							</FormErrorMessage>
						)}
					</FormControl>

					<FormLabel mb={2} my={3} color={"#095FBA"}>
						Doctor Department
						<span style={{ color: "red" }}>*</span>
					</FormLabel>
					<FormControl
						mt={2}
						isInvalid={(errors().doctor_department || []).length !== 0}
					>
						<Input
							type="text"
							name="doctor_department"
							bgColor={"#FFFFFF"}
							borderColor={"#095FBA"}
							placeholder="Doctor Department"
						/>
						{errors().doctor_department && (
							<FormErrorMessage>{errors().doctor_department}</FormErrorMessage>
						)}
					</FormControl>

					<FormLabel mb={2} my={3} color={"#095FBA"}>
						Doctor speciality
						<span style={{ color: "red" }}>*</span>
					</FormLabel>
					<FormControl
						mt={2}
						isInvalid={(errors().doctor_speciality || []).length !== 0}
					>
						<Input
							type="text"
							name="doctor_speciality"
							bgColor={"#FFFFFF"}
							borderColor={"#095FBA"}
							placeholder="Doctor Specilty"
						/>
						{errors().doctor_speciality && (
							<FormErrorMessage>{errors().doctor_speciality}</FormErrorMessage>
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
