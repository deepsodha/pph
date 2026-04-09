import {
	FormLabel,
	FormControl,
	FormErrorMessage,
	Textarea,
} from "@chakra-ui/react";
import { SubmitButton } from "../component/submit-button";
import { useForm } from "@felte/react";
import type { z } from "zod";
import { validator } from "@felte/validator-zod";
import reporterDom from "@felte/reporter-dom";
import type { LastUpdatedInput, PastSurgicalHistoryAdd } from "@repo/types/dto";
import { pastSurgicalHistoryAddScheama } from "@repo/types/validation";
import type { PastSurgicalHistoryStateExtend } from "@repo/types/dexie-state";
import { useState } from "react";
import ReactSelect, { type MultiValue } from "react-select";

export type PastSurgicalHistoryAddFormProps = {
	onSubmit: (v: PastSurgicalHistoryAdd) => void;
	patientId?: string;
	PastSurgicalHistoryId?: string;
	edit?: boolean;
	initialValues: PastSurgicalHistoryStateExtend | undefined;
	lastUpdatedInput: () => Promise<LastUpdatedInput>;
};

interface OptionType {
	value: string;
	label: string;
}

const options: OptionType[] = [
	{ value: "TypeA", label: "TypeA" },
	{ value: "TypeB", label: "TypeB" },
	{ value: "TypeC", label: "TypeC" },
	{ value: "TypeD", label: "TypeD" },
];

export const PastSurgicalHistoryForm = (
	props: PastSurgicalHistoryAddFormProps,
) => {
	if (props.initialValues === undefined && props.edit === true) {
		return <p>Loading...</p>;
	}
	const [selectedRoles, setSelectedRoles] = useState(
		props.initialValues?.common_surgeries || [],
	);
	const handleRoleChange = (newValue: MultiValue<OptionType>) => {
		if (newValue) {
			const selectedRoles: string[] = newValue.map(
				(option) => option.value as string,
			);
			setSelectedRoles(selectedRoles);
			setData("common_surgeries", selectedRoles);
		} else {
			setSelectedRoles([]);
		}
	};

	const { form, errors, setData, isSubmitting } = useForm<
		z.infer<typeof pastSurgicalHistoryAddScheama>
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
			comments: props.initialValues?.comments || "",
		},
		extend: [
			validator({ schema: pastSurgicalHistoryAddScheama }),
			reporterDom(),
		],
	});

	return (
		<form ref={form}>
			<FormLabel mb={2} my={3} color={"#095FBA"}>
				{"Common Surgeries"}
			</FormLabel>
			<FormControl
				mt={2}
				isInvalid={(errors().common_surgeries || []).length !== 0}
			>
				{/* <InputGroup size="lg" alignItems={"center"}>
					<InputRightElement height="47px">
						<SearchIcon />
					</InputRightElement> */}
				<ReactSelect
					name="common_surgeries"
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
						}),
					}}
				/>
				{/* </InputGroup> */}
				{errors().common_surgeries && (
					<FormErrorMessage>{errors().common_surgeries}</FormErrorMessage>
				)}
			</FormControl>
			<FormLabel mb={2} my={3} color={"#095FBA"}>
				Comments
			</FormLabel>
			<FormControl mt={2} isInvalid={(errors().comments || []).length !== 0}>
				<Textarea
					bgColor="#FFFFFF"
					name="comments"
					borderColor="#095FBA"
					placeholder="Comments"
				/>
				{errors().comments && (
					<FormErrorMessage>{errors().comments}</FormErrorMessage>
				)}
			</FormControl>

			<SubmitButton loading={isSubmitting()} />
		</form>
	);
};
