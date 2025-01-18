import { Select } from "@chakra-ui/react";

type SelectPhoneNumberTypeProps = {
	name: string;
};
const SelectPhoneNumberType = ({ name }: SelectPhoneNumberTypeProps) => {
	return (
		<div>
			<Select name={name} bgColor="#FFFFFF" borderColor="#095FBA">
				<option selected hidden disabled value="">
					Select Phone Number Type
				</option>
				<option value="Mobile">Mobile</option>
				<option value="Home">Home</option>
				<option value="Office">Office</option>
			</Select>
		</div>
	);
};

export default SelectPhoneNumberType;
