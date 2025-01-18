import { useForm } from "@felte/react";
import { SubmitButton } from "../component/submit-button";
import type { z } from "zod";
import { validator } from "@felte/validator-zod";
import reporterDom from "@felte/reporter-dom";
import {
	FormLabel,
	FormControl,
	Input,
	FormErrorMessage,
	Textarea,
	Select,
	NumberInput,
	NumberInputField,
} from "@chakra-ui/react";
import type { Getorganization, OrganizationAdd } from "@repo/types/dto";
import { organizationAddSchema } from "@repo/types/validation";
import { CountryData } from "../../../apps/pi-health-mobile/src/atoms/city-country-state";

export type OrganizationFormProps = {
	initialValues: Getorganization | undefined;
	onSubmit: (p: OrganizationAdd) => void;
	country?: string[];
	city?: string[];
	state?: string[];
};

export const OrganizationForm = (props: OrganizationFormProps) => {
	const { form, errors, data, isSubmitting, setData, setIsDirty, setTouched } =
		useForm<z.infer<typeof organizationAddSchema>>({
			onSubmit: async (values) => {
				props.onSubmit(values);
			},
			initialValues: {
				name: props.initialValues?.name || "",
				details: props.initialValues?.details || "",
				phone_number: props.initialValues?.mobile_number || "",
				email: props.initialValues?.email || "",
				address: {
					pin_code: props.initialValues?.pin_code || "",
					city: props.initialValues?.city || "",
					state: props.initialValues?.state || "",
					address_line: props.initialValues?.address_line || "",
					country: props.initialValues?.country || "",
				},
			},
			extend: [validator({ schema: organizationAddSchema }), reporterDom()],
		});
	const { updateCountry, updateState } = CountryData();

	return (
		<div>
			<form ref={form}>
				<FormLabel mb={2} my={3} color={"#095FBA"}>
					{"Organization Name"}
					<span style={{ color: "red" }}>*</span>
				</FormLabel>
				<FormControl mt={2} isInvalid={(errors().name || []).length !== 0}>
					<Input
						type="text"
						name="name"
						bgColor="#FFFFFF"
						borderColor="#095FBA"
						placeholder={"Organization Name"}
					/>
					{errors().name && (
						<FormErrorMessage>{errors().name}</FormErrorMessage>
					)}
				</FormControl>

				<FormLabel mb={2} my={3} color={"#095FBA"}>
					{"Organization Details"}
					<span style={{ color: "red" }}>*</span>
				</FormLabel>
				<FormControl mt={2} isInvalid={(errors().details || []).length !== 0}>
					<Textarea
						bgColor={"#FFFFFF"}
						name={"details"}
						borderColor={"#095FBA"}
						placeholder={"Organization Details"}
						maxLength={100}
					/>
					{errors().details && (
						<FormErrorMessage>{errors().details}</FormErrorMessage>
					)}
				</FormControl>

				{/* <FormLabel mb={2} my={3} color={"#095FBA"}>
					{"Phone Number"}
				</FormLabel>
				<FormControl
					mt={2}
					isInvalid={(errors().phone_number || []).length !== 0}
				>
					<Input
						type="text"
						name="phone_number"
						bgColor="#FFFFFF"
						borderColor="#095FBA"
						placeholder={"Phone Number"}
					/>
					{errors().phone_number && (
						<FormErrorMessage>{errors().phone_number}</FormErrorMessage>
					)}
				</FormControl> */}

				<FormLabel mb={2} my={3} color={"#095FBA"}>
					Phone Number<span style={{ color: "red" }}>*</span>
				</FormLabel>
				<FormControl
					mt={2}
					isInvalid={(errors().phone_number || []).length !== 0}
				>
					<NumberInput
						onChange={(valueAsString) => {
							setData("phone_number", valueAsString);
							setIsDirty(true);
							setTouched("phone_number", true);
						}}
						onBlur={() => setTouched("phone_number", true)}
						borderColor={"#095FBA"}
						value={data().phone_number}
					>
						<NumberInputField
							placeholder="Phone Number"
							height={10}
							name="phone_number"
							maxLength={10}
							bgColor={"#FFFFFF"}
							borderColor={"#095FBA"}
						/>
					</NumberInput>
					{errors().phone_number && (
						<FormErrorMessage>{errors().phone_number}</FormErrorMessage>
					)}
				</FormControl>

				<FormLabel mb={2} my={3} color={"#095FBA"}>
					{"Email"}
					<span style={{ color: "red" }}>*</span>
				</FormLabel>
				<FormControl mt={2} isInvalid={(errors().email || []).length !== 0}>
					<Input
						type="text"
						name="email"
						bgColor="#FFFFFF"
						borderColor="#095FBA"
						placeholder={"Email"}
					/>
					{errors().email && (
						<FormErrorMessage>{errors().email}</FormErrorMessage>
					)}
				</FormControl>
				<FormLabel mb={2} my={3} color={"#095FBA"}>
					{"Country"}
					<span style={{ color: "red" }}>*</span>
				</FormLabel>
				<FormControl
					mt={2}
					isInvalid={(errors().address.country || []).length !== 0}
				>
					<Select
						placeholder="Country"
						name="address.country"
						bgColor="#FFFFFF"
						borderColor="#095FBA"
						onChange={() => updateCountry(data().address.country)}
					>
						{props.country?.map((country) => (
							<option key={country} value={country}>
								{country}
							</option>
						))}
					</Select>
					{errors().address.country && (
						<FormErrorMessage>{errors().address.country}</FormErrorMessage>
					)}
				</FormControl>
				<FormLabel mb={2} my={3} color={"#095FBA"}>
					{"State"}
					<span style={{ color: "red" }}>*</span>
				</FormLabel>
				<FormControl
					mt={2}
					isInvalid={(errors().address.state || []).length !== 0}
				>
					<Select
						placeholder="State"
						name="address.state"
						bgColor="#FFFFFF"
						borderColor="#095FBA"
						onChange={() => updateState(data().address.state)}
					>
						{(props.state || []).map((state) => (
							<option key={state} value={state}>
								{state}
							</option>
						))}
					</Select>
					{errors().address.state && (
						<FormErrorMessage>{errors().address.state}</FormErrorMessage>
					)}
				</FormControl>

				<FormLabel mb={2} my={3} color={"#095FBA"}>
					{"City"}
					<span style={{ color: "red" }}>*</span>
				</FormLabel>
				<FormControl
					mt={2}
					isInvalid={(errors().address.city || []).length !== 0}
				>
					<Select
						placeholder="City"
						name="address.city"
						bgColor="#FFFFFF"
						borderColor="#095FBA"
					>
						{(props.city || []).map((state) => (
							<option key={state} value={state}>
								{state}
							</option>
						))}
					</Select>
					{errors().address.city && (
						<FormErrorMessage>{errors().address.city}</FormErrorMessage>
					)}
				</FormControl>

				<FormLabel mb={2} my={3} color={"#095FBA"}>
					{"Address"}
					<span style={{ color: "red" }}>*</span>
				</FormLabel>
				<FormControl
					mt={2}
					isInvalid={(errors().address.address_line || []).length !== 0}
				>
					<Input
						type="text"
						name="address.address_line"
						bgColor="#FFFFFF"
						borderColor="#095FBA"
						placeholder={"Address"}
						maxLength={100}
					/>
					{errors().address.address_line && (
						<FormErrorMessage>{errors().address.address_line}</FormErrorMessage>
					)}
				</FormControl>

				<FormLabel mb={2} my={3} color={"#095FBA"}>
					{"Pin Code"}
					<span style={{ color: "red" }}>*</span>
				</FormLabel>

				<FormControl
					mt={2}
					isInvalid={(errors().address?.pin_code || []).length !== 0}
				>
					<NumberInput
						onChange={(valueAsString) => {
							setData("address.pin_code", valueAsString);
							setIsDirty(true);
							setTouched("address.pin_code", true);
						}}
						onBlur={() => setTouched("address.pin_code", true)}
						borderColor={"#095FBA"}
						value={data().address.pin_code}
					>
						<NumberInputField
							placeholder="Pin Code"
							height={10}
							name="address.pin_code"
							maxLength={6}
							bgColor={"#FFFFFF"}
							borderColor={"#095FBA"}
						/>
					</NumberInput>

					{errors().address?.pin_code && (
						<FormErrorMessage>{errors().address?.pin_code}</FormErrorMessage>
					)}
				</FormControl>

				<SubmitButton loading={isSubmitting()} />
			</form>
		</div>
	);
};
