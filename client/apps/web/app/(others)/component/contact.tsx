import { Box, Button, Link, Text } from "@chakra-ui/react";

const Contact: React.FC = () => {
	return (
		<div>
			<>
				<Box
					display="flex"
					flexDirection={{ md: "column", lg: "row", base: "column" }}
					justifyContent={"space-between"}
					bgColor="#095FBA"
					m="4%"
					width={"80%"}
					p="2%"
					ml={"9%"}
					alignItems={"center"}
					borderRadius="6px"
				>
					<Box ml={"3%"}>
						<Text
							color={"white"}
							fontSize={{ sm: "29px", md: "35px", lg: "38px" }}
							fontWeight={500}
							textAlign={"center"}
						>
							For Price related queries kindly contact Us
						</Text>
					</Box>
					<Box mr={"3%"}>
						<Link href="/contact-us">
							{" "}
							<Button
								color="#095FBA"
								bgColor="white"
								_hover={{ bg: "white", color: "#095FBA" }}
							>
								Contact Us
							</Button>
						</Link>
					</Box>
				</Box>
			</>
		</div>
	);
};
export default Contact;
