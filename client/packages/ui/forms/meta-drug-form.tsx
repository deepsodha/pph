import {
	Center,
	FormLabel,
	FormControl,
	Input,
	Box,
	FormErrorMessage,
} from "@chakra-ui/react";
import { SubmitButton } from "../component";
import { useForm } from "@felte/react";
import type { Drugs } from "@repo/types/dto";
import { validator } from "@felte/validator-zod";
import reporterDom from "@felte/reporter-dom";
import type { z } from "zod";
import { DrugaddSchema } from "@repo/types/validation";

export type DrugFormProps = {
	onSubmit: (p: Drugs) => void;
	DrugId?: string;
	edit?: boolean;
	initialValues?: Drugs | undefined;
};

export const MetaDrugForm = (props: DrugFormProps) => {
	const { form, errors, isSubmitting } = useForm<z.infer<typeof DrugaddSchema>>(
		{
			onSubmit: (values) => {
				props.onSubmit(values);
			},
			initialValues: {
				brand: props.initialValues?.brand,
				generic: props.initialValues?.generic,
				details: props.initialValues?.details,
				category: props.initialValues?.category,
				side_effects: props.initialValues?.side_effects,
				drugsdose_info: props.initialValues?.drugsdose_info,
				precautions: props.initialValues?.precautions,
				manufacturer_name: props.initialValues?.manufacturer_name,
				medicines: props.initialValues?.medicines,
				contra_indications: props.initialValues?.contra_indications,
				diseases: props.initialValues?.diseases,
				interactions: props.initialValues?.interactions,
				contains: props.initialValues?.contains,
			},
			extend: [validator({ schema: DrugaddSchema }), reporterDom()],
		},
	);

	return (
		<div>
			<form ref={form}>
				<Center>
					<Box width={{ md: "80%", base: "90%", lg: "70%" }}>
						<FormLabel mb={2} my={3} color={"#095FBA"}>
							Brand
						</FormLabel>
						<FormControl mt={2} isInvalid={(errors().brand || []).length !== 0}>
							<Input
								type="text"
								name="brand"
								bgColor={"#FFFFFF"}
								borderColor={"#095FBA"}
								placeholder="Enter Brand"
							/>
							{errors().brand && (
								<FormErrorMessage>{errors().brand}</FormErrorMessage>
							)}
						</FormControl>
						<FormLabel mb={2} my={3} color={"#095FBA"}>
							Generic
						</FormLabel>
						<FormControl
							mt={2}
							isInvalid={(errors().generic || []).length !== 0}
						>
							<Input
								type="text"
								name="generic"
								bgColor={"#FFFFFF"}
								borderColor={"#095FBA"}
								placeholder="Enter Generic"
							/>
							{errors().generic && (
								<FormErrorMessage>{errors().generic}</FormErrorMessage>
							)}
						</FormControl>
						<FormLabel mb={2} my={3} color={"#095FBA"}>
							Details
						</FormLabel>
						<FormControl
							mt={2}
							isInvalid={(errors().details || []).length !== 0}
						>
							<Input
								type="text"
								name="details"
								bgColor={"#FFFFFF"}
								borderColor={"#095FBA"}
								placeholder="Enter Details"
							/>
							{errors().details && (
								<FormErrorMessage>{errors().details}</FormErrorMessage>
							)}
						</FormControl>
						<FormLabel mb={2} my={3} color={"#095FBA"}>
							Category
						</FormLabel>
						<FormControl
							mt={2}
							isInvalid={(errors().category || []).length !== 0}
						>
							<Input
								type="text"
								name="category"
								bgColor={"#FFFFFF"}
								borderColor={"#095FBA"}
								placeholder="Enter Category"
							/>
							{errors().category && (
								<FormErrorMessage>{errors().category}</FormErrorMessage>
							)}
						</FormControl>
						<FormLabel mb={2} my={3} color={"#095FBA"}>
							Side Effects
						</FormLabel>
						<FormControl
							mt={2}
							isInvalid={(errors().side_effects || []).length !== 0}
						>
							<Input
								type="text"
								name="side_effects"
								bgColor={"#FFFFFF"}
								borderColor={"#095FBA"}
								placeholder="Enter Side Effects"
							/>
							{errors().side_effects && (
								<FormErrorMessage>{errors().side_effects}</FormErrorMessage>
							)}
						</FormControl>
						<FormLabel mb={2} my={3} color={"#095FBA"}>
							Drugsdose Info
						</FormLabel>
						<FormControl
							mt={2}
							isInvalid={(errors().drugsdose_info || []).length !== 0}
						>
							<Input
								type="text"
								name="drugsdose_info"
								bgColor={"#FFFFFF"}
								borderColor={"#095FBA"}
								placeholder="Enter Drugsdose Info"
							/>
							{errors().drugsdose_info && (
								<FormErrorMessage>{errors().drugsdose_info}</FormErrorMessage>
							)}
						</FormControl>
						<FormLabel mb={2} my={3} color={"#095FBA"}>
							Precautions
						</FormLabel>
						<FormControl
							mt={2}
							isInvalid={(errors().precautions || []).length !== 0}
						>
							<Input
								type="text"
								name="precautions"
								bgColor={"#FFFFFF"}
								borderColor={"#095FBA"}
								placeholder="Enter Precautions"
							/>
							{errors().precautions && (
								<FormErrorMessage>{errors().precautions}</FormErrorMessage>
							)}
						</FormControl>
						<FormLabel mb={2} my={3} color={"#095FBA"}>
							Manufacturer Name
						</FormLabel>
						<FormControl
							mt={2}
							isInvalid={(errors().manufacturer_name || []).length !== 0}
						>
							<Input
								type="text"
								name="manufacturer_name"
								bgColor={"#FFFFFF"}
								borderColor={"#095FBA"}
								placeholder="Enter Manufacturer Name"
							/>
							{errors().manufacturer_name && (
								<FormErrorMessage>
									{errors().manufacturer_name}
								</FormErrorMessage>
							)}
						</FormControl>
						<FormLabel mb={2} my={3} color={"#095FBA"}>
							Medicines
						</FormLabel>
						<FormControl
							mt={2}
							isInvalid={(errors().medicines || []).length !== 0}
						>
							<Input
								type="text"
								name="medicines"
								bgColor={"#FFFFFF"}
								borderColor={"#095FBA"}
								placeholder="Enter Medicines"
							/>
							{errors().medicines && (
								<FormErrorMessage>{errors().medicines}</FormErrorMessage>
							)}
						</FormControl>
						<FormLabel mb={2} my={3} color={"#095FBA"}>
							Contra Indications
						</FormLabel>
						<FormControl
							mt={2}
							isInvalid={(errors().contra_indications || []).length !== 0}
						>
							<Input
								type="text"
								name="contra_indications"
								bgColor={"#FFFFFF"}
								borderColor={"#095FBA"}
								placeholder="Enter Contra Idications"
							/>
							{errors().contra_indications && (
								<FormErrorMessage>
									{errors().contra_indications}
								</FormErrorMessage>
							)}
						</FormControl>
						<FormLabel mb={2} my={3} color={"#095FBA"}>
							Diseases
						</FormLabel>
						<FormControl
							mt={2}
							isInvalid={(errors().diseases || []).length !== 0}
						>
							<Input
								type="text"
								name="diseases"
								bgColor={"#FFFFFF"}
								borderColor={"#095FBA"}
								placeholder="Enter Diseases"
							/>
							{errors().diseases && (
								<FormErrorMessage>{errors().diseases}</FormErrorMessage>
							)}
						</FormControl>
						<FormLabel mb={2} my={3} color={"#095FBA"}>
							Interactions
						</FormLabel>
						<FormControl
							mt={2}
							isInvalid={(errors().interactions || []).length !== 0}
						>
							<Input
								type="text"
								name="interactions"
								bgColor={"#FFFFFF"}
								borderColor={"#095FBA"}
								placeholder="Enter Interactions"
							/>
							{errors().interactions && (
								<FormErrorMessage>{errors().interactions}</FormErrorMessage>
							)}
						</FormControl>
						<FormLabel mb={2} my={3} color={"#095FBA"}>
							Contains
						</FormLabel>
						<FormControl
							mt={2}
							isInvalid={(errors().contains || []).length !== 0}
						>
							<Input
								type="text"
								name="contains"
								bgColor={"#FFFFFF"}
								borderColor={"#095FBA"}
								placeholder="Contains"
							/>
							{errors().contains && (
								<FormErrorMessage>{errors().contains}</FormErrorMessage>
							)}
						</FormControl>
						<SubmitButton loading={isSubmitting()} />
					</Box>
				</Center>
			</form>
		</div>
	);
};
