import {
	FormControl,
	FormErrorMessage,
	FormLabel,
	Input,
	InputGroup,
	InputRightElement,
	Textarea,
	Image,
	Text,
	Box,
	Center,
	NumberInput,
	NumberInputField,
} from "@chakra-ui/react";

import type { z } from "zod";
import { SubmitButton } from "../component";
import { systemAdminAddSchema } from "@repo/types/validation";
import { useForm } from "@felte/react";
import type {
	GenderType,
	GovIdType,
	LastUpdatedInput,
	LocationRecord,
	PhoneNoTypeForContact,
	SystemAdminAdd,
} from "@repo/types/dto";
import type {
	OrganizationStateExtend,
	SystemAdminStateExtend,
} from "@repo/types/dexie-state";
import { convertToUTC } from "../component/convert-to-utc";
import { dobDate } from "../component/utc-date-to-normal-date";
import { useEffect, useState } from "react";
// import { CountryData } from "../../../apps/pi-health-mobile/src/atoms/city-country-state";
// import { SystemAdminCountryData } from "../../../apps/pi-health-mobile/src/atoms/system-admin-country";
import { validator } from "@felte/validator-zod";
import reporterDom from "@felte/reporter-dom";
import { HeadingTag } from "../component/heading-tag";
import type { SystemAdminRole } from "../../types/dto/DB/SystemAdminRole";
import ReactSelect, { type MultiValue } from "react-select";
import SelectGovernmentIdType from "../component/government-id-type";
import SelectGenderType from "../component/gender-types";
import SelectPhoneNumberType from "../component/phone-number-type";
import PhotoUrlInput from "../component/photo-url-input";
import DateOfBirth from "../component/date-of-birth";
export type AddSystemAdminFormProps1 = {
	onSubmit: (p: SystemAdminAdd) => void;
	image_url1: string;
	image_url2: string;
	edit?: boolean;
	lastUpdatedInput: () => Promise<LastUpdatedInput>;
	initialValues2?: OrganizationStateExtend | undefined;
	initialValues?: SystemAdminStateExtend | undefined;
	country?: string[];
	pincode?: string[];
	state?: string[];
	city?: string[];
	adminCountry?: string[];
	adminState?: string[];
	adminCity?: string[];
	locationData?: LocationRecord[] | undefined;
	pincodeOnChange?: (pincode: string) => void;
};

interface OptionType {
	value: SystemAdminRole;
	label: SystemAdminRole;
}

const options: OptionType[] = [
	{ value: "Patient", label: "Patient" },
	{ value: "Biller", label: "Biller" },
	{ value: "BusinessManager", label: "BusinessManager" },
	{ value: "ClinicalAssistant", label: "ClinicalAssistant" },
	{ value: "Doctor", label: "Doctor" },
	{ value: "SystemAdmin", label: "SystemAdmin" },
	{ value: "OfficeStaff", label: "OfficeStaff" },
];

export const SystemAdminForm = (props: AddSystemAdminFormProps1) => {
	if (props.initialValues === undefined && props.edit === true) {
		return <p>Loading...</p>;
	}
	const [selectedRoles, setSelectedRoles] = useState<SystemAdminRole[]>(
		props.initialValues?.roles || ["SystemAdmin"],
	);
	const handleRoleChange = (newValue: MultiValue<OptionType>) => {
		if (newValue) {
			const selectedRoles: SystemAdminRole[] = newValue.map(
				(option) => option.value as SystemAdminRole,
			);
			setSelectedRoles(selectedRoles);
			setData("roles", selectedRoles);
		} else {
			setSelectedRoles([]);
		}
	};

	const { form, errors, data, setData, isSubmitting, setIsDirty, setTouched } =
		useForm<z.infer<typeof systemAdminAddSchema>>({
			onSubmit: async (values) => {
				const utcDate = convertToUTC(values.user.dob);
				const modifiedValues = {
					...values,
					org_details: {
						...values.org_details,
						city: props.locationData?.map((item) => item)[0]?.district || "",
						country:
							props.locationData?.map((item) => item)[0]?.country_name || "",
						state: props.locationData?.map((item) => item)[0]?.state_name || "",
					},
					last_updated_input: await props.lastUpdatedInput(),
				};
				modifiedValues.user.dob = utcDate;
				console.log(modifiedValues.org_details.city);
				props.onSubmit(modifiedValues);
			},
			initialValues: {
				org_details: {
					name: props.initialValues2?.name || "",
					details: props.initialValues2?.details || "",
					phone_number: props.initialValues2?.phone_number || "",
					email: props.initialValues2?.email || "",
					address: {
						pin_code: props.initialValues2?.address.pin_code || "",
						city: props.initialValues2?.address.city || "",
						state: props.initialValues2?.address.state || "",
						address_line: props.initialValues2?.address.address_line || "",
						country: props.initialValues2?.address.country || "",
					},
				},
				user: {
					first_name: props.initialValues?.user.first_name || "",
					middle_name: props.initialValues?.user.middle_name || "",
					last_name: props.initialValues?.user.last_name || "",
					dob:
						(props.initialValues?.user.dob &&
							dobDate(props?.initialValues?.user.dob)) ||
						"",
					gender: (props.initialValues?.user.gender as GenderType) || "Male",
					email: props.initialValues?.user.email || "",
					photo_url: props.initialValues?.user.photo_url || "https://cdn",
				},
				roles: (props.initialValues?.roles as Array<SystemAdminRole>) || [
					"SystemAdmin",
				],
				phone: {
					number: props.initialValues?.phone.number || "",
					number_type:
						(props.initialValues?.phone.number_type as PhoneNoTypeForContact) ||
						"",
				},
				government_info: {
					id_no: props.initialValues?.government_info.id_no || "",
					id_type:
						(props.initialValues?.government_info.id_type as GovIdType) || "",
				},
				password: "",
				confirm_password: "",
			},
			extend: [validator({ schema: systemAdminAddSchema }), reporterDom()],
		});
	const [showPassword, setShowPassword] = useState(false);

	const [showConfirmPassword, setConfirmShowPassword] = useState(false);

	useEffect(() => {
		setData(
			"org_details.address.city",
			props?.locationData?.map((item) => item)[0]?.district ||
				props.initialValues2?.address.city ||
				"",
		);
		setData(
			"org_details.address.state",
			props?.locationData?.map((item) => item)[0]?.state_name ||
				props.initialValues2?.address.state ||
				"",
		);
		setData(
			"org_details.address.country",
			props?.locationData?.map((item) => item)[0]?.country_name ||
				props.initialValues2?.address.country ||
				"",
		);
	}, [props?.locationData, setData, props?.initialValues2]);

	console.log(data().org_details.address.city, "this is my city");

	return (
		<div>
			<Center>
				<Box width={{ md: "80%", base: "90%", lg: "70%" }}>
					<Box
						mt={{ md: "7%", base: "1%", lg: "1%" }}
						mb={{ md: "3%", base: "1%", lg: "1%" }}
					>
						<HeadingTag label_text="Organization Details" />
					</Box>
					<form ref={form}>
						<FormLabel mb={2} my={3} color={"#095FBA"}>
							{"Organization Name"}
							<span style={{ color: "red" }}>*</span>
						</FormLabel>
						<FormControl
							mt={2}
							isInvalid={(errors().org_details.name || []).length !== 0}
						>
							<Input
								type="text"
								name="org_details.name"
								bgColor="#FFFFFF"
								borderColor="#095FBA"
								placeholder={"Organization Name"}
								maxLength={25}
							/>
							{errors().org_details.name && (
								<FormErrorMessage>{errors().org_details.name}</FormErrorMessage>
							)}
						</FormControl>

						<FormLabel mb={2} my={3} color={"#095FBA"}>
							{"Organization Details"}
							<span style={{ color: "red" }}>*</span>
						</FormLabel>
						<FormControl
							mt={2}
							isInvalid={(errors().org_details.details || []).length !== 0}
						>
							<Textarea
								bgColor={"#FFFFFF"}
								name={"org_details.details"}
								borderColor={"#095FBA"}
								placeholder={"Organization Details"}
								maxLength={100}
							/>
							{errors().org_details.details && (
								<FormErrorMessage>
									{errors().org_details.details}
								</FormErrorMessage>
							)}
						</FormControl>
						<FormLabel mb={2} my={3} color={"#095FBA"}>
							Phone Number<span style={{ color: "red" }}>*</span>
						</FormLabel>
						<FormControl
							mt={2}
							isInvalid={(errors().org_details.phone_number || []).length !== 0}
						>
							<NumberInput
								onChange={(valueAsString) => {
									setData("org_details.phone_number", valueAsString);
									setIsDirty(true);
									setTouched("org_details.phone_number", true);
								}}
								onBlur={() => setTouched("org_details.phone_number", true)}
								borderColor={"#095FBA"}
								value={data().org_details.phone_number}
							>
								<NumberInputField
									placeholder="Phone Number"
									height={10}
									name="org_details.phone_number"
									maxLength={10}
									bgColor={"#FFFFFF"}
									borderColor={"#095FBA"}
								/>
							</NumberInput>

							{errors().org_details.phone_number && (
								<FormErrorMessage>
									{errors().org_details.phone_number}
								</FormErrorMessage>
							)}
						</FormControl>

						<FormLabel mb={2} my={3} color={"#095FBA"}>
							{"Email"}
							<span style={{ color: "red" }}>*</span>
						</FormLabel>
						<FormControl
							mt={2}
							isInvalid={(errors().org_details.email || []).length !== 0}
						>
							<Input
								type="text"
								name="org_details.email"
								bgColor="#FFFFFF"
								borderColor="#095FBA"
								placeholder={"Email"}
							/>
							{errors().org_details.email && (
								<FormErrorMessage>
									{errors().org_details.email}
								</FormErrorMessage>
							)}
						</FormControl>
						<FormLabel mb={2} my={3} color={"#095FBA"}>
							{"Pin Code"}
							<span style={{ color: "red" }}>*</span>
						</FormLabel>
						<FormControl
							mt={2}
							isInvalid={
								(errors().org_details?.address?.pin_code || []).length !== 0
							}
						>
							{/* <NumberInput
								onChange={(valueAsString) => {
									setData("org_details.address.pin_code", valueAsString);
									setIsDirty(true);
									setTouched("org_details.address.pin_code", true);
									// props.pincodeOnChange(valueAsString)
								}}
								onBlur={() => setTouched("org_details.address.pin_code", true)}
								borderColor={"#095FBA"}
								value={data().org_details?.address?.pin_code}
							>
								<NumberInputField
									placeholder="Pin Code"
									height={10}
									name="org_details.address.pin_code"
									maxLength={6}
									bgColor={"#FFFFFF"}
									borderColor={"#095FBA"}
								/>
							</NumberInput> */}
							<Input
								type="text"
								name="org_details.address.pin_code"
								bgColor="#FFFFFF"
								borderColor="#095FBA"
								placeholder={"Pincode"}
								onChange={(e) => props?.pincodeOnChange?.(e.target.value)}
							/>

							{errors().org_details?.address?.pin_code && (
								<FormErrorMessage>
									{errors().org_details?.address?.pin_code}
								</FormErrorMessage>
							)}
						</FormControl>
						<FormLabel mb={2} my={3} color={"#095FBA"}>
							{"Country"}
							<span style={{ color: "red" }}>*</span>
						</FormLabel>
						<FormControl
							mt={2}
							isInvalid={
								(errors().org_details.address.country || []).length !== 0
							}
						>
							<Input
								type="text"
								name="org_details.address.country"
								bgColor="#FFFFFF"
								borderColor="#095FBA"
								placeholder={"Country"}
								isReadOnly={true}
								value={
									props.locationData?.map((item) => item)[0]?.country_name ||
									props.initialValues2?.address.country
								}
							/>

							{errors().org_details.address.country && (
								<FormErrorMessage>
									{errors().org_details.address.country}
								</FormErrorMessage>
							)}
						</FormControl>
						<FormLabel mb={2} my={3} color={"#095FBA"}>
							{"State"}
							<span style={{ color: "red" }}>*</span>
						</FormLabel>
						<FormControl
							mt={2}
							isInvalid={
								(errors().org_details.address.state || []).length !== 0
							}
						>
							<Input
								type="text"
								name="org_details.address.state"
								bgColor="#FFFFFF"
								borderColor="#095FBA"
								placeholder={"State"}
								isReadOnly={true}
								value={
									props.locationData?.map((item) => item)[0]?.state_name ||
									props.initialValues2?.address.state
								}
							/>
							{errors().org_details.address.state && (
								<FormErrorMessage>
									{errors().org_details.address.state}
								</FormErrorMessage>
							)}
						</FormControl>

						<FormLabel mb={2} my={3} color={"#095FBA"}>
							{"City"}
							<span style={{ color: "red" }}>*</span>
						</FormLabel>
						<FormControl
							mt={2}
							isInvalid={(errors().org_details.address.city || []).length !== 0}
						>
							<Input
								type="text"
								name="org_details.address.city"
								bgColor="#FFFFFF"
								borderColor="#095FBA"
								placeholder={"City"}
								isReadOnly={true}
								value={
									props.locationData?.map((item) => item)[0]?.district ||
									props.initialValues2?.address.city
								}
							/>

							{errors().org_details.address.city && (
								<FormErrorMessage>
									{errors().org_details.address.city}
								</FormErrorMessage>
							)}
						</FormControl>

						<FormLabel mb={2} my={3} color={"#095FBA"}>
							{"Address"}
							<span style={{ color: "red" }}>*</span>
						</FormLabel>
						<FormControl
							mt={2}
							mb={2}
							isInvalid={
								(errors().org_details.address.address_line || []).length !== 0
							}
						>
							<Input
								type="text"
								name="org_details.address.address_line"
								bgColor="#FFFFFF"
								borderColor="#095FBA"
								placeholder={"Address"}
								maxLength={100}
							/>
							{errors().org_details.address.address_line && (
								<FormErrorMessage>
									{errors().org_details.address.address_line}
								</FormErrorMessage>
							)}
						</FormControl>

						<Box
							mt={{ md: "7%", base: "7%", lg: "10%" }}
							mb={{ md: "2%", base: "2%", lg: "2%" }}
						>
							<HeadingTag label_text="System Admin Details" />
						</Box>
						<FormLabel mb={2} my={3} color={"#095FBA"}>
							{"First Name"}
							<span style={{ color: "red" }}>*</span>
						</FormLabel>
						<FormControl
							mt={2}
							isInvalid={(errors().user.first_name || []).length !== 0}
						>
							<Input
								type="text"
								name="user.first_name"
								bgColor="#FFFFFF"
								borderColor="#095FBA"
								placeholder={"First Name"}
								maxLength={25}
								onInput={(event) => {
									const target = event.target as HTMLInputElement;
									target.value = target.value.replace(/\s\s+/g, " ");
								}}
							/>
							{errors().user.first_name && (
								<FormErrorMessage>{errors().user.first_name}</FormErrorMessage>
							)}
						</FormControl>

						<FormLabel mb={2} my={3} color={"#095FBA"}>
							{"Middle Name"}
							<span style={{ color: "red" }}>*</span>
						</FormLabel>
						<FormControl
							mt={2}
							isInvalid={(errors().user.middle_name || []).length !== 0}
						>
							<Input
								type="text"
								name="user.middle_name"
								bgColor="#FFFFFF"
								borderColor="#095FBA"
								placeholder={"Middle Name"}
								maxLength={25}
								onInput={(event) => {
									const target = event.target as HTMLInputElement;
									target.value = target.value.replace(/\s\s+/g, " ");
								}}
							/>
							{errors().user.middle_name && (
								<FormErrorMessage>{errors().user.middle_name}</FormErrorMessage>
							)}
						</FormControl>

						<FormLabel mb={2} my={3} color={"#095FBA"}>
							{"Last Name"}
							<span style={{ color: "red" }}>*</span>
						</FormLabel>
						<FormControl
							mt={2}
							isInvalid={(errors().user.last_name || []).length !== 0}
						>
							<Input
								type="text"
								name="user.last_name"
								bgColor="#FFFFFF"
								borderColor="#095FBA"
								placeholder={"Last Name"}
								maxLength={25}
								onInput={(event) => {
									const target = event.target as HTMLInputElement;
									target.value = target.value.replace(/\s\s+/g, " ");
								}}
							/>
							{errors().user.last_name && (
								<FormErrorMessage>{errors().user.last_name}</FormErrorMessage>
							)}
						</FormControl>

						<FormLabel mb={2} my={3} color={"#095FBA"}>
							{"Date of Birth"}
							<span style={{ color: "red" }}>*</span>
						</FormLabel>
						<FormControl
							mt={2}
							isInvalid={(errors().user.dob || []).length !== 0}
						>
							<DateOfBirth name={"user.dob"} />
							{errors().user.dob && (
								<FormErrorMessage>{errors().user.dob}</FormErrorMessage>
							)}
						</FormControl>

						<FormLabel mb={2} my={3} color={"#095FBA"}>
							{"Email"}
							<span style={{ color: "red" }}>*</span>
						</FormLabel>
						<FormControl
							mt={2}
							isInvalid={(errors().user.email || []).length !== 0}
						>
							<Input
								type="text"
								name="user.email"
								bgColor="#FFFFFF"
								borderColor="#095FBA"
								placeholder={"Email"}
							/>
							{errors().user.email && (
								<FormErrorMessage>{errors().user.email}</FormErrorMessage>
							)}
						</FormControl>

						<FormLabel mb={2} my={3} color={"#095FBA"}>
							{"Gender"}
							<span style={{ color: "red" }}>*</span>
						</FormLabel>
						<FormControl
							mt={2}
							isInvalid={(errors().user.gender || []).length !== 0}
						>
							<SelectGenderType name="user.gender" />
							{errors().user.gender && (
								<FormErrorMessage>{errors().user.gender}</FormErrorMessage>
							)}
						</FormControl>

						<FormLabel mb={2} my={3} color={"#095FBA"}>
							{"Add Photo"}
							<span style={{ color: "red" }}>*</span>
						</FormLabel>
						<FormControl
							mt={2}
							isInvalid={(errors().user.photo_url || []).length !== 0}
						>
							<PhotoUrlInput name="user.photo_url" />
							{errors().user.photo_url && (
								<FormErrorMessage>{errors().user.photo_url}</FormErrorMessage>
							)}
						</FormControl>

						<FormLabel mb={2} my={3} color={"#095FBA"} hidden={true}>
							Roles<span style={{ color: "red" }}>*</span>
						</FormLabel>
						<FormControl
							mt={2}
							isInvalid={(errors().roles || []).length !== 0}
							hidden={true}
						>
							<ReactSelect
								name="roles"
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

							{errors().roles && (
								<FormErrorMessage>{errors().roles}</FormErrorMessage>
							)}
						</FormControl>
						<FormLabel mb={2} my={3} color={"#095FBA"}>
							{"Phone Number Type"}
							<span style={{ color: "red" }}>*</span>
						</FormLabel>
						<FormControl
							mt={2}
							isInvalid={(errors().phone.number_type || []).length !== 0}
						>
							<SelectPhoneNumberType name="phone.number_type" />
							{errors().phone.number_type && (
								<FormErrorMessage>
									{errors().phone.number_type}
								</FormErrorMessage>
							)}
						</FormControl>

						<FormLabel mb={2} my={3} color={"#095FBA"}>
							{"Phone Number"}
							<span style={{ color: "red" }}>*</span>
						</FormLabel>
						<FormControl
							mt={2}
							isInvalid={(errors().phone?.number || []).length !== 0}
						>
							<NumberInput
								onChange={(valueAsString) => {
									setData("phone.number", valueAsString);
									setIsDirty(true);
									setTouched("phone.number", true);
								}}
								onBlur={() => setTouched("phone.number", true)}
								borderColor={"#095FBA"}
								value={data().phone.number}
							>
								<NumberInputField
									placeholder="Phone Number"
									height={10}
									name="phone.number"
									maxLength={10}
									bgColor={"#FFFFFF"}
									borderColor={"#095FBA"}
								/>
							</NumberInput>

							{errors().phone?.number && (
								<FormErrorMessage>{errors().phone?.number}</FormErrorMessage>
							)}
						</FormControl>
						<FormLabel mb={2} my={3} color={"#095FBA"}>
							{"Government Id Type"}
							<span style={{ color: "red" }}>*</span>
						</FormLabel>
						<FormControl
							mt={2}
							isInvalid={(errors().government_info.id_type || []).length !== 0}
						>
							<SelectGovernmentIdType name="government_info.id_type" />
							{errors().government_info.id_type && (
								<FormErrorMessage>
									{errors().government_info.id_type}
								</FormErrorMessage>
							)}
						</FormControl>

						<FormLabel mb={2} my={3} color={"#095FBA"}>
							{"Government Id"}
							<span style={{ color: "red" }}>*</span>
						</FormLabel>
						<FormControl
							mt={2}
							isInvalid={(errors().government_info.id_no || []).length !== 0}
						>
							<Input
								type="text"
								name="government_info.id_no"
								bgColor="#FFFFFF"
								borderColor="#095FBA"
								placeholder={"Government Id"}
							/>
							{errors().government_info.id_no && (
								<FormErrorMessage>
									{errors().government_info.id_no}
								</FormErrorMessage>
							)}
						</FormControl>

						<FormLabel mb={2} my={3} color={"#095FBA"}>
							Password<span style={{ color: "red" }}>*</span>
						</FormLabel>

						<InputGroup alignItems={"center"}>
							<InputRightElement height="47px" mr={2}>
								<Image
									src={showPassword ? props.image_url1 : props.image_url2}
									onClick={() => setShowPassword(!showPassword)}
									width={25}
									height={20}
								/>
							</InputRightElement>
							<Input
								borderColor={"#095FBA"}
								placeholder="Password"
								bgColor={"#FFFFFF"}
								type={showPassword ? "text" : "password"}
								name="password"
							/>
						</InputGroup>
						<Text color={"red"} fontSize="sm">
							{errors().password}
						</Text>

						<FormLabel mb={2} my={3} color={"#095FBA"}>
							Confirm Password<span style={{ color: "red" }}>*</span>
						</FormLabel>
						<InputGroup alignItems={"center"}>
							<InputRightElement height="47px" mr={2}>
								<Image
									src={
										showConfirmPassword ? props.image_url1 : props.image_url2
									}
									onClick={() => setConfirmShowPassword(!showConfirmPassword)}
									width={25}
									height={20}
								/>
							</InputRightElement>
							<Input
								borderColor={"#095FBA"}
								placeholder="Confirm Password"
								bgColor={"#FFFFFF"}
								type={showConfirmPassword ? "text" : "password"}
								name="confirm_password"
							/>
						</InputGroup>
						<Text color={"red"} fontSize="sm">
							{errors().confirm_password}
						</Text>
						<SubmitButton loading={isSubmitting()} />
					</form>
				</Box>
			</Center>
		</div>
	);
};
