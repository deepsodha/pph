import type {
	GenderType,
	LastUpdatedInput,
	SocialHistoryState,
} from "@repo/types/dto";
import type { SocialHistoryAdd } from "../../types/dto/SocialHistoryAdd";
import { useForm } from "@felte/react";
import type { z } from "zod";
import { validator } from "@felte/validator-zod";
import { socialHistoryAddSchema } from "@repo/types/validation";
import reporterDom from "@felte/reporter-dom";
import { SubmitButton } from "../component";
import {
	Box,
	Button,
	FormControl,
	FormErrorMessage,
	FormLabel,
	Heading,
	InputGroup,
	InputRightElement,
	Select,
	Textarea,
} from "@chakra-ui/react";
import SelectGenderType from "../component/gender-types";

export type SocialHistoryAddFormProps = {
	onSubmit: (v: SocialHistoryAdd) => void;
	patientId?: string;
	socialHistoryId?: string;
	edit?: boolean;
	initialValues: SocialHistoryState | undefined;
	lastUpdatedInput: () => Promise<LastUpdatedInput>;
};

export const SocialHistoryForm = (props: SocialHistoryAddFormProps) => {
	if (props.initialValues === undefined && props.edit === true) {
		return <p>Loading...</p>;
	}
	const { form, errors, isSubmitting, data } = useForm<
		z.infer<typeof socialHistoryAddSchema>
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
			birth_gender: (props.initialValues?.birth_gender as GenderType) || "",
			comments: props.initialValues?.comments || "",
		},
		extend: [validator({ schema: socialHistoryAddSchema }), reporterDom()],
	});
	return (
		<div>
			<form ref={form}>
				<FormLabel mb={2} my={3} color={"#095FBA"}>
					Birth Gender
				</FormLabel>
				<FormControl
					mt={2}
					isInvalid={(errors().birth_gender || []).length !== 0}
				>
					<SelectGenderType name="birth_gender" />
					{errors().birth_gender && (
						<FormErrorMessage>{errors().birth_gender}</FormErrorMessage>
					)}
				</FormControl>
				<FormLabel mb={2} my={3} color={"#095FBA"}>
					Tobacco
				</FormLabel>
				<Box display="flex" alignItems="center" mt={2}>
					<InputGroup size="lg">
						<Select
							name="tobacco"
							bgColor="#FFFFFF"
							borderColor="#095FBA"
							value={data().tobacco}
							multiple
							height={150}
							defaultValue={["type1"]}
							sx={{
								"> option": {
									height: "30px",
									width: "100%",
									background: "white",
									border: "1px",
									borderColor: "#095FBA",
									borderRadius: "3px",
									borderBlockColor: "#095FBA",
								},
							}}
						>
							<option value="type1">type 1</option>
							<option value="type2">type 2</option>
							<option value="type3">type 3</option>
							<option value="type4">type 4</option>
							<option value="type5">type 5</option>
						</Select>
						<InputRightElement>
							<Button bgColor={"transparent"} mr="5%">
								{/* <SearchIcon bgColor={"transparent"} height="100%" width="100%"/> */}
							</Button>
						</InputRightElement>
					</InputGroup>
				</Box>
				<Heading fontSize={"sm"} color="red">
					{errors().tobacco}
				</Heading>
				<FormLabel mb={2} my={3} color={"#095FBA"}>
					Alcohol
				</FormLabel>
				<Box display="flex" alignItems="center" mt={2}>
					<InputGroup size="lg">
						<Select
							name="alcohol"
							bgColor="#FFFFFF"
							borderColor="#095FBA"
							value={data().alcohol}
							multiple
							height={150}
							defaultValue={["type1"]}
							sx={{
								"> option": {
									height: "30px",
									width: "100%",
									background: "white",
									border: "1px",
									borderColor: "#095FBA",
									borderRadius: "3px",
									borderBlockColor: "#095FBA",
								},
							}}
						>
							<option value="type1">type 1</option>
							<option value="type2">type 2</option>
							<option value="type3">type 3</option>
							<option value="type4">type 4</option>
							<option value="type5">type 5</option>
						</Select>
						<InputRightElement>
							<Button bgColor={"transparent"} mr="5%">
								{/* <SearchIcon bgColor={"transparent"} height="100%" width="100%"/> */}
							</Button>
						</InputRightElement>
					</InputGroup>
				</Box>
				<Heading fontSize={"sm"} color="red">
					{errors().alcohol}
				</Heading>
				<FormLabel mb={2} my={3} color={"#095FBA"}>
					Drug Abuse
				</FormLabel>
				<Box display="flex" alignItems="center" mt={2}>
					<InputGroup size="lg">
						<Select
							name="drug_abuse"
							bgColor="#FFFFFF"
							borderColor="#095FBA"
							value={data().drug_abuse}
							multiple
							height={150}
							defaultValue={["type1"]}
							sx={{
								"> option": {
									height: "30px",
									width: "100%",
									background: "white",
									border: "1px",
									borderColor: "#095FBA",
									borderRadius: "3px",
									borderBlockColor: "#095FBA",
								},
							}}
						>
							<option value="type1">type 1</option>
							<option value="type2">type 2</option>
							<option value="type3">type 3</option>
							<option value="type4">type 4</option>
							<option value="type5">type 5</option>
						</Select>
						<InputRightElement>
							<Button bgColor={"transparent"} mr="5%">
								{/* <SearchIcon bgColor={"transparent"} height="100%" width="100%"/> */}
							</Button>
						</InputRightElement>
					</InputGroup>
				</Box>
				<Heading fontSize={"sm"} color="red">
					{errors().drug_abuse}
				</Heading>
				<FormLabel mb={2} my={3} color={"#095FBA"}>
					Cardiovascular
				</FormLabel>
				<Box display="flex" alignItems="center" mt={2}>
					<InputGroup size="lg">
						<Select
							name="cardiovascular"
							bgColor="#FFFFFF"
							borderColor="#095FBA"
							value={data().cardiovascular}
							multiple
							height={150}
							defaultValue={["type1"]}
							sx={{
								"> option": {
									height: "30px",
									width: "100%",
									background: "white",
									border: "1px",
									borderColor: "#095FBA",
									borderRadius: "3px",
									borderBlockColor: "#095FBA",
								},
							}}
						>
							<option value="type1">type 1</option>
							<option value="type2">type 2</option>
							<option value="type3">type 3</option>
							<option value="type4">type 4</option>
							<option value="type5">type 5</option>
						</Select>
						<InputRightElement>
							<Button bgColor={"transparent"} mr="5%">
								{/* <SearchIcon bgColor={"transparent"} height="100%" width="100%"/> */}
							</Button>
						</InputRightElement>
					</InputGroup>
				</Box>
				<Heading fontSize={"sm"} color="red">
					{errors().cardiovascular}
				</Heading>
				<FormLabel mb={2} my={3} color={"#095FBA"}>
					Safety
				</FormLabel>
				<Box display="flex" alignItems="center" mt={2}>
					<InputGroup size="lg">
						<Select
							name="safety"
							bgColor="#FFFFFF"
							borderColor="#095FBA"
							value={data().safety}
							multiple
							height={150}
							defaultValue={["type1"]}
							sx={{
								"> option": {
									height: "30px",
									width: "100%",
									background: "white",
									border: "1px",
									borderColor: "#095FBA",
									borderRadius: "3px",
									borderBlockColor: "#095FBA",
								},
							}}
						>
							<option value="type1">type 1</option>
							<option value="type2">type 2</option>
							<option value="type3">type 3</option>
							<option value="type4">type 4</option>
							<option value="type5">type 5</option>
						</Select>
						<InputRightElement>
							<Button bgColor={"transparent"} mr="5%">
								{/* <SearchIcon bgColor={"transparent"} height="100%" width="100%"/> */}
							</Button>
						</InputRightElement>
					</InputGroup>
				</Box>
				<Heading fontSize={"sm"} color="red">
					{errors().safety}
				</Heading>
				<FormLabel mb={2} my={3} color={"#095FBA"}>
					Sexual Activity
				</FormLabel>
				<Box display="flex" alignItems="center" mt={2}>
					<InputGroup size="lg">
						<Select
							name="sexual_activity"
							bgColor="#FFFFFF"
							borderColor="#095FBA"
							value={data().sexual_activity}
							multiple
							height={150}
							defaultValue={["type1"]}
							sx={{
								"> option": {
									height: "30px",
									width: "100%",
									background: "white",
									border: "1px",
									borderColor: "#095FBA",
									borderRadius: "3px",
									borderBlockColor: "#095FBA",
								},
							}}
						>
							<option value="type1">type 1</option>
							<option value="type2">type 2</option>
							<option value="type3">type 3</option>
							<option value="type4">type 4</option>
							<option value="type5">type 5</option>
						</Select>
						<InputRightElement>
							<Button bgColor={"transparent"} mr="5%">
								{/* <SearchIcon bgColor={"transparent"} height="100%" width="100%"/> */}
							</Button>
						</InputRightElement>
					</InputGroup>
				</Box>
				<Heading fontSize={"sm"} color="red">
					{errors().sexual_activity}
				</Heading>

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
		</div>
	);
};
