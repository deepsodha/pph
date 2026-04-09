import { Text } from "@chakra-ui/react";

type NotesListBoldFieldProps = {
	label: string;
	value: string | number | null;
};
const NotesListBoldField = ({ label, value }: NotesListBoldFieldProps) => {
	return (
		<div>
			<Text fontSize={"20px"}>
				<Text
					fontSize={"20px"}
					fontWeight={"bold"}
					display={"inline-block"}
					marginRight={"5px"}
				>
					{label}
				</Text>{" "}
				{value}
			</Text>
		</div>
	);
};

export default NotesListBoldField;
