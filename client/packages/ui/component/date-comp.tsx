import { Input } from "@chakra-ui/react";

type dateProps = {
	name: string;
	type: string;
	placeholder: string;
	min?: string;
	max?: string;
};

const DateComponent = ({ name, type, placeholder, min, max }: dateProps) => {
	return (
		<Input
			type={type}
			name={name}
			bgColor={"#FFFFFF"}
			borderColor={"#095FBA"}
			placeholder={placeholder}
			min={min}
			max={max}
		/>
	);
};
export default DateComponent;
