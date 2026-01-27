import {
	Center,
	FormLabel,
	FormControl,
	Input,
	Box,
	FormErrorMessage,
	NumberInput,
	NumberInputField,
} from "@chakra-ui/react";
import { SubmitButton } from "../component";
import { useForm } from "@felte/react";
import type { GetUsers, RoleType, Users } from "@repo/types/dto";
import { validator } from "@felte/validator-zod";
import reporterDom from "@felte/reporter-dom";
import type { z } from "zod";
import { UserSchema } from "@repo/types/validation";
import ReactSelect, { type MultiValue } from "react-select";
import { useState } from "react";

export type UsersFormProps = {
	onSubmit: (p: Users) => void;
	UserId?: string;
	edit?: boolean;
	initialValues?: GetUsers | undefined;
};

interface OptionType {
	value: RoleType;
	label: RoleType;
}
const options: OptionType[] = [
	{ value: "SuperAdmin", label: "SuperAdmin" },
	{ value: "SystemAdmin", label: "SystemAdmin" },
];
export const UsersForm = (props: UsersFormProps) => {
	const { form, errors, setData, setIsDirty, setTouched, data, isSubmitting } =
		useForm<z.infer<typeof UserSchema>>({
			onSubmit: (values) => {
				props.onSubmit(values);
			},
			initialValues: { mobile_number: "", password: "", role: ["SuperAdmin"] },
			extend: [validator({ schema: UserSchema }), reporterDom()],
		});
	const [selectedRoles, setSelectedRoles] = useState<RoleType[]>([
		"SystemAdmin",
	]);
	const handleRoleChange = (newValue: MultiValue<OptionType>) => {
		if (newValue) {
			const selectedRoles: RoleType[] = newValue.map(
				(option) => option.value as RoleType,
			);
			setSelectedRoles(selectedRoles);
			setData("role", selectedRoles);
		} else {
			setSelectedRoles([]);
		}
	};
	return (
		<div>
			<form ref={form}>
				<Center>
					<Box width={{ md: "80%", base: "90%", lg: "70%" }}>
						<FormLabel mb={2} my={3} color={"#095FBA"}>
							Mobile Number
						</FormLabel>
						<FormControl
							mt={2}
							isInvalid={(errors().mobile_number || []).length !== 0}
						>
							<NumberInput
								onChange={(valueAsString) => {
									setData("mobile_number", valueAsString);
									setIsDirty(true);
									setTouched("mobile_number", true);
								}}
								onBlur={() => setTouched("mobile_number", true)}
								borderColor={"#095FBA"}
								value={data().mobile_number}
							>
								<NumberInputField
									placeholder="Phone Number"
									height={10}
									name="mobile_number"
									maxLength={10}
									bgColor={"#FFFFFF"}
									borderColor={"#095FBA"}
								/>
							</NumberInput>
							{errors().mobile_number && (
								<FormErrorMessage>{errors().mobile_number}</FormErrorMessage>
							)}
						</FormControl>
						<FormLabel mb={2} my={3} color={"#095FBA"}>
							Role
						</FormLabel>
						<FormControl
							mt={2}
							isInvalid={(errors().role || []).length !== 0}
							// hidden={true}
						>
							<ReactSelect
								name="role"
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
										borderColor: "#095FBA",
									}),
								}}
							/>

							{errors().role && (
								<FormErrorMessage>{errors().role}</FormErrorMessage>
							)}
						</FormControl>
						<FormLabel mb={2} my={3} color={"#095FBA"}>
							Password
						</FormLabel>
						<FormControl
							mt={2}
							isInvalid={(errors().password || []).length !== 0}
						>
							<Input
								type="text"
								name="password"
								bgColor={"#FFFFFF"}
								borderColor={"#095FBA"}
								placeholder="Password"
								// onChange={handleChange}
							/>

							{errors().password && (
								<FormErrorMessage>{errors().password}</FormErrorMessage>
							)}
						</FormControl>
						<SubmitButton loading={isSubmitting()} />
					</Box>
				</Center>
			</form>
		</div>
	);
};
