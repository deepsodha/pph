import {
	FormLabel,
	FormControl,
	FormErrorMessage,
	Input,
	Select,
	InputGroup,
	InputRightElement,
} from "@chakra-ui/react";
import { SubmitButton } from "../component/submit-button";
import { useForm } from "@felte/react";
import { validator } from "@felte/validator-zod";
import reporterDom from "@felte/reporter-dom";
import type {
	LastUpdatedInput,
	FamilyHistoryAdd,
	FamilyHistoryState,
} from "@repo/types/dto";
import { familyHistoryAddSchema } from "@repo/types/validation";
import { SearchIcon } from "@chakra-ui/icons";
import { MultiSelect, useMultiSelect } from "chakra-multiselect";
import { useEffect } from "react";

export type FamilyHistoryAddFormProps = {
	onSubmit: (v: FamilyHistoryAdd) => void;
	patientId?: string;
	FamilyHistoryId?: string;
	edit?: boolean;
	initialValues: FamilyHistoryState | undefined;
	lastUpdatedInput: () => Promise<LastUpdatedInput>;
};

export const FamilyHistoryForm = (props: FamilyHistoryAddFormProps) => {
	if (props.initialValues === undefined && props.edit === true) {
		return <p>Loading...</p>;
	}
	const {
		value: generalValue,
		options: generalOptions,
		onChange: generalOnchange,
	} = useMultiSelect({
		value: [] as string[],
		options: [
			{ label: "abc", value: "abc" },
			{ label: "jkl", value: "jkl" },
		],
	});
	const {
		value: cancerValue,
		options: cancerOptions,
		onChange: cancerOnchange,
	} = useMultiSelect({
		value: [] as string[],
		options: [
			{ label: "abc", value: "abc" },
			{ label: "jkl", value: "jkl" },
		],
	});
	const {
		value: commentsValue,
		options: commentsOptions,
		onChange: commentsOnchange,
	} = useMultiSelect({
		value: [] as string[],
		options: [
			{ label: "abc", value: "abc" },
			{ label: "jkl", value: "jkl" },
		],
	});
	const { form, errors, isSubmitting, setData } = useForm<FamilyHistoryAdd>({
		onSubmit: async (values) => {
			const generalValues =
				(values.general as { value: string; label: string }[] | null)?.map(
					(item) => item.value,
				) || [];
			const commentValues =
				(values.comments as { value: string; label: string }[] | null)?.map(
					(item) => item.value,
				) || [];
			const cancerValues =
				(values.cancer as { value: string; label: string }[] | null)?.map(
					(item) => item.value,
				) || [];

			const modifiedValues = {
				...values,
				general: generalValues,
				cancer: cancerValues,
				comments: commentValues,
				last_updated_input: await props.lastUpdatedInput(),
			};

			props.onSubmit(modifiedValues);
		},
		initialValues: {
			patient_id: props.patientId,
			family_member: props.initialValues?.family_member || "",
			health_status: props.initialValues?.health_status,
		},
		extend: [validator({ schema: familyHistoryAddSchema }), reporterDom()],
	});

	useEffect(() => {
		// setData("general", generalValue);
		if (generalValue !== undefined) {
			setData("general", generalValue as string[]);
		}
		if (cancerValue !== undefined) {
			setData("cancer", cancerValue as string[]);
		}

		if (commentsValue !== undefined) {
			setData("comments", commentsValue as string[]);
		}
	}, [generalValue, cancerValue, commentsValue, setData]);

	return (
		<form ref={form}>
			<FormLabel mb={2} my={3} color={"#095FBA"}>
				{"Family Member"}
			</FormLabel>
			<FormControl
				mt={2}
				isInvalid={(errors().family_member || []).length !== 0}
			>
				<Input
					type="text"
					name="family_member"
					bgColor="#FFFFFF"
					borderColor="#095FBA"
					placeholder={"Type Family Member"}
				/>
				{errors().family_member && (
					<FormErrorMessage>{errors().family_member}</FormErrorMessage>
				)}
			</FormControl>
			<FormLabel mb={2} my={3} color={"#095FBA"}>
				{"Health Status"}
			</FormLabel>
			<FormControl
				mt={2}
				isInvalid={(errors().health_status || []).length !== 0}
			>
				<Select
					name="health_status"
					bgColor="#FFFFFF"
					borderColor="#095FBA"
					placeholder={"Select Health Status"}
				>
					<option value="Alive">Alive</option>
					<option value="Deceased">Deceased</option>
					<option value="Unknown">Unknown</option>
				</Select>
				{errors().health_status && (
					<FormErrorMessage>{errors().health_status}</FormErrorMessage>
				)}
			</FormControl>
			<FormLabel mb={2} my={3} color={"#095FBA"}>
				{"General"}
			</FormLabel>
			<FormControl mt={2} isInvalid={(errors().general || []).length !== 0}>
				<InputGroup size="lg" alignItems={"center"}>
					<InputRightElement height="47px">
						<SearchIcon />
					</InputRightElement>
					{/* TODO : change select to multi-select */}
					<MultiSelect
						options={generalOptions}
						height={100}
						name="general"
						value={generalValue}
						label="Choose or create items"
						onChange={generalOnchange}
						create
					/>
				</InputGroup>
				{errors().general && (
					<FormErrorMessage>{errors().general}</FormErrorMessage>
				)}
			</FormControl>
			<FormLabel mb={2} my={3} color={"#095FBA"}>
				{"Cancer"}
			</FormLabel>
			<FormControl mt={2} isInvalid={(errors().cancer || []).length !== 0}>
				<InputGroup size="lg" alignItems={"center"}>
					<InputRightElement height="47px">
						<SearchIcon />
					</InputRightElement>
					<MultiSelect
						options={cancerOptions}
						height={100}
						name="cancer"
						value={cancerValue}
						label="Choose or create items"
						onChange={cancerOnchange}
						create
					/>
				</InputGroup>
				{errors().cancer && (
					<FormErrorMessage>{errors().cancer}</FormErrorMessage>
				)}
			</FormControl>

			<FormLabel mb={2} my={3} color={"#095FBA"}>
				Comments
			</FormLabel>
			<FormControl mt={2} isInvalid={(errors().comments || []).length !== 0}>
				<InputGroup size="lg" alignItems={"center"}>
					<InputRightElement height="47px">
						<SearchIcon />
					</InputRightElement>
					<MultiSelect
						options={commentsOptions}
						height={100}
						name="comments"
						value={commentsValue}
						label="Choose or create items"
						onChange={commentsOnchange}
						create
					/>
				</InputGroup>
				{errors().comments && (
					<FormErrorMessage>{errors().comments}</FormErrorMessage>
				)}
			</FormControl>
			<SubmitButton loading={isSubmitting()} />
		</form>
	);
};
