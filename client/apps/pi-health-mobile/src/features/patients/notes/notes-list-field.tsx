import { Text } from "@chakra-ui/react";

type NotestListFieldProps = {
	label: string;
	value: string | number | null;
};
const NotestListField = ({ label, value }: NotestListFieldProps) => {
	return (
		<div>
			<Text fontSize={"16px"}>
				<Text fontWeight={"bold"} display={"inline-block"} marginRight={"5px"}>
					{label}
				</Text>
				{value}
			</Text>
		</div>
	);
};

export default NotestListField;
