import { useForm } from "@felte/react";
import type { z } from "zod";
import { validator } from "@felte/validator-zod";
import reporterDom from "@felte/reporter-dom";
import { convertToUTC } from "../component/convert-to-utc";
import {
	FormLabel,
	FormControl,
	Input,
	FormErrorMessage,
	Box,
	Center,
	NumberInput,
	NumberInputField,
} from "@chakra-ui/react";
import { SubmitButton } from "../component/submit-button";
import { patientInputSchema } from "@repo/types/validation";
import type {
	GenderType,
	GovIdType,
	LastUpdatedInput,
	LocationRecord,
	PatientAdd,
	PhoneNoTypeForContact,
} from "@repo/types/dto";
import type { PatientStateExtend } from "@repo/types/dexie-state";
import { dobDate } from "../component/utc-date-to-normal-date";
import SelectGovernmentIdType from "../component/government-id-type";
import SelectGenderType from "../component/gender-types";
import SelectPhoneNumberType from "../component/phone-number-type";
import PhotoUrlInput from "../component/photo-url-input";
import DateOfBirth from "../component/date-of-birth";
import { useEffect } from "react";

export type AddPatientFormProps = {
	onSubmit: (p: PatientAdd) => void;
	patientId?: string;
	edit?: boolean;
	initialValues?: PatientStateExtend | undefined;
	lastUpdatedInput: () => Promise<LastUpdatedInput>;
	locationData?: LocationRecord[] | undefined;
	pincodeOnChange?: (pincode: string) => void;
};
export const PatientForm = (props: AddPatientFormProps) => {
	if (props.initialValues === undefined && props.edit === true) {
		return <p>Loading...</p>;
	}

	const { form, errors, data, isSubmitting, setData, setIsDirty, setTouched } =
		useForm<z.infer<typeof patientInputSchema>>({
			onSubmit: async (values) => {
				const utcDate = convertToUTC(values.user.dob);

				const modifiedValues = {
					...values,
					last_updated_input: await props.lastUpdatedInput(),
				};
				modifiedValues.user.dob = utcDate;
				props.onSubmit(modifiedValues);
			},
			initialValues: {
				user: {
					first_name: props.initialValues?.user.first_name || "",
					middle_name: props.initialValues?.user.middle_name || "",
					last_name: props.initialValues?.user.last_name || "",
					dob:
						props.initialValues?.user.dob &&
						dobDate(props.initialValues.user.dob),
					email: props.initialValues?.user.email,
					gender: (props.initialValues?.user.gender as GenderType) || "Female",
					photo_url: props.initialValues?.user?.photo_url || "https://abc",
				},
				address: {
					pin_code: props.initialValues?.address.pin_code || "",
					city: props.initialValues?.address.city || "",
					state: props.initialValues?.address.state || "",
					address_line: props.initialValues?.address.address_line || "",
					country: props.initialValues?.address.country || "",
				},
				phone: {
					number: props.initialValues?.phone.number || "",
					number_type: props.initialValues?.phone
						.number_type as PhoneNoTypeForContact,
				},
				government_info: {
					id_no: props.initialValues?.government_info.id_no || "",
					id_type: props.initialValues?.government_info?.id_type as GovIdType,
				},
			},
			extend: [validator({ schema: patientInputSchema }), reporterDom()],
		});
	useEffect(() => {
		setData(
			"address.city",
			props?.locationData?.map((item) => item)[0]?.district ||
				props.initialValues?.address.city ||
				"",
		);
		setData(
			"address.state",
			props?.locationData?.map((item) => item)[0]?.state_name ||
				props.initialValues?.address.state ||
				"",
		);
		setData(
			"address.country",
			props?.locationData?.map((item) => item)[0]?.country_name ||
				props.initialValues?.address.country ||
				"",
		);
	}, [props?.locationData, setData, props?.initialValues]);
	return (
		<Center>
			<Box width={{ md: "80%", base: "90%", lg: "70%" }}>
				<form ref={form}>
					<FormLabel mb={2} my={3} color={"#095FBA"}>
						First Name
						<span style={{ color: "red" }}>*</span>
					</FormLabel>
					<FormControl
						mt={2}
						isInvalid={(errors().user?.first_name || []).length !== 0}
					>
						<Input
							type="text"
							name="user.first_name"
							bgColor={"#FFFFFF"}
							borderColor={"#095FBA"}
							placeholder="First Name"
							maxLength={25}
							onInput={(event) => {
								const target = event.target as HTMLInputElement;
								target.value = target.value.replace(/\s\s+/g, " ");
							}}
						/>
						{errors().user?.first_name && (
							<FormErrorMessage>{errors().user?.first_name}</FormErrorMessage>
						)}
					</FormControl>

					<FormLabel mb={2} my={3} color={"#095FBA"}>
						Middle Name
						<span style={{ color: "red" }}>*</span>
					</FormLabel>
					<FormControl
						mt={2}
						isInvalid={(errors().user?.middle_name || []).length !== 0}
					>
						<Input
							type="text"
							name="user.middle_name"
							bgColor={"#FFFFFF"}
							borderColor={"#095FBA"}
							placeholder="Middle Name"
							maxLength={25}
							onInput={(event) => {
								const target = event.target as HTMLInputElement;
								target.value = target.value.replace(/\s\s+/g, " ");
							}}
						/>
						{errors().user?.middle_name && (
							<FormErrorMessage>{errors().user?.middle_name}</FormErrorMessage>
						)}
					</FormControl>

					<FormLabel mb={2} my={3} color={"#095FBA"}>
						Last Name
						<span style={{ color: "red" }}>*</span>
					</FormLabel>
					<FormControl
						mt={2}
						isInvalid={(errors().user?.last_name || []).length !== 0}
					>
						<Input
							type="text"
							name="user.last_name"
							bgColor={"#FFFFFF"}
							borderColor={"#095FBA"}
							placeholder="Last Name"
							maxLength={25}
							onInput={(event) => {
								const target = event.target as HTMLInputElement;
								target.value = target.value.replace(/\s\s+/g, " ");
							}}
						/>
						{errors().user?.last_name && (
							<FormErrorMessage>{errors().user?.last_name}</FormErrorMessage>
						)}
					</FormControl>

					<FormLabel mb={2} my={3} color={"#095FBA"}>
						Date of Birth
						<span style={{ color: "red" }}>*</span>
					</FormLabel>
					<FormControl
						mt={2}
						isInvalid={(errors().user?.dob || []).length !== 0}
					>
						<DateOfBirth name={"user.dob"} />
						{errors().user?.dob && (
							<FormErrorMessage>{errors().user?.dob}</FormErrorMessage>
						)}
					</FormControl>

					<FormLabel mb={2} my={3} color={"#095FBA"}>
						Email
						<span style={{ color: "red" }}>*</span>
					</FormLabel>
					<FormControl
						mt={2}
						isInvalid={(errors().user?.email || []).length !== 0}
					>
						<Input
							type="email"
							name="user.email"
							bgColor={"#FFFFFF"}
							borderColor={"#095FBA"}
							placeholder="Email"
						/>
						{errors().user?.email && (
							<FormErrorMessage>{errors().user?.email}</FormErrorMessage>
						)}
					</FormControl>

					<FormLabel mb={2} my={3} color={"#095FBA"}>
						Add Photo
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

					<FormLabel mb={2} my={3} color={"#095FBA"}>
						Select Gender
						<span style={{ color: "red" }}>*</span>
					</FormLabel>
					<FormControl
						mt={2}
						isInvalid={(errors().user?.gender || []).length !== 0}
					>
						<SelectGenderType name={"user.gender"} />
						{errors().user?.gender && (
							<FormErrorMessage>{errors().user?.gender}</FormErrorMessage>
						)}
					</FormControl>
					<FormLabel mb={2} my={3} color={"#095FBA"}>
						Pin Code
						<span style={{ color: "red" }}>*</span>
					</FormLabel>
					<FormControl
						mt={2}
						isInvalid={(errors().address?.pin_code || []).length !== 0}
					>
						{/* <NumberInput
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
								// onChange={() => {
								//   updatePincode(data().address.pin_code);
								// }}
							/>
						</NumberInput> */}
						<Input
							type="text"
							name="address.pin_code"
							bgColor="#FFFFFF"
							borderColor="#095FBA"
							placeholder={"Pincode"}
							onChange={(e) => props?.pincodeOnChange?.(e.target.value)}
						/>

						{errors().address?.pin_code && (
							<FormErrorMessage>{errors().address?.pin_code}</FormErrorMessage>
						)}
					</FormControl>

					<FormLabel mb={2} my={3} color={"#095FBA"}>
						Country
						<span style={{ color: "red" }}>*</span>
					</FormLabel>
					<FormControl
						mt={2}
						isInvalid={(errors().address?.country || []).length !== 0}
					>
						<Input
							type="text"
							name="address.country"
							bgColor="#FFFFFF"
							borderColor="#095FBA"
							placeholder={"Country"}
							value={
								props.locationData?.map((item) => item)[0]?.country_name ||
								props.initialValues?.address.country
							}
						/>
						{errors().address?.country && (
							<FormErrorMessage>{errors().address?.country}</FormErrorMessage>
						)}
					</FormControl>
					<FormLabel mb={2} my={3} color={"#095FBA"}>
						State
						<span style={{ color: "red" }}>*</span>
					</FormLabel>
					<FormControl
						mt={2}
						isInvalid={(errors().address?.state || []).length !== 0}
					>
						<Input
							type="text"
							name="address.state"
							bgColor="#FFFFFF"
							borderColor="#095FBA"
							placeholder={"State"}
							value={
								props.locationData?.map((item) => item)[0]?.state_name ||
								props.initialValues?.address.state
							}
						/>
						{errors().address?.state && (
							<FormErrorMessage>{errors().address?.state}</FormErrorMessage>
						)}
					</FormControl>
					<FormLabel mb={2} my={3} color={"#095FBA"}>
						Area
						<span style={{ color: "red" }}>*</span>
					</FormLabel>
					<FormControl
						mt={2}
						isInvalid={(errors().address?.city || []).length !== 0}
					>
						<Input
							placeholder="Area"
							name="address.city"
							bgColor="#FFFFFF"
							borderColor="#095FBA"
							value={
								props.locationData?.map((item) => item)[0]?.district ||
								props.initialValues?.address.city
							}
						/>
						{errors().address?.city && (
							<FormErrorMessage>{errors().address?.city}</FormErrorMessage>
						)}
					</FormControl>

					<FormLabel mb={2} my={3} color={"#095FBA"}>
						Address
						<span style={{ color: "red" }}>*</span>
					</FormLabel>
					<FormControl
						mt={2}
						isInvalid={(errors().address?.address_line || []).length !== 0}
					>
						<Input
							type="text"
							name="address.address_line"
							bgColor={"#FFFFFF"}
							borderColor={"#095FBA"}
							placeholder="Address"
							maxLength={100}
						/>
						{errors().address?.address_line && (
							<FormErrorMessage>
								{errors().address?.address_line}
							</FormErrorMessage>
						)}
					</FormControl>

					<FormLabel mb={2} my={3} color={"#095FBA"}>
						Select Phone Number Type
						<span style={{ color: "red" }}>*</span>
					</FormLabel>
					<FormControl
						mt={2}
						isInvalid={(errors().phone?.number_type || []).length !== 0}
					>
						{/* <Select
							name="phone.number_type"
							bgColor="#FFFFFF"
							borderColor="#095FBA"
						>
							<option hidden disabled value="">
								Select Phone Number Type
							</option>
							<option value="Mobile">Mobile</option>
							<option value="Home">Home</option>
							<option value="Office">Office</option>
						</Select> */}
						<SelectPhoneNumberType name={"phone.number_type"} />
						{errors().phone?.number_type && (
							<FormErrorMessage>{errors().phone?.number_type}</FormErrorMessage>
						)}
					</FormControl>
					<FormLabel mb={2} my={3} color={"#095FBA"}>
						Phone Number
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
						Government Id Type
						<span style={{ color: "red" }}>*</span>
					</FormLabel>
					<FormControl
						mt={2}
						isInvalid={(errors().government_info.id_type || []).length !== 0}
					>
						{/* <Select
							name="government_info.id_type"
							bgColor="#FFFFFF"
							borderColor="#095FBA"
						>
							<option hidden disabled value="">
								Select Government ID Type
							</option>
							<option value="AadhaarCard">AadhaarCard</option>
							<option value="DrivingLicense">DrivingLicense</option>
							<option value="Passport">Passport</option>
						</Select> */}
						<SelectGovernmentIdType name={"government_info.id_type"} />
						{errors().government_info.id_type && (
							<FormErrorMessage>
								{errors().government_info.id_type}
							</FormErrorMessage>
						)}
					</FormControl>

					<FormLabel mb={2} my={3} color={"#095FBA"}>
						Government Id
						<span style={{ color: "red" }}>*</span>
					</FormLabel>
					<FormControl
						mt={2}
						isInvalid={(errors().government_info?.id_no || []).length !== 0}
					>
						<Input
							type="text"
							name="government_info.id_no"
							bgColor={"#FFFFFF"}
							borderColor={"#095FBA"}
							placeholder="Government Id"
						/>
						{errors().government_info?.id_no && (
							<FormErrorMessage>
								{errors().government_info?.id_no}
							</FormErrorMessage>
						)}
					</FormControl>
					<SubmitButton loading={isSubmitting()} />
				</form>
			</Box>
		</Center>
	);
};
