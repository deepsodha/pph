import { Box, FormControl, Input, Select } from "@chakra-ui/react";
import { SubmitButton } from "../../../apps/medistore/src/components/submitButton";
export const DrugForm = () => {
	return (
		<div>
			<form>
				<FormControl mt={7}>
					<Input
						type="text"
						name="drug_name"
						_focus={{
							borderColor: "#1A998E",
						}}
						focusBorderColor="#1A998E"
						borderColor="#1A998E"
						placeholder={"Drug Name"}
					/>
				</FormControl>

				<FormControl mt={7}>
					<Select
						placeholder="Drug Type"
						name="drug_type"
						_focus={{
							borderColor: "#1A998E",
						}}
						focusBorderColor="#1A998E"
						borderColor="#1A998E"
					>
						<option value="Type1">Type 1</option>
						<option value="Type2">Type 2</option>
					</Select>
				</FormControl>
				<FormControl mt={7}>
					<Input
						type="number"
						name="price"
						_focus={{
							borderColor: "#1A998E",
						}}
						focusBorderColor="#1A998E"
						borderColor="#1A998E"
						placeholder={"Price"}
					/>
				</FormControl>
				<FormControl mt={7}>
					<Input
						type="text"
						name="quantity_in_stock"
						bgColor="#FFFFFF"
						borderColor="#1A998E"
						_focus={{
							borderColor: "#1A998E",
						}}
						focusBorderColor="#1A998E"
						placeholder="Quantity in stock"
					/>
				</FormControl>
				<Box mt="50%">
					<SubmitButton />
				</Box>
			</form>
		</div>
	);
};

export default DrugForm;
