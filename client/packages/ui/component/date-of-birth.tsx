import { Input } from "@chakra-ui/react";

type dateOfBirthProps = {
	name: string;
};

const DateOfBirth = ({ name }: dateOfBirthProps) => {
	const today = new Date();
	today.setDate(today.getDate() - 1);
	const yesterday = today.toISOString().slice(0, 10);
	return (
		<Input
			type="date"
			name={name}
			bgColor={"#FFFFFF"}
			borderColor={"#095FBA"}
			placeholder="Date of Birth"
			max={yesterday}
		/>
	);
};
export default DateOfBirth;
