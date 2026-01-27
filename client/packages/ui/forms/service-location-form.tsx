import {
	FormLabel,
	FormControl,
	Input,
	Box,
	// Select,
	FormErrorMessage,
	Center,
} from "@chakra-ui/react";

import { useForm } from "@felte/react";
import type {
	LastUpdatedInput,
	LocationRecord,
	ServiceLocationAdd,
} from "@repo/types/dto";
import { serviceLocationInputSchema } from "@repo/types/validation";
import type { z } from "zod";
import { SubmitButton } from "../component/submit-button";
import type { ServiceLocationStateExtend } from "@repo/types/dexie-state";
import { validator } from "@felte/validator-zod";
import reporterDom from "@felte/reporter-dom";
import { useEffect } from "react";

export type ServiceLocationDetailsProps = {
	onSubmit: (p: ServiceLocationAdd) => void;
	servicelocationId?: string;
	initialValues?: ServiceLocationStateExtend | undefined;
	lastUpdatedInput: () => Promise<LastUpdatedInput>;
	edit?: boolean;
	country?: string[];
	city?: string[];
	state?: string[];
	locationData?: LocationRecord[] | undefined;
	pincodeOnChange?: (pincode: string) => void;
};

export const ServiceLocationForm = (props: ServiceLocationDetailsProps) => {
	if (props.initialValues === undefined && props.edit === true) {
		return <p>Loading...</p>;
	}

	const { form, errors, isSubmitting, setData } = useForm<
		z.infer<typeof serviceLocationInputSchema>
	>({
		onSubmit: async (values) => {
			const newval = {
				...values,
				last_updated_input: await props.lastUpdatedInput(),
			};

			props.onSubmit(newval);
		},
		initialValues: {
			service_location_name: props.initialValues?.service_location_name || "",
			address: {
				pin_code: props.initialValues?.address.pin_code || "",
				city: props.initialValues?.address.city || "",
				state: props.initialValues?.address.state || "",
				address_line: props.initialValues?.address.address_line || "",
				country: props.initialValues?.address.country || "",
			},
			start_time: props.initialValues?.start_time,
			end_time: props.initialValues?.end_time,
		},
		extend: [validator({ schema: serviceLocationInputSchema }), reporterDom()],
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
					<FormLabel mb={2} color={"#095FBA"} my={3}>
						Name
						<span style={{ color: "red" }}>*</span>
					</FormLabel>
					<FormControl
						mt={2}
						width={"100%"}
						isInvalid={(errors().service_location_name || []).length !== 0}
					>
						<Input
							placeholder="Service Location Name"
							_placeholder={{ color: "#717B9E" }}
							name="service_location_name"
							bgColor="#FFFFFF"
							borderColor="#095FBA"
							maxLength={25}
						/>
						{errors().service_location_name && (
							<FormErrorMessage>
								{errors().service_location_name}
							</FormErrorMessage>
						)}
					</FormControl>

					<FormLabel mb={2} my={3} color={"#095FBA"}>
						{"Pin Code"}
						<span style={{ color: "red" }}>*</span>
					</FormLabel>
					<FormControl
						mt={2}
						isInvalid={(errors()?.address?.pin_code || []).length !== 0}
					>
						{/* <NumberInput
							onChange={(valueAsString) => {
								setData("address.pin_code", valueAsString);
								setIsDirty(true);
								setTouched("address.pin_code", true);
							}}
							onBlur={() => setTouched("address.pin_code", true)}
							borderColor={"#095FBA"}
							value={data().address?.pin_code}
						>
							<NumberInputField
								placeholder="Pin Code"
								height={10}
								name="address.pin_code"
								maxLength={6}
								bgColor={"#FFFFFF"}
								borderColor={"#095FBA"}
								onChange={() => {
									updatePincode(data()?.address?.pin_code);
								}}
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
							<FormErrorMessage>{errors()?.address?.pin_code}</FormErrorMessage>
						)}
					</FormControl>
					<FormLabel mb={2} my={3} color={"#095FBA"}>
						{"Country"}
						<span style={{ color: "red" }}>*</span>
					</FormLabel>
					<FormControl
						mt={2}
						isInvalid={(errors()?.address?.country || []).length !== 0}
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
						{"State"}
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
						{"Area"}
						<span style={{ color: "red" }}>*</span>
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
						{"Address Line"}
						<span style={{ color: "red" }}>*</span>
					</FormLabel>
					<FormControl
						mt={2}
						isInvalid={(errors().address?.address_line || []).length !== 0}
					>
						<Input
							type="text"
							name="address.address_line"
							bgColor="#FFFFFF"
							borderColor="#095FBA"
							placeholder={"Address Line"}
						/>
						{errors().address?.address_line && (
							<FormErrorMessage>
								{errors().address?.address_line}
							</FormErrorMessage>
						)}
					</FormControl>
					<FormLabel mb={2} my={3} color={"#095FBA"}>
						{"Start Time"}
						<span style={{ color: "red" }}>*</span>
					</FormLabel>
					<FormControl
						mt={2}
						isInvalid={(errors().start_time || []).length !== 0}
					>
						<Input
							type="time"
							name="start_time"
							step="1"
							bgColor="#FFFFFF"
							borderColor="#095FBA"
							placeholder={"Start Time"}
						/>
						{errors().start_time && (
							<FormErrorMessage>{errors().start_time}</FormErrorMessage>
						)}
					</FormControl>
					<FormLabel mb={2} my={3} color={"#095FBA"}>
						{"End Time"}
						<span style={{ color: "red" }}>*</span>
					</FormLabel>
					<FormControl
						mt={2}
						isInvalid={(errors().end_time || []).length !== 0}
					>
						<Input
							type="time"
							name="end_time"
							step="1"
							bgColor="#FFFFFF"
							borderColor="#095FBA"
							placeholder={"End Time"}
							//min={"start_time"}
						/>
						{errors().end_time && (
							<FormErrorMessage>{errors().end_time}</FormErrorMessage>
						)}
					</FormControl>

					<SubmitButton loading={isSubmitting()} />
				</form>
			</Box>
		</Center>
	);
};
