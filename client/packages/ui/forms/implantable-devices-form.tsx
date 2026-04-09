import {
	FormLabel,
	FormControl,
	FormErrorMessage,
	Input,
	Textarea,
	Checkbox,
} from "@chakra-ui/react";
import { SubmitButton } from "../component/submit-button";
import { useForm } from "@felte/react";
import type { z } from "zod";
import { validator } from "@felte/validator-zod";
import reporterDom from "@felte/reporter-dom";
import type {
	LastUpdatedInput,
	ImplantableDevicesAdd,
	ImplantableDevicesState,
} from "@repo/types/dto";
import { implanatableDevicesAddSchema } from "@repo/types/validation";
export type ImplantableDeviceAddFormProps = {
	onSubmit: (v: ImplantableDevicesAdd) => void;
	patientId?: string;
	ImplantabledeviceId?: string;
	edit?: boolean;
	initialValues: ImplantableDevicesState | undefined;
	lastUpdatedInput: () => Promise<LastUpdatedInput>;
};

export const ImplantableDeviceForm = (props: ImplantableDeviceAddFormProps) => {
	if (props.initialValues === undefined && props.edit === true) {
		return <p>Loading...</p>;
	}
	const { form, errors, isSubmitting } = useForm<
		z.infer<typeof implanatableDevicesAddSchema>
	>({
		onSubmit: async (values) => {
			const modifiedValues = {
				...values,
				last_updated_input: await props.lastUpdatedInput(),
			};

			props.onSubmit(modifiedValues);
		},
		initialValues: {
			patient_id: props.patientId,
			status: props.initialValues?.status || "",
			udi: props.initialValues?.udi || "",
			// udi_unknown:props.initialValues?.udi_unknown,
			comments: props.initialValues?.comments || "",
		},
		extend: [
			validator({ schema: implanatableDevicesAddSchema }),
			reporterDom(),
		],
	});

	return (
		<form ref={form}>
			{/* //TODO: make Status dropdown */}
			<FormLabel mb={2} my={3} color={"#095FBA"}>
				{"Status"}
			</FormLabel>
			<FormControl mt={2} isInvalid={(errors().status || []).length !== 0}>
				<Input
					type="text"
					name="status"
					bgColor="#FFFFFF"
					borderColor="#095FBA"
					placeholder={"Status"}
				/>
				{errors().status && (
					<FormErrorMessage>{errors().status}</FormErrorMessage>
				)}
			</FormControl>
			<FormLabel mb={2} my={3} color={"#095FBA"}>
				{"UDI"}
			</FormLabel>
			<FormControl mt={2} isInvalid={(errors().udi || []).length !== 0}>
				<Input
					type="text"
					name="udi"
					bgColor="#FFFFFF"
					borderColor="#095FBA"
					placeholder={"UDI"}
				/>
				{errors().udi && <FormErrorMessage>{errors().udi}</FormErrorMessage>}
			</FormControl>
			<Checkbox
				defaultChecked
				marginTop="20px"
				marginBottom="10px"
				name="udi_unknown"
			>
				UDI Unknown
			</Checkbox>

			<FormLabel mb={2} my={3} color={"#095FBA"}>
				Comments
			</FormLabel>
			<FormControl mt={2} isInvalid={(errors().comments || []).length !== 0}>
				<Textarea
					bgColor="#FFFFFF"
					name="comments"
					borderColor="#095FBA"
					placeholder="Comments"
					maxLength={100}
				/>
				{errors().comments && (
					<FormErrorMessage>{errors().comments}</FormErrorMessage>
				)}
			</FormControl>

			<SubmitButton loading={isSubmitting()} />
		</form>
	);
};
