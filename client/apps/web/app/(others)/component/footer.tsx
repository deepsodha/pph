import { Box, Button, Image, Input, Text } from "@chakra-ui/react";
import Link from "next/link";

import React from "react";

const Footer = () => {
	return (
		<div>
			<Box
				display={"flex"}
				flexDirection={"row"}
				bgColor={"#DDEDFA"}
				pb={"5%"}
				pt={"5%"}
				justifyContent="space-around"
				flexWrap="wrap"
			>
				<Box
					justifyContent={{ base: "center", md: "space-around" }}
					// mt={"5%"}
					// ml={{ base: "0%", md: "0%", lg: "5%" }}
				>
					<Box
						display={"flex"}
						flexDirection={"column"}
						alignItems={{ base: "center", lg: "start" }}
					>
						<Box
						// ml={{ base: "35%", md: "0%" }}
						>
							<Image src={"Pi-health logo (2).svg"} />
						</Box>

						<Box pt="10%">
							<Text
								fontSize={{ base: 20, md: 24 }}
								fontWeight={400}
								width={{ md: 300 }}
								textAlign={{ base: "center", lg: "left" }}
								color="#1C1C1C"
								mb="10%"
							>
								“A Simple Story About <br />
								The Doctorate Medical Center & Health Care Foundation”
							</Text>
						</Box>
					</Box>
				</Box>

				<Box
					display={"flex"}
					flexDirection={"column"}
					// mt={"5%"}
					ml={{ base: "0%", md: "5%" }}
					//justifyContent={{ base: "center", sm: "center", md: "space-around" }}
					//flexWrap="wrap"
					justifyContent={{ base: "center", md: "space-around" }}
				>
					<Box
						display={{ base: "flex", md: "flex" }}
						flexDirection={{ base: "column", md: "row" }}
					>
						<Box
							display={{ base: "flex", md: "flex" }}
							flexDirection={{ base: "row", md: "row" }}
							mr={{ base: "0%", md: "5%" }}
							justifyContent={{ base: "center" }}
							gap={2}
							mb={{ base: "2%", md: "0%" }}
						>
							<Box>
								<Image src={"Phone Icon.svg"} />
							</Box>
							<Box>
								<Text
									fontSize={{ base: 16, md: 18 }}
									fontWeight={500}
									color="#888888"
								>
									(+91) 123 - 4567 - 900
								</Text>
							</Box>
						</Box>

						<Box
							display={{ base: "flex", md: "flex" }}
							flexDirection={{ base: "row", md: "row" }}
							gap={2}
							mr={{ base: "0%", md: "5%" }}
							justifyContent={{ base: "center" }}
							mb={{ base: "2%", md: "0%" }}
						>
							<Box>
								<Image src={"Gmail Icon.svg"} />
							</Box>

							<Text
								fontSize={{ base: 16, md: 18 }}
								fontWeight={500}
								color="#888888"
							>
								support@pihealth.com
							</Text>
						</Box>
						<Box
							display={"flex"}
							alignItems={"center"}
							justifyContent={"center"}
						>
							<Box
								mr={{ base: "0%", md: "5%" }}
								ml={{ base: "2%", md: "0%" }}
								mb={{ base: "22px", lg: "0px" }}
								alignItems={"center"}
								justifyContent={"center"}
							>
								<Image src={"Facebook Icon.svg"} boxSize={{ base: 6, md: 7 }} />
							</Box>

							<Box
								mr={{ lg: "5%" }}
								ml={{ base: "32px", lg: "32px" }}
								mb={{ base: "22px", lg: "0px" }}
							>
								<Image src={"Twitter Icon.svg"} boxSize={{ base: 6, md: 7 }} />
							</Box>
						</Box>
					</Box>

					<Box
						display={{ base: "flex", md: "flex" }}
						flexDirection={{ base: "column", md: "row" }}
						justifyContent={{ base: "center", md: "space-between" }}
						mt={{ base: 0, sm: 0, md: 10 }}
						ml={{ base: "2%", sm: 0, md: "0%" }}
					>
						<Box
							display={{ base: "flex", md: "flex" }}
							flexDirection={{ base: "column", md: "column" }}
							alignItems={{ base: "center", lg: "start" }}
							mr={{ base: 0, sm: 0, md: 10 }}
							mb={{ base: "2%", md: "0%" }}
							pr={{ base: 0, sm: 0, md: 0 }}
						>
							<Text
								fontSize={{ base: 16, md: 18 }}
								fontWeight={700}
								color="#1C1C1C"
								textAlign={{ base: "center" }}
							>
								Quick Links
							</Text>
							<Text
								fontSize={{ base: 14, md: 15 }}
								fontWeight={500}
								color="#888888"
								mt={5}
								textAlign={{ base: "center" }}
							>
								Pi-Health
							</Text>
							<Text
								fontSize={{ base: 14, md: 15 }}
								fontWeight={500}
								color="#888888"
								mt={5}
								textAlign={{ base: "center" }}
							>
								Rx.Pi-Health
							</Text>
						</Box>

						<Box
							display={{ base: "flex", md: "flex" }}
							flexDirection={{ base: "column", md: "column" }}
							mr={{ base: "5%", sm: 0, md: 14 }}
							mb={{ base: "2%", md: "0%" }}
							alignItems={{ base: "center", lg: "start" }}
						>
							<Text
								fontSize={{ base: 16, md: 18 }}
								fontWeight={700}
								color="#1C1C1C"
								textAlign={{ base: "center" }}
							>
								Explore
							</Text>
							<Text
								fontSize={{ base: 14, md: 15 }}
								fontWeight={500}
								color="#888888"
								mt={5}
								textAlign={{ base: "center" }}
							>
								All Products
							</Text>
							<Link href="/price">
								<Text
									fontSize={{ base: 14, md: 15 }}
									fontWeight={500}
									color="#888888"
									mt={5}
									textAlign={{ base: "center" }}
								>
									Pricing
								</Text>
							</Link>
							<Link href="/contact-us">
								<Text
									fontSize={{ base: 14, md: 15 }}
									fontWeight={500}
									color="#888888"
									mt={5}
									textAlign={{ base: "center" }}
								>
									Contact us
								</Text>
							</Link>
						</Box>

						<Box
							display={{ base: "flex", md: "flex" }}
							flexDirection="column"
							mt={{ base: 0, sm: 0, md: 0 }}
							mr={{ base: 0, sm: 0, md: 0 }}
							mb={{ base: "2%", md: "0%" }}
						>
							<Text
								fontSize={{ base: 16, md: 18 }}
								fontWeight={700}
								color="#1C1C1C"
								textAlign={{ base: "center", md: "left" }}
								// mr={{ base: 11, sm: 20, md: 0 }}
							>
								Book an appointment
							</Text>
							<Box>
								<Text
									fontSize={{ base: 14, md: 15 }}
									fontWeight={500}
									color="#888888"
									mt={"5%"}
									ml={{ base: "10%", md: 0 }}
									width={{ base: "80%", md: "80%" }}
									textAlign={{ base: "center", md: "left" }}
								>
									It is a long established fact that a reader will be distracted
								</Text>
							</Box>
							<Box
								display={{ base: "flex", md: "flex" }}
								flexDirection={{ base: "column", md: "row" }}
								width={{ base: "80%", md: "80%" }}
								mt={{ base: "4%", md: "5%" }}
								ml={{ base: "10%", md: 0 }}
								color={"black"}
							>
								<Input
									variant={"outline"}
									borderColor={"#51A7D6"}
									type={"text"}
									color={"black"}
									_placeholder={{ color: "gray.500" }}
									placeholder="Enter Email Address"
								/>
								<Button
									bgColor={"#095FBA"}
									color={"white"}
									mt={{ base: "5%", md: 0 }}
									ml={{ base: "0%", md: "3%" }}
									_hover={{ bg: "white", color: "#095FBA" }}
								>
									Submit
								</Button>
							</Box>
						</Box>
					</Box>
				</Box>
			</Box>

			<Box
				display={"flex"}
				flexDirection={"row"}
				width={"100%"}
				border={"1px solid #8890A4"}
				justifyContent={"center"}
			>
				<Text
					textAlign={"center"}
					color={"#8890A4"}
					fontWeight={"500"}
					p={"2%"}
				>
					Copyright © Pi - Health | Designed by FuzzyCloud
				</Text>
			</Box>
		</div>
	);
};

export default Footer;
