import { Select } from "@chakra-ui/react";

type SelectGenderTypeProps = {
	name: string;
};
const SelectGenderType = ({ name }: SelectGenderTypeProps) => {
	return (
		<div>
			<Select
				placeholder="Gender"
				name={name}
				bgColor="#FFFFFF"
				borderColor="#095FBA"
			>
				<option value="Male">Male</option>
				<option value="Female">Female</option>
				<option value="Other">Other</option>
				<option value="Unknown">Unknown</option>
			</Select>
		</div>
	);
};

export default SelectGenderType;
