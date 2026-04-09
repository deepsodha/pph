import {
	FormLabel,
	Box,
	Text,
	FormControl,
	FormErrorMessage,
	Input,
	Center,
} from "@chakra-ui/react";
import { SubmitButton } from "../component/submit-button";
import { useForm } from "@felte/react";
import { validator } from "@felte/validator-zod";
import reporterDom from "@felte/reporter-dom";
import { convertToUTC } from "../component/convert-to-utc";
import type { VitalsState, VitalsAdd, LastUpdatedInput } from "@repo/types/dto";
import { vitalsAddSchema } from "@repo/types/validation";
import { convertUTCtoLocal } from "../component/utc-date-to-normal-date";
import type { z } from "zod";
import DateComponent from "../component/date-comp";
export type VitalAddFormProps = {
	onSubmit: (v: VitalsAdd) => void;
	patientId?: string;
	VitalsId?: string;
	edit?: boolean;
	initialValues?: VitalsState | undefined;
	lastUpdatedInput: () => Promise<LastUpdatedInput>;
};

export const VitalsForm = (props: VitalAddFormProps) => {
	if (props.initialValues === undefined && props.edit === true) {
		return <p>Loading...</p>;
	}
	const today = new Date();
	today.setDate(today.getDate() - 1);
	const yesterday = today.toISOString().slice(0, 16);
	const { form, errors, isSubmitting } = useForm<
		z.infer<typeof vitalsAddSchema>
	>({
		onSubmit: async (values) => {
			const utcDate = convertToUTC(values.date_time ?? "");
			const modifiedValues = {
				...values,
				last_updated_input: await props.lastUpdatedInput(),
				date_time: utcDate,
			};

			props.onSubmit(modifiedValues);
		},
		initialValues: {
			patient_id: props.patientId,
			date_time:
				(props.initialValues?.date_time &&
					convertUTCtoLocal(props.initialValues.date_time)) ||
				"",
			blood_pressure: props.initialValues?.blood_pressure || undefined,
			heart_rate: props.initialValues?.heart_rate || undefined,
			height: props.initialValues?.height || undefined,
			weight: props.initialValues?.weight || undefined,
			temperature: props.initialValues?.temperature || undefined,
			bmi: props.initialValues?.bmi || undefined,
			comments: props.initialValues?.comments || "",
		},
		extend: [validator({ schema: vitalsAddSchema }), reporterDom()],
	});

	return (
		<form ref={form}>
			<FormLabel mb={2} my={3} color={"#095FBA"}>
				{"Date and Time"}
				<span style={{ color: "red" }}>*</span>
			</FormLabel>
			<FormControl mt={2} isInvalid={(errors().date_time || []).length !== 0}>
				<DateComponent
					type={"datetime-local"}
					name={"date_time"}
					placeholder={"Date and Time"}
					max={yesterday}
				/>
				{errors().date_time && (
					<FormErrorMessage>{errors().date_time}</FormErrorMessage>
				)}
			</FormControl>
			<FormLabel mb={2} my={3} color={"#095FBA"}>
				{"Blood Pressure"}
				<span style={{ color: "red" }}>*</span>
			</FormLabel>
			<Box
				style={{ display: "flex" }}
				width="100%"
				alignItems={"center"}
				justifyContent={"space-around"}
			>
				<Box width="100%">
					<FormControl isInvalid={(errors().blood_pressure || []).length !== 0}>
						<Input
							type={"number"}
							name={"blood_pressure"}
							width="100%"
							bgColor={"#FFFFFF"}
							borderColor={"#095FBA"}
							placeholder={"120"}
						/>
						{errors().blood_pressure && (
							<FormErrorMessage>{errors().blood_pressure}</FormErrorMessage>
						)}
					</FormControl>
				</Box>
				<Box>
					<Text color={"#095FBA"} fontSize={"4xl"} mr={2} ml={2}>
						/
					</Text>
				</Box>
				<Box width="100%">
					<FormControl isInvalid={(errors().blood_pressure || []).length !== 0}>
						<Input
							type={"number"}
							name={"blood_pressure"}
							bgColor={"#FFFFFF"}
							borderColor={"#095FBA"}
							placeholder={"120"}
						/>
						{errors().blood_pressure && (
							<FormErrorMessage>{errors().blood_pressure}</FormErrorMessage>
						)}
					</FormControl>
				</Box>
				<Box
					mt={1}
					width="40%"
					ml="2%"
					border="1px"
					borderColor="#095FBA"
					bgColor="#FFFFFF"
					height="40px"
					borderRadius="md"
					alignItems="center"
				>
					<Center>
						<Text mt={1}>mmHg</Text>
					</Center>
				</Box>
			</Box>

			<FormLabel mb={2} my={3} color={"#095FBA"}>
				{"Heart Rate"}
				<span style={{ color: "red" }}>*</span>
			</FormLabel>
			<Box width="100%" display="flex">
				<FormControl isInvalid={(errors().heart_rate || []).length !== 0}>
					<Input
						type="number"
						name="heart_rate"
						bgColor="#FFFFFF"
						borderColor="#095FBA"
						placeholder={"Heart Rate"}
					/>
					{errors().heart_rate && (
						<FormErrorMessage>{errors().heart_rate}</FormErrorMessage>
					)}
				</FormControl>
				<Box
					width="20%"
					ml="2%"
					border="1px"
					borderColor="#095FBA"
					bgColor="#FFFFFF"
					height="40px"
					borderRadius="md"
					alignItems="center"
				>
					<Center>
						<Text mt={1}>bpm</Text>
					</Center>
				</Box>
			</Box>
			<FormLabel mb={2} my={3} color={"#095FBA"}>
				Height
				<span style={{ color: "red" }}>*</span>
			</FormLabel>
			<Box justifyContent={"space-around"} display="flex">
				<Box width="100%">
					<FormControl isInvalid={(errors().height || []).length !== 0}>
						<Input
							type={"number"}
							name={"height"}
							width="100%"
							bgColor={"#FFFFFF"}
							borderColor={"#095FBA"}
							placeholder={"6'"}
						/>
						{errors().height && (
							<FormErrorMessage>{errors().height}</FormErrorMessage>
						)}
					</FormControl>
				</Box>
				<Box>
					<Text ml={3} mt={1} mr={4}>
						ft
					</Text>
				</Box>
				<Box width="100%">
					<FormControl isInvalid={(errors().height || []).length !== 0}>
						<Input
							type={"number"}
							name={"height"}
							width="100%"
							bgColor={"#FFFFFF"}
							borderColor={"#095FBA"}
							placeholder={"4'"}
						/>
						{errors().height && (
							<FormErrorMessage>{errors().height}</FormErrorMessage>
						)}
					</FormControl>
				</Box>
				<Box>
					<Text ml={3} mt={1} mr={4}>
						in
					</Text>
				</Box>
			</Box>
			<FormLabel mb={2} my={3} color={"#095FBA"}>
				{"Weight"}
				<span style={{ color: "red" }}>*</span>
			</FormLabel>
			<Box width="100%" display="flex">
				<FormControl isInvalid={(errors().weight || []).length !== 0}>
					<Input
						type="number"
						name="weight"
						bgColor="#FFFFFF"
						borderColor="#095FBA"
						placeholder={"Weight"}
					/>
					{errors().weight && (
						<FormErrorMessage>{errors().weight}</FormErrorMessage>
					)}
				</FormControl>
				<Box
					width="20%"
					ml="2%"
					border="1px"
					borderColor="#095FBA"
					bgColor="#FFFFFF"
					height="40px"
					borderRadius="md"
					alignItems="center"
				>
					<Center>
						<Text mt={1}>lbs</Text>
					</Center>
				</Box>
			</Box>
			<FormLabel mb={2} my={3} color={"#095FBA"}>
				{"Temperature"}
				<span style={{ color: "red" }}>*</span>
			</FormLabel>
			<Box width="100%" display="flex">
				<FormControl isInvalid={(errors().temperature || []).length !== 0}>
					<Input
						type="number"
						name="temperature"
						bgColor="#FFFFFF"
						borderColor="#095FBA"
						placeholder={"Temperature"}
					/>
					{errors().temperature && (
						<FormErrorMessage>{errors().temperature}</FormErrorMessage>
					)}
				</FormControl>
				<Box
					width="20%"
					ml="2%"
					border="1px"
					borderColor="#095FBA"
					bgColor="#FFFFFF"
					height="40px"
					borderRadius="md"
					alignItems="center"
				>
					<Center>
						<Text mt={1}>°f</Text>
					</Center>
				</Box>
			</Box>
			<div>
				<FormLabel mb={2} my={3} color={"#095FBA"}>
					{"BMI"}
					<span style={{ color: "red" }}>*</span>
				</FormLabel>
				<FormControl mt={2} isInvalid={(errors().bmi || []).length !== 0}>
					<Input
						type="number"
						name="bmi"
						bgColor="#FFFFFF"
						borderColor="#095FBA"
						placeholder={"BMI"}
					/>
					{errors().bmi && <FormErrorMessage>{errors().bmi}</FormErrorMessage>}
				</FormControl>
			</div>
			<div>
				<FormLabel mb={2} my={3} color={"#095FBA"}>
					{"Comments"}
				</FormLabel>
				<FormControl mt={2} isInvalid={(errors().comments || []).length !== 0}>
					<Input
						type="text"
						name="comments"
						bgColor="#FFFFFF"
						borderColor="#095FBA"
						placeholder={"Comments"}
						maxLength={100}
					/>
					{errors().comments && (
						<FormErrorMessage>{errors().comments}</FormErrorMessage>
					)}
				</FormControl>
			</div>

			<SubmitButton loading={isSubmitting()} />
		</form>
	);
};
