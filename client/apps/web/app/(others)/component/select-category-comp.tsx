import { Box, Img, Text } from "@chakra-ui/react";
const SelectCat = () => {
	return (
		<Box
			// maxW="sm"
			borderWidth="1px"
			borderRadius="lg"
			overflow="hidden"
			// p="6"
			m="2%"
			width={"400px"}
			borderColor={"black"}
			textAlign="center"
			bgColor={"white"}
			color="black"
			// onClick={handleCardClick}
			cursor="pointer"
		>
			<Box>
				<Text
					textAlign={"left"}
					fontSize={"2xl"}
					as="h3"
					size="lg"
					pl={2}
					py={4}
					bgColor={"black"}
					color={"white"}
				>
					Select Category
				</Text>
				<Text
					textAlign={"left"}
					fontSize={"2xl"}
					as="h3"
					size="lg"
					pl={2}
					py={2}
					bgColor={"white"}
					color={"black"}
				>
					Patient Management
				</Text>
				<Text
					textAlign={"left"}
					fontSize={"2xl"}
					as="h3"
					size="lg"
					pl={2}
					py={2}
					bgColor={"white"}
					color={"black"}
				>
					EHR
				</Text>
			</Box>
			<Box mt={"39px"} display={"flex"} justifyContent={"center"}>
				<Img height={"300px"} src="ladydoc.svg" />
			</Box>
		</Box>
	);
};

export default SelectCat;
