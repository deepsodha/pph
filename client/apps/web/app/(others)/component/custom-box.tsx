import { Box, Text, Image, useColorModeValue } from "@chakra-ui/react";
import type React from "react";

interface CustomBox {
	icon: string;
	title: string;
	description: string;
	id: number;
}

const CustomBox: React.FC<CustomBox> = ({ icon, title, description }) => {
	const color = useColorModeValue("black", "white");
	return (
		<Box
			display={"flex"}
			alignItems={{ base: "center", lg: "start" }}
			border={"1px solid #095FBA"}
			flexDirection={"column"}
			borderRadius={"10px"}
		>
			<Box ml={"5%"} mt={"5%"}>
				<Image src={icon} />
			</Box>
			<Box ml={"5%"} mt={"3%"} textAlign={{ base: "center", lg: "start" }}>
				<Text fontWeight={600} color={color} fontSize={"24px"}>
					{title}
				</Text>
			</Box>
			<Box m={"5%"} textAlign={{ base: "center", lg: "start" }}>
				<Text fontWeight={400} fontSize={"16px"} width={"100%"}>
					{description}
				</Text>
			</Box>
		</Box>
	);
};
export default CustomBox;
