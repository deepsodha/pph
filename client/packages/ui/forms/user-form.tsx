import type {
	GenderType,
	GovIdType,
	LastUpdatedInput,
	LocationRecord,
	PhoneNoTypeForContact,
	UserAdd,
	UserUpdate,
} from "@repo/types/dto";
import reporterDom from "@felte/reporter-dom";
import { UserInputSchema, UserUpdateSchema } from "@repo/types/validation";
import { convertToUTC } from "../component/convert-to-utc";
import { useForm } from "@felte/react";
import type { z } from "zod";
import { validator } from "@felte/validator-zod";
import { useEffect, useState } from "react";
import {
	Box,
	Center,
	FormControl,
	FormErrorMessage,
	FormLabel,
	Input,
	Text,
	Image,
	NumberInput,
	NumberInputField,
	InputGroup,
	InputRightElement,
} from "@chakra-ui/react";
import { SubmitButton } from "../component";
import type { UserStateExtend } from "@repo/types/dexie-state";
import { dobDate } from "../component/utc-date-to-normal-date";
import PhotoUrlInput from "../component/photo-url-input";
import DateOfBirth from "../component/date-of-birth";
import SelectGovernmentIdType from "../component/government-id-type";
import SelectGenderType from "../component/gender-types";
import SelectPhoneNumberType from "../component/phone-number-type";
export type AdduserProps = {
	onSubmit: (p: UserAdd | UserUpdate) => void;
	lastUpdatedInput: () => Promise<LastUpdatedInput>;
	UserId?: string;
	edit?: boolean;
	image_url1: string;
	image_url2: string;
	initialValues?: UserStateExtend | undefined;
	locationData?: LocationRecord[] | undefined;
	pincodeOnChange?: (pincode: string) => void;
};
export const UserForm = (props: AdduserProps) => {
	if (props.initialValues === undefined && props.edit === true) {
		return <p>Loading...</p>;
	}

	const { form, errors, isSubmitting, data, setData, setIsDirty, setTouched } =
		useForm<z.infer<typeof UserInputSchema>>({
			onSubmit: async (values) => {
				const utcDate = convertToUTC(values.user.dob);
				const modifiedValues: UserAdd = {
					user: {
						first_name: values.user.first_name || "",
						middle_name: values.user?.middle_name || "",
						last_name: values.user?.last_name || "",
						dob: values.user?.dob && dobDate(values.user.dob),
						email: values.user?.email,
						gender: (values.user?.gender as GenderType) || "Male",
						photo_url: values.user?.photo_url || "https://abc",
					},
					phone: {
						number: values.phone?.number || "",
						number_type:
							(values.phone?.number_type as PhoneNoTypeForContact) || "",
					},
					address: {
						pin_code: values.address?.pin_code || "",
						city: values.address?.city || "",
						state: values.address?.state || "",
						address_line: values.address?.address_line || "",
						country: values.address?.country || "",
					},
					goverment_info: {
						id_no: values.goverment_info?.id_no || "",
						id_type: (values.goverment_info.id_type as GovIdType) || "",
					},
					password: values.password,
					confirm_password: values.confirm_password,
					last_updated_input: await props.lastUpdatedInput(),
				};
				const modifiedEdit: UserUpdate = {
					...modifiedValues,
					id: props.UserId || "",
				};
				modifiedValues.user.dob = utcDate;
				modifiedEdit.user.dob = utcDate;
				props.onSubmit(props.edit ? modifiedEdit : modifiedValues);
			},
			initialValues: {
				user: {
					first_name: props.initialValues?.user.first_name || "",
					middle_name: props.initialValues?.user?.middle_name || "",
					last_name: props.initialValues?.user?.last_name || "",
					dob:
						props.initialValues?.user?.dob &&
						dobDate(props.initialValues.user.dob),
					email: props.initialValues?.user?.email,
					gender: (props.initialValues?.user?.gender as GenderType) || "Male",
					photo_url: props.initialValues?.user?.photo_url || "https://abc",
				},
				phone: {
					number: props.initialValues?.phone?.number || "",
					number_type:
						(props.initialValues?.phone
							?.number_type as PhoneNoTypeForContact) || "",
				},
				address: {
					pin_code: props.initialValues?.address?.pin_code || "",
					city: props.initialValues?.address?.city || "",
					state: props.initialValues?.address?.state || "",
					address_line: props.initialValues?.address?.address_line || "",
					country: props.initialValues?.address?.country || "",
				},
				goverment_info: {
					id_no: props.initialValues?.government_info?.id_no || "",
					id_type:
						(props.initialValues?.government_info?.id_type as GovIdType) || "",
				},
			},

			extend: [
				validator({ schema: props.edit ? UserUpdateSchema : UserInputSchema }),
				reporterDom(),
			],
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

	const [showPassword, setShowPassword] = useState(false);
	const [showConfirmPassword, setConfirmShowPassword] = useState(false);
	return (
		<Center>
			<Box width={{ md: "80%", base: "90%", lg: "70%" }}>
				<form ref={form}>
					<FormLabel mb={2} my={3} color={"#095FBA"}>
						First Name
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
					</FormLabel>
					<FormControl
						mt={2}
						isInvalid={(errors().user?.middle_name || []).length !== 0}
					>
						<Input
							type="text"
							onInput={(event) => {
								const target = event.target as HTMLInputElement;
								target.value = target.value.replace(/\s\s+/g, " ");
							}}
							name="user.middle_name"
							bgColor={"#FFFFFF"}
							borderColor={"#095FBA"}
							placeholder="Middle Name"
							maxLength={25}
						/>
						{errors().user?.middle_name && (
							<FormErrorMessage>{errors().user?.middle_name}</FormErrorMessage>
						)}
					</FormControl>

					<FormLabel mb={2} my={3} color={"#095FBA"}>
						Last Name
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
						Gender
					</FormLabel>
					<FormControl
						mt={2}
						isInvalid={(errors().user?.gender || []).length !== 0}
					>
						<SelectGenderType name="user.gender" />
						{errors().user?.gender && (
							<FormErrorMessage>{errors().user?.gender}</FormErrorMessage>
						)}
					</FormControl>

					<FormLabel mb={2} my={3} color={"#095FBA"}>
						Add Photo
					</FormLabel>
					<FormControl
						mt={2}
						isInvalid={(errors().user?.photo_url || []).length !== 0}
					>
						<PhotoUrlInput name="user.photo_url" />
						{errors().user?.photo_url && (
							<FormErrorMessage>{errors().user?.photo_url}</FormErrorMessage>
						)}
					</FormControl>
					<FormLabel mb={2} my={3} color={"#095FBA"}>
						Pin Code
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
							// 	updatePincode(data().address?.pin_code);
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
							isReadOnly={true}
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
							isReadOnly={true}
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
						City
					</FormLabel>
					<FormControl
						mt={2}
						isInvalid={(errors().address?.city || []).length !== 0}
					>
						<Input
							type="text"
							name="address.city"
							bgColor="#FFFFFF"
							borderColor="#095FBA"
							placeholder={"City"}
							isReadOnly={true}
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
						Phone Number Type
					</FormLabel>
					<FormControl
						mt={2}
						isInvalid={(errors().phone?.number_type || []).length !== 0}
					>
						<SelectPhoneNumberType name="phone.number_type" />
						{errors().phone?.number_type && (
							<FormErrorMessage>{errors().phone?.number_type}</FormErrorMessage>
						)}
					</FormControl>

					<FormLabel mb={2} my={3} color={"#095FBA"}>
						Phone Number
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
					</FormLabel>
					<FormControl
						mt={2}
						isInvalid={(errors().goverment_info.id_type || []).length !== 0}
					>
						<SelectGovernmentIdType name="goverment_info.id_type" />
						{errors().goverment_info.id_type && (
							<FormErrorMessage>
								{errors().goverment_info.id_type}
							</FormErrorMessage>
						)}
					</FormControl>

					<FormLabel mb={2} my={3} color={"#095FBA"}>
						Goverment ID
					</FormLabel>
					<FormControl
						mt={2}
						isInvalid={(errors().goverment_info?.id_no || []).length !== 0}
					>
						<Input
							type="text"
							name="goverment_info.id_no"
							bgColor={"#FFFFFF"}
							borderColor={"#095FBA"}
							placeholder="Goverment ID"
						/>
						{errors().goverment_info?.id_no && (
							<FormErrorMessage>
								{errors().goverment_info?.id_no}
							</FormErrorMessage>
						)}
					</FormControl>
					{!props.edit && (
						<>
							<FormLabel mb={2} my={3} color={"#095FBA"}>
								Password
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
								Confirm Password
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
						</>
					)}

					<SubmitButton loading={isSubmitting()} />
				</form>
			</Box>
		</Center>
	);
};
