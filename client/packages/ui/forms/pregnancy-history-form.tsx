import { useForm } from "@felte/react";
import { validator } from "@felte/validator-zod";
import type {
	LastUpdatedInput,
	OBandPregnancyAdd,
	OBandPregnancyState,
} from "@repo/types/dto";
import { obAndPregnancyAddSchemaa } from "@repo/types/validation";
import type { z } from "zod";
import { SubmitButton } from "../component";
import reporterDom from "@felte/reporter-dom";
import {
	Box,
	Center,
	FormControl,
	FormErrorMessage,
	FormLabel,
	Input,
	NumberDecrementStepper,
	NumberIncrementStepper,
	NumberInput,
	NumberInputField,
	NumberInputStepper,
	Text,
	Textarea,
} from "@chakra-ui/react";

export type PregnancyHistoryAddFormProps = {
	onSubmit: (v: OBandPregnancyAdd) => void;
	patientId?: string;
	pregnancyHistoryId?: string;
	edit?: boolean;
	initialValues: OBandPregnancyState | undefined;
	lastUpdatedInput: () => Promise<LastUpdatedInput>;
};

export const PregnancyHistoryForm = (props: PregnancyHistoryAddFormProps) => {
	if (props.initialValues === undefined && props.edit === true) {
		return <p>Loading...</p>;
	}
	const { form, errors, setData, data, isSubmitting } = useForm<
		z.infer<typeof obAndPregnancyAddSchemaa>
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
			age_onset_of_menses: props.initialValues?.age_onset_of_menses,
			age_at_menopause: props.initialValues?.age_at_menopause,
			comments_ob: props.initialValues?.comments_ob || "",
			total_pregnancy: props.initialValues?.total_pregnancy || undefined,
			full_term: props.initialValues?.full_term || undefined,
			pre_term: props.initialValues?.pre_term || undefined,
			miscarriages: props.initialValues?.miscarriages || undefined,
			living: props.initialValues?.living || undefined,
			comments_pregnancy: props.initialValues?.comments_pregnancy || "",
		},
		extend: [validator({ schema: obAndPregnancyAddSchemaa }), reporterDom()],
	});

	return (
		<form ref={form}>
			<FormLabel mb={2} my={3} color={"#000000"}>
				{"OB History"}
			</FormLabel>

			<FormLabel mb={2} my={3} color={"#095FBA"}>
				{"Age Onset of Menses"}
			</FormLabel>
			<Box width="100%" display="flex">
				<FormControl
					isInvalid={(errors().age_onset_of_menses || []).length !== 0}
				>
					<Input
						type="number"
						name="age_onset_of_menses"
						bgColor="#FFFFFF"
						borderColor="#095FBA"
						placeholder={"Age Onset of Menses"}
					/>
					{errors().age_onset_of_menses && (
						<FormErrorMessage>{errors().age_onset_of_menses}</FormErrorMessage>
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
						<Text mt={2} color={"#717B9E"}>
							Years
						</Text>
					</Center>
				</Box>
			</Box>

			<FormLabel mb={2} my={3} color={"#095FBA"}>
				{"Age at Menopause"}
			</FormLabel>
			<Box width="100%" display="flex">
				<FormControl isInvalid={(errors().age_at_menopause || []).length !== 0}>
					<Input
						type="number"
						name="age_at_menopause"
						bgColor="#FFFFFF"
						borderColor="#095FBA"
						placeholder={"Age at Menopause"}
					/>
					{errors().age_at_menopause && (
						<FormErrorMessage>{errors().age_at_menopause}</FormErrorMessage>
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
						<Text mt={2} color={"#717B9E"}>
							Years
						</Text>
					</Center>
				</Box>
			</Box>

			<FormLabel mb={2} my={3} color={"#095FBA"}>
				Comments
			</FormLabel>
			<FormControl mt={2} isInvalid={(errors().comments_ob || []).length !== 0}>
				<Textarea
					bgColor="#FFFFFF"
					name="comments_ob"
					borderColor="#095FBA"
					placeholder="Comments"
					maxLength={100}
				/>
				{errors().comments_ob && (
					<FormErrorMessage>{errors().comments_ob}</FormErrorMessage>
				)}
			</FormControl>

			<FormLabel mb={2} my={3} color={"#000000"}>
				{"Pregnancy History"}
			</FormLabel>

			<FormLabel mb={2} my={3} color={"#095FBA"}>
				Total Pregnancy
			</FormLabel>
			<Box justifyContent={"space-around"} display="flex">
				<Box width="100%">
					<NumberInput
						onChange={(_, valueAsNumber) => {
							setData("total_pregnancy", valueAsNumber);
						}}
						borderColor="#095fba"
						value={data().total_pregnancy || undefined}
						bgColor="#ffffff"
					>
						<NumberInputField
							placeholder="Total Pregnancy"
							borderColor="#095fba"
						/>
						<NumberInputStepper>
							<NumberIncrementStepper borderColor={"#095FBA"} />
							<NumberDecrementStepper borderColor={"#095FBA"} />
						</NumberInputStepper>
					</NumberInput>
				</Box>
			</Box>

			<FormLabel mb={2} my={3} color={"#095FBA"}>
				Full Term
			</FormLabel>
			<Box justifyContent={"space-around"} display="flex">
				<Box width="100%">
					<NumberInput
						onChange={(_, valueAsNumber) => {
							setData("full_term", valueAsNumber);
						}}
						borderColor="#095fba"
						value={data().full_term || undefined}
						bgColor="#ffffff"
					>
						<NumberInputField placeholder="Full Term" borderColor="#095fba" />
						<NumberInputStepper>
							<NumberIncrementStepper borderColor={"#095FBA"} />
							<NumberDecrementStepper borderColor={"#095FBA"} />
						</NumberInputStepper>
					</NumberInput>
				</Box>
			</Box>

			<FormLabel mb={2} my={3} color={"#095FBA"}>
				Pre Term
			</FormLabel>
			<Box justifyContent={"space-around"} display="flex">
				<Box width="100%">
					<NumberInput
						onChange={(_, valueAsNumber) => {
							setData("pre_term", valueAsNumber);
						}}
						borderColor="#095fba"
						value={data().pre_term || undefined}
						bgColor="#ffffff"
					>
						<NumberInputField placeholder="Pre Term" borderColor="#095fba" />
						<NumberInputStepper>
							<NumberIncrementStepper borderColor={"#095FBA"} />
							<NumberDecrementStepper borderColor={"#095FBA"} />
						</NumberInputStepper>
					</NumberInput>
				</Box>
			</Box>

			<FormLabel mb={2} my={3} color={"#095FBA"}>
				Miscarriages
			</FormLabel>
			<Box justifyContent={"space-around"} display="flex">
				<Box width="100%">
					<NumberInput
						onChange={(_, valueAsNumber) => {
							setData("miscarriages", valueAsNumber);
						}}
						borderColor="#095fba"
						value={data().miscarriages || undefined}
						bgColor="#ffffff"
					>
						<NumberInputField
							placeholder="Miscarriages"
							borderColor="#095fba"
						/>
						<NumberInputStepper>
							<NumberIncrementStepper borderColor={"#095FBA"} />
							<NumberDecrementStepper borderColor={"#095FBA"} />
						</NumberInputStepper>
					</NumberInput>
				</Box>
			</Box>

			<FormLabel mb={2} my={3} color={"#095FBA"}>
				Living
			</FormLabel>
			<Box justifyContent={"space-around"} display="flex">
				<Box width="100%">
					<NumberInput
						onChange={(_, valueAsNumber) => {
							setData("living", valueAsNumber);
						}}
						borderColor="#095fba"
						value={data().living || undefined}
						bgColor="#ffffff"
					>
						<NumberInputField placeholder="Living" borderColor="#095fba" />
						<NumberInputStepper>
							<NumberIncrementStepper borderColor={"#095FBA"} />
							<NumberDecrementStepper borderColor={"#095FBA"} />
						</NumberInputStepper>
					</NumberInput>
				</Box>
			</Box>

			<FormLabel mb={2} my={3} color={"#095FBA"}>
				Comments
			</FormLabel>
			<FormControl
				mt={2}
				isInvalid={(errors().comments_pregnancy || []).length !== 0}
			>
				<Textarea
					bgColor="#FFFFFF"
					name=".comments_pregnancy"
					borderColor="#095FBA"
					placeholder="Comments"
					maxLength={100}
				/>
				{errors().comments_pregnancy && (
					<FormErrorMessage>{errors().comments_pregnancy}</FormErrorMessage>
				)}
			</FormControl>

			<SubmitButton loading={isSubmitting()} />
		</form>
	);
};
