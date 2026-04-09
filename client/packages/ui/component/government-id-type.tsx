import { Select } from "@chakra-ui/react";

type SelectGenderTypeProps = {
	name: string;
};
const SelectGovernmentIdType = ({ name }: SelectGenderTypeProps) => {
	return (
		<div>
			<Select name={name} bgColor="#FFFFFF" borderColor="#095FBA">
				<option selected hidden disabled value="">
					Select Government ID Type
				</option>
				<option value="AadhaarCard">Aadhaar Card</option>
				<option value="DrivingLicense">Driving License</option>
				<option value="Passport">Passport</option>
			</Select>
		</div>
	);
};

export default SelectGovernmentIdType;
