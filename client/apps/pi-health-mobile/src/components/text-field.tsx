import { Text } from "@chakra-ui/react";

type TextFieldProps = {
	heading: string;
	result: string | null | undefined | number | string[];
};
const TextField = ({ heading, result }: TextFieldProps) => {
	return (
		<div>
			<Text marginLeft="6px" fontSize="14px" mb="2%" fontWeight={500}>
				<Text as="span" color="#095FBA">
					{heading}
				</Text>{" "}
				<Text fontSize="14px" fontWeight={400} as="span" color="#121224">
					{result}
				</Text>
			</Text>
		</div>
	);
};

export default TextField;
