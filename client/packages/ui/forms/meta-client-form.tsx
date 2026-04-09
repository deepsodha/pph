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
import { validator } from "@felte/validator-zod";
import reporterDom from "@felte/reporter-dom";
import type { z } from "zod";
import { clientaddSchema } from "@repo/types/validation";
import type { Clients } from "@repo/types/dto";

export type ClientFormProps = {
	onSubmit: (p: Clients) => void;
	ClientId?: string;
	edit?: boolean;
	initialValues?: Clients | undefined;
};

export const MetaClientForm = (props: ClientFormProps) => {
	const { form, errors, isSubmitting, setData, setTouched, data, setIsDirty } =
		useForm<z.infer<typeof clientaddSchema>>({
			onSubmit: (values) => {
				props.onSubmit(values);
			},
			initialValues: {
				name: props.initialValues?.name,
				address: props.initialValues?.address,
				mobile_number: props.initialValues?.mobile_number,
				email: props.initialValues?.email,
				gst_no: props.initialValues?.gst_no,
				pan_no: props.initialValues?.pan_no,
			},
			extend: [validator({ schema: clientaddSchema }), reporterDom()],
		});

	return (
		<div>
			<form ref={form}>
				<Center>
					<Box width={{ md: "80%", base: "90%", lg: "70%" }}>
						<FormLabel mb={2} my={3} color={"#095FBA"}>
							Name
						</FormLabel>
						<FormControl mt={2} isInvalid={(errors().name || []).length !== 0}>
							<Input
								type="text"
								name="name"
								bgColor={"#FFFFFF"}
								borderColor={"#095FBA"}
								placeholder="Enter Name"
							/>
							{errors().name && (
								<FormErrorMessage>{errors().name}</FormErrorMessage>
							)}
						</FormControl>
						<FormLabel mb={2} my={3} color={"#095FBA"}>
							Address
						</FormLabel>
						<FormControl
							mt={2}
							isInvalid={(errors().address || []).length !== 0}
						>
							<Input
								type="text"
								name="address"
								bgColor={"#FFFFFF"}
								borderColor={"#095FBA"}
								placeholder="Enter Address"
							/>
							{errors().address && (
								<FormErrorMessage>{errors().address}</FormErrorMessage>
							)}
						</FormControl>
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
							Email
						</FormLabel>
						<FormControl mt={2} isInvalid={(errors().email || []).length !== 0}>
							<Input
								type="text"
								name="email"
								bgColor={"#FFFFFF"}
								borderColor={"#095FBA"}
								placeholder="Enter Email"
							/>
							{errors().email && (
								<FormErrorMessage>{errors().email}</FormErrorMessage>
							)}
						</FormControl>
						<FormLabel mb={2} my={3} color={"#095FBA"}>
							GST No.
						</FormLabel>
						<FormControl
							mt={2}
							isInvalid={(errors().gst_no || []).length !== 0}
						>
							<Input
								type="text"
								name="gst_no"
								bgColor={"#FFFFFF"}
								borderColor={"#095FBA"}
								placeholder="Enter GST No."
							/>
							{errors().gst_no && (
								<FormErrorMessage>{errors().gst_no}</FormErrorMessage>
							)}
						</FormControl>
						<FormLabel mb={2} my={3} color={"#095FBA"}>
							Pan No.
						</FormLabel>
						<FormControl
							mt={2}
							isInvalid={(errors().pan_no || []).length !== 0}
						>
							<Input
								type="text"
								name="pan_no"
								bgColor={"#FFFFFF"}
								borderColor={"#095FBA"}
								placeholder="Enter Pan No."
							/>
							{errors().pan_no && (
								<FormErrorMessage>{errors().pan_no}</FormErrorMessage>
							)}
						</FormControl>
						<SubmitButton loading={isSubmitting()} />
					</Box>
				</Center>
			</form>
		</div>
	);
};
