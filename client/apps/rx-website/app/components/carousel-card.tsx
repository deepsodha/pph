import { Box, Button, Image, Text } from "@chakra-ui/react";

interface CarouseCard {
	src: string;
	title: string;
	description: string;
	m: string;
	mdm?: string;
}
const CarouselCard: React.FC<CarouseCard> = ({
	src,
	title,
	description,
	m,
	mdm,
}) => (
	<Box
		bgColor={"#F0FAFE"}
		display={"flex"}
		flexDirection={"column"}
		justifyContent="center"
		alignItems={"center"}
		p="0%"
		m="0%"
		width={"50%"}
		borderRadius={"5%"}
	>
		<Image src={src} height="100%" width="100%" p="10%" pb={"8%"} />
		<Box mr={{ base: "0%", md: mdm, lg: m }}>
			<Text
				fontWeight={500}
				fontSize={"24px"}
				textAlign={{ base: "center", md: "left" }}
			>
				{title}
			</Text>
		</Box>
		<Text
			fontWeight={400}
			fontSize={"16px"}
			width="78%"
			textAlign={{ base: "center", md: "left" }}
			ml="0%"
			mt={"2%"}
			mr="4%"
		>
			{description}
		</Text>
		<Box mt={"10%"} mb="10%" mr={{ base: "0%", md: "20%", lg: "30%" }}>
			<Button
				bgColor={"#095FBA"}
				color="white"
				size={"sm"}
				_hover={{
					bgColor: "white",
					color: "#095FBA",
					border: "1px solid #095FBA",
				}}
			>
				Continue Reading
			</Button>
		</Box>
	</Box>
);
export default CarouselCard;
