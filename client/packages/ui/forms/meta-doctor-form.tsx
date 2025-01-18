import {
	Box,
	Center,
	FormControl,
	FormErrorMessage,
	FormLabel,
	Input,
} from "@chakra-ui/react";
import { SubmitButton } from "../component/submit-button";
import { useForm } from "@felte/react";
import { DoctorSchema } from "@repo/types/validation";
import { validator } from "@felte/validator-zod";
import type { z } from "zod";
import type { Doctors } from "@repo/types/dto";
import reporterDom from "@felte/reporter-dom";

export type DoctorAddProps = {
	onSubmit: (p: Doctors) => void;
	edit?: boolean;
	initialValues?: Doctors | undefined;
};

const DoctorAddForm = (props: DoctorAddProps) => {
	const { form, errors, isSubmitting } = useForm<z.infer<typeof DoctorSchema>>({
		onSubmit: (values) => {
			props.onSubmit(values);
		},
		initialValues: {
			doctor_name: props.initialValues?.doctor_name,
			speciality: props.initialValues?.speciality,
			experience: props.initialValues?.experience,
			hospital_address: props.initialValues?.hospital_address,
			city: props.initialValues?.city,
			pincode: props.initialValues?.pincode,
		},
		extend: [validator({ schema: DoctorSchema }), reporterDom()],
	});

	return (
		<>
			<form ref={form}>
				<Center>
					<Box width={"90%"}>
						<FormLabel mb={2} my={3} color={"#095FBA"}>
							Doctor Name
						</FormLabel>
						<FormControl
							mt={2}
							isInvalid={(errors().doctor_name || []).length !== 0}
						>
							<Input
								type="text"
								name="doctor_name"
								bgColor={"#FFFFFF"}
								borderColor={"#095FBA"}
								placeholder="Enter Doctor Name"
							/>
							{errors().doctor_name && (
								<FormErrorMessage>{errors().doctor_name}</FormErrorMessage>
							)}
						</FormControl>
						<FormLabel mb={2} my={3} color={"#095FBA"}>
							Speciality
						</FormLabel>
						<FormControl
							mt={2}
							isInvalid={(errors().speciality || []).length !== 0}
						>
							<Input
								type="text"
								name="speciality"
								bgColor={"#FFFFFF"}
								borderColor={"#095FBA"}
								placeholder="Enter Speciality"
							/>
							{errors().speciality && (
								<FormErrorMessage>{errors().speciality}</FormErrorMessage>
							)}
						</FormControl>
						<FormLabel mb={2} my={3} color={"#095FBA"}>
							Experience
						</FormLabel>
						<FormControl
							mt={2}
							isInvalid={(errors().experience || []).length !== 0}
						>
							<Input
								type="text"
								name="experience"
								bgColor={"#FFFFFF"}
								borderColor={"#095FBA"}
								placeholder="Enter Experience"
							/>
							{errors().experience && (
								<FormErrorMessage>{errors().experience}</FormErrorMessage>
							)}
						</FormControl>
						<FormLabel mb={2} my={3} color={"#095FBA"}>
							Hospital Address
						</FormLabel>
						<FormControl
							mt={2}
							isInvalid={(errors().hospital_address || []).length !== 0}
						>
							<Input
								type="text"
								name="hospital_address"
								bgColor={"#FFFFFF"}
								borderColor={"#095FBA"}
								placeholder="Enter Hospital Address"
							/>
							{errors().hospital_address && (
								<FormErrorMessage>{errors().hospital_address}</FormErrorMessage>
							)}
						</FormControl>
						<FormLabel mb={2} my={3} color={"#095FBA"}>
							City
						</FormLabel>
						<FormControl mt={2} isInvalid={(errors().city || []).length !== 0}>
							<Input
								type="text"
								name="city"
								bgColor={"#FFFFFF"}
								borderColor={"#095FBA"}
								placeholder="Enter city"
							/>
							{errors().city && (
								<FormErrorMessage>{errors().city}</FormErrorMessage>
							)}
						</FormControl>
						<FormLabel mb={2} my={3} color={"#095FBA"}>
							Pincode
						</FormLabel>
						<FormControl
							mt={2}
							isInvalid={(errors().pincode || []).length !== 0}
						>
							<Input
								type="text"
								name="pincode"
								bgColor={"#FFFFFF"}
								borderColor={"#095FBA"}
								placeholder="Enter pincode"
							/>
							{errors().pincode && (
								<FormErrorMessage>{errors().pincode}</FormErrorMessage>
							)}
						</FormControl>
						<SubmitButton loading={isSubmitting()} />
					</Box>
				</Center>
			</form>
		</>
	);
};

export default DoctorAddForm;
