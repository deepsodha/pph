import { Box, Button, Text } from "@chakra-ui/react";
import Link from "next/link";
import React from "react";

const Abovefooter = () => {
	return (
		<div>
			<Box bgColor={"#095FBA"}>
				<Text
					fontWeight={700}
					fontSize={"40px"}
					color="white"
					textAlign={"center"}
					p={"3%"}
				>
					Take the guesswork out of your medication needs
				</Text>

				<Text
					fontWeight={400}
					fontSize={"24px"}
					color="white"
					textAlign={"center"}
				>
					Discover, Compare, and choose the right drugs for you
				</Text>

				<Box
					display={"flex"}
					justifyContent={"center"}
					alignItems={"center"}
					flexDirection={{ base: "column", md: "row" }}
					p={"3%"}
					pb={{ base: "10%", lg: "5%" }}
				>
					<Button
						variant={"outline"}
						bgColor={"#095FBA"}
						size={{ base: "sm", md: "md", lg: "lg" }}
						mb={{ base: "5%", md: "0%" }}
						color={"white"}
						_hover={{
							backgroundColor: "white",
							color: "#095FBA",
							borderColor: "#095FBA",
							border: "1px",
						}}
					>
						<Link href="/">Start Exploring your resources </Link>
					</Button>
					<Button
						bgColor={"white"}
						color={"#095FBA"}
						size={{ base: "sm", md: "md", lg: "lg" }}
						ml={"2%"}
						_hover={{
							backgroundColor: "#095FBA",
							color: "white",
							borderColor: "#095FBA",
							border: "1px",
						}}
					>
						<Link href={"/contact-us"}>Contact Us</Link>
					</Button>
				</Box>
			</Box>
		</div>
	);
};

export default Abovefooter;
