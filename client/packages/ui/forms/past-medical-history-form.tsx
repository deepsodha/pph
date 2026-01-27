import { useForm } from "@felte/react";
import { SubmitButton } from "../component/submit-button";
import {
	FormLabel,
	FormControl,
	Select,
	Box,
	InputRightElement,
	Input,
	InputGroup,
	FormErrorMessage,
	Textarea,
} from "@chakra-ui/react";
import { SearchIcon } from "@chakra-ui/icons";
import type {
	LastUpdatedInput,
	PastMedicalHistoryAdd,
	PastMedicalHistoryState,
} from "@repo/types/dto";
import type { z } from "zod";
import { validator } from "@felte/validator-zod";
import reporterDom from "@felte/reporter-dom";
import { pastMedicalHistoryAddSchema } from "@repo/types/validation";

export type PastHistoryDetailsProps = {
	onSubmit: (p: PastMedicalHistoryAdd) => void;
	initialValues: PastMedicalHistoryState | undefined;
	patientId?: string;
	PastMedicalHistoryId?: string;
	edit?: boolean;
	lastUpdatedInput: () => Promise<LastUpdatedInput>;
};

export const PastMedicalHistoryForm = (props: PastHistoryDetailsProps) => {
	if (props.initialValues === undefined && props.edit === true) {
		return <p>Loading...</p>;
	}
	const { form, errors, isSubmitting } = useForm<
		z.infer<typeof pastMedicalHistoryAddSchema>
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
			blood_type: props.initialValues?.blood_type || "",
			head: props.initialValues?.head || "",
			respiratory: props.initialValues?.respiratory || "",
			musculoskeletal: props.initialValues?.musculoskeletal || "",
			endocrine: props.initialValues?.endocrine || "",
			eyes: props.initialValues?.eyes || "",
			gastrointestinal: props.initialValues?.gastrointestinal || "",
			skin: props.initialValues?.skin || "",
			ears: props.initialValues?.ears || "",
			noses: props.initialValues?.noses || "",
			neurological: props.initialValues?.neurological || "",
			heme: props.initialValues?.heme || "",
			mouth: props.initialValues?.mouth || "",
			infectious: props.initialValues?.infectious || "",
			cardiovascular: props.initialValues?.cardiovascular || "",
			genitourinary: props.initialValues?.genitourinary || "",
			psychiatric: props.initialValues?.psychiatric || "",
			comments: props.initialValues?.comments || "",
		},
		extend: [validator({ schema: pastMedicalHistoryAddSchema }), reporterDom()],
	});

	return (
		<div>
			<form ref={form}>
				<Box
					display={"flex"}
					flexDirection={"column"}
					justifyContent={"center"}
					alignItems={"left"}
				>
					<FormLabel mb={2} color={"#095FBA"} my={3}>
						Blood Type
					</FormLabel>
					<FormControl
						mt={2}
						width={"95%"}
						isInvalid={(errors().blood_type || []).length !== 0}
					>
						<Select
							size={"lg"}
							placeholder="Blood Type"
							_placeholder={{ color: "#717B9E" }}
							name="blood_type"
							bgColor="#FFFFFF"
							borderColor="#095FBA"
						>
							<option value="option1">Option 1</option>
							<option value="option2">Option 2</option>
							<option value="option3">Option 3</option>
						</Select>
						{errors().blood_type && (
							<FormErrorMessage>{errors().blood_type}</FormErrorMessage>
						)}
					</FormControl>
					<FormLabel mb={2} my={3} color={"#095FBA"}>
						Head
					</FormLabel>
					<FormControl isInvalid={(errors().head || []).length !== 0}>
						<InputGroup size="lg" alignItems={"center"}>
							<InputRightElement height="47px" mr={4}>
								<SearchIcon />
							</InputRightElement>
							<Input
								borderColor={"#095FBA"}
								placeholder="Head"
								bgColor={"#FFFFFF"}
								type={"text"}
								name="head"
								mr={4}
							/>
						</InputGroup>
						{errors().head && (
							<FormErrorMessage>{errors().head}</FormErrorMessage>
						)}
					</FormControl>
					<FormLabel mb={2} my={3} color={"#095FBA"}>
						Eyes
					</FormLabel>
					<FormControl isInvalid={(errors().eyes || []).length !== 0}>
						<InputGroup size="lg" alignItems={"center"}>
							<InputRightElement height="47px" mr={4}>
								<SearchIcon />
							</InputRightElement>
							<Input
								borderColor={"#095FBA"}
								placeholder="eyes"
								bgColor={"#FFFFFF"}
								type={"text"}
								name="eyes"
								mr={4}
							/>
						</InputGroup>
						{errors().eyes && (
							<FormErrorMessage>{errors().eyes}</FormErrorMessage>
						)}
					</FormControl>
					<FormLabel mb={2} my={3} color={"#095FBA"}>
						Ears
					</FormLabel>
					<FormControl isInvalid={(errors().eyes || []).length !== 0}>
						<InputGroup size="lg" alignItems={"center"}>
							<InputRightElement height="47px" mr={4}>
								<SearchIcon />
							</InputRightElement>
							<Input
								borderColor={"#095FBA"}
								placeholder="Ears"
								bgColor={"#FFFFFF"}
								type={"text"}
								name="ears"
								mr={4}
							/>
						</InputGroup>
						{errors().ears && (
							<FormErrorMessage>{errors().ears}</FormErrorMessage>
						)}
					</FormControl>
					<FormLabel mb={2} my={3} color={"#095FBA"}>
						Nose / Sinuses
					</FormLabel>
					<FormControl isInvalid={(errors().noses || []).length !== 0}>
						<InputGroup size="lg" alignItems={"center"}>
							<InputRightElement height="47px" mr={4}>
								<SearchIcon />
							</InputRightElement>
							<Input
								borderColor={"#095FBA"}
								placeholder="Nose / Sinuses"
								bgColor={"#FFFFFF"}
								type={"text"}
								name="noses"
								mr={4}
							/>
						</InputGroup>
						{errors().noses && (
							<FormErrorMessage>{errors().noses}</FormErrorMessage>
						)}
					</FormControl>
					<FormLabel mb={2} my={3} color={"#095FBA"}>
						Mouth / Throat / Teeth
					</FormLabel>
					<FormControl isInvalid={(errors().mouth || []).length !== 0}>
						<InputGroup size="lg" alignItems={"center"}>
							<InputRightElement height="47px" mr={4}>
								<SearchIcon />
							</InputRightElement>
							<Input
								borderColor={"#095FBA"}
								placeholder="Mouth / Throat / Teeth"
								bgColor={"#FFFFFF"}
								type={"text"}
								name="mouth"
								mr={4}
							/>
						</InputGroup>
						{errors().mouth && (
							<FormErrorMessage>{errors().mouth}</FormErrorMessage>
						)}
					</FormControl>
					<FormLabel mb={2} my={3} color={"#095FBA"}>
						Cardiovascular
					</FormLabel>
					<FormControl isInvalid={(errors().cardiovascular || []).length !== 0}>
						<InputGroup size="lg" alignItems={"center"}>
							<InputRightElement height="47px" mr={4}>
								<SearchIcon />
							</InputRightElement>
							<Input
								borderColor={"#095FBA"}
								placeholder="Cardiovascular"
								bgColor={"#FFFFFF"}
								type={"text"}
								name="cardiovascular"
								mr={4}
							/>
						</InputGroup>
						{errors().cardiovascular && (
							<FormErrorMessage>{errors().cardiovascular}</FormErrorMessage>
						)}
					</FormControl>
					<FormLabel mb={2} my={3} color={"#095FBA"}>
						Respiratory
					</FormLabel>
					<FormControl isInvalid={(errors().respiratory || []).length !== 0}>
						<InputGroup size="lg" alignItems={"center"}>
							<InputRightElement height="47px" mr={4}>
								<SearchIcon />
							</InputRightElement>
							<Input
								borderColor={"#095FBA"}
								placeholder="Respiratory"
								bgColor={"#FFFFFF"}
								type={"text"}
								name="respiratory"
								mr={4}
							/>
						</InputGroup>
						{errors().respiratory && (
							<FormErrorMessage>{errors().respiratory}</FormErrorMessage>
						)}
					</FormControl>
					<FormLabel mb={2} my={3} color={"#095FBA"}>
						Gastrointestinal
					</FormLabel>
					<FormControl
						isInvalid={(errors().gastrointestinal || []).length !== 0}
					>
						<InputGroup size="lg" alignItems={"center"}>
							<InputRightElement height="47px" mr={4}>
								<SearchIcon />
							</InputRightElement>
							<Input
								borderColor={"#095FBA"}
								placeholder="Gastrointestinal"
								bgColor={"#FFFFFF"}
								type={"text"}
								name="gastrointestinal"
								mr={4}
							/>
						</InputGroup>
						{errors().gastrointestinal && (
							<FormErrorMessage>{errors().gastrointestinal}</FormErrorMessage>
						)}
					</FormControl>
					<FormLabel mb={2} my={3} color={"#095FBA"}>
						Genitourinary
					</FormLabel>
					<FormControl isInvalid={(errors().genitourinary || []).length !== 0}>
						<InputGroup size="lg" alignItems={"center"}>
							<InputRightElement height="47px" mr={4}>
								<SearchIcon />
							</InputRightElement>
							<Input
								borderColor={"#095FBA"}
								placeholder="Genitourinary"
								bgColor={"#FFFFFF"}
								type={"text"}
								name="genitourinary"
								mr={4}
							/>
						</InputGroup>
						{errors().genitourinary && (
							<FormErrorMessage>{errors().genitourinary}</FormErrorMessage>
						)}
					</FormControl>
					<FormLabel mb={2} my={3} color={"#095FBA"}>
						Musculoskeletal
					</FormLabel>
					<FormControl
						isInvalid={(errors().musculoskeletal || []).length !== 0}
					>
						<InputGroup size="lg" alignItems={"center"}>
							<InputRightElement height="47px" mr={4}>
								<SearchIcon />
							</InputRightElement>
							<Input
								borderColor={"#095FBA"}
								placeholder="Musculoskeletal"
								bgColor={"#FFFFFF"}
								type={"text"}
								name="musculoskeletal"
								mr={4}
							/>
						</InputGroup>
						{errors().musculoskeletal && (
							<FormErrorMessage>{errors().musculoskeletal}</FormErrorMessage>
						)}
					</FormControl>
					<FormLabel mb={2} my={3} color={"#095FBA"}>
						Skin
					</FormLabel>
					<FormControl isInvalid={(errors().skin || []).length !== 0}>
						<InputGroup size="lg" alignItems={"center"}>
							<InputRightElement height="47px" mr={4}>
								<SearchIcon />
							</InputRightElement>
							<Input
								borderColor={"#095FBA"}
								placeholder="Skin"
								bgColor={"#FFFFFF"}
								type={"text"}
								name="skin"
								mr={4}
							/>
						</InputGroup>
						{errors().skin && (
							<FormErrorMessage>{errors().skin}</FormErrorMessage>
						)}
					</FormControl>

					<FormLabel mb={2} my={3} color={"#095FBA"}>
						Neurological
					</FormLabel>
					<FormControl isInvalid={(errors().neurological || []).length !== 0}>
						<InputGroup size="lg" alignItems={"center"}>
							<InputRightElement height="47px" mr={4}>
								<SearchIcon />
							</InputRightElement>
							<Input
								borderColor={"#095FBA"}
								placeholder="Neurological"
								bgColor={"#FFFFFF"}
								type={"text"}
								name="neurological"
								mr={4}
							/>
						</InputGroup>
						{errors().neurological && (
							<FormErrorMessage>{errors().neurological}</FormErrorMessage>
						)}
					</FormControl>
					<FormLabel mb={2} my={3} color={"#095FBA"}>
						Psychiatric
					</FormLabel>
					<FormControl isInvalid={(errors().psychiatric || []).length !== 0}>
						<InputGroup size="lg" alignItems={"center"}>
							<InputRightElement height="47px" mr={4}>
								<SearchIcon />
							</InputRightElement>
							<Input
								borderColor={"#095FBA"}
								placeholder="Psychiatric"
								bgColor={"#FFFFFF"}
								type={"text"}
								name="psychiatric"
								mr={4}
							/>
						</InputGroup>
						{errors().psychiatric && (
							<FormErrorMessage>{errors().psychiatric}</FormErrorMessage>
						)}
					</FormControl>
					<FormLabel mb={2} my={3} color={"#095FBA"}>
						Endocrine
					</FormLabel>
					<FormControl isInvalid={(errors().endocrine || []).length !== 0}>
						<InputGroup size="lg" alignItems={"center"}>
							<InputRightElement height="47px" mr={4}>
								<SearchIcon />
							</InputRightElement>
							<Input
								borderColor={"#095FBA"}
								placeholder="Endocrine"
								bgColor={"#FFFFFF"}
								type={"text"}
								name="endocrine"
								mr={4}
							/>
						</InputGroup>
						{errors().endocrine && (
							<FormErrorMessage>{errors().endocrine}</FormErrorMessage>
						)}
					</FormControl>
					<FormLabel mb={2} my={3} color={"#095FBA"}>
						Heme / Onc
					</FormLabel>
					<FormControl isInvalid={(errors().heme || []).length !== 0}>
						<InputGroup size="lg" alignItems={"center"}>
							<InputRightElement height="47px" mr={4}>
								<SearchIcon />
							</InputRightElement>
							<Input
								borderColor={"#095FBA"}
								placeholder="Heme / Onc"
								bgColor={"#FFFFFF"}
								type={"text"}
								name="heme"
								mr={4}
							/>
						</InputGroup>
						{errors().heme && (
							<FormErrorMessage>{errors().heme}</FormErrorMessage>
						)}
					</FormControl>
					<FormLabel mb={2} my={3} color={"#095FBA"}>
						Infectious
					</FormLabel>
					<FormControl isInvalid={(errors().infectious || []).length !== 0}>
						<InputGroup size="lg" alignItems={"center"}>
							<InputRightElement height="47px" mr={4}>
								<SearchIcon />
							</InputRightElement>
							<Input
								borderColor={"#095FBA"}
								placeholder="Infectious"
								bgColor={"#FFFFFF"}
								type={"text"}
								name="infectious"
								mr={4}
							/>
						</InputGroup>
						{errors().infectious && (
							<FormErrorMessage>{errors().infectious}</FormErrorMessage>
						)}
					</FormControl>

					<FormLabel mb={2} my={3} color={"#095FBA"}>
						{"Comments"}
					</FormLabel>
					<FormControl isInvalid={(errors().comments || []).length !== 0}>
						<Textarea
							mr={3}
							bgColor={"#FFFFFF"}
							name={"comments"}
							borderColor={"#095FBA"}
							placeholder={"Comments"}
						/>
						{errors().comments && (
							<FormErrorMessage>{errors().comments}</FormErrorMessage>
						)}
					</FormControl>
					<SubmitButton loading={isSubmitting()} />
				</Box>
			</form>
		</div>
	);
};
