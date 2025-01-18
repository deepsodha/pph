import { AddIcon } from "@chakra-ui/icons";
import { Box, Center } from "@chakra-ui/react";

const NotesAddIcon = () => {
	return (
		<Box
			width="100%"
			border="1px"
			borderColor="#095FBA"
			bgColor="#FFFFFF"
			height="40px"
			borderRadius="md"
			alignItems="center"

			// onClick={() => Link(`/medication/add/${patientId}`)}
		>
			<Center>
				<AddIcon mt={3} width={"80px"} />
			</Center>
		</Box>
	);
};

export default NotesAddIcon;
