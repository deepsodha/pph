import { EditIcon } from "@chakra-ui/icons";
import { Box, Center } from "@chakra-ui/react";

const NotesEditIcon = () => {
	return (
		<Box
			width="100%"
			border="1px"
			borderColor="#095FBA"
			bgColor="#FFFFFF"
			height="40px"
			borderRadius="md"
			alignItems="center"
			//   onClick={() => Link(`/medication/edit/${patientId}/${medicationId}`)}
		>
			<Center>
				<EditIcon mt={2.5} pl={0.5} width="80px" height={"18px"} />
			</Center>
		</Box>
	);
};

export default NotesEditIcon;
