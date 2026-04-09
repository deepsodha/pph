import { Box, Button, Image, Input, Link, Text } from "@chakra-ui/react";
import React from "react";

const Footer = () => {
	return (
		<div>
			<Box
				display={"flex"}
				flexDirection={"column"}
				bgColor={"#F6FBFD"}
				width={"100%"}
				//pt={20}
			>
				<Box
					justifyContent={{ base: "center", md: "space-around" }}
					alignItems={{ base: "center", md: "flex-start" }}
					display={"flex"}
					flexDirection={{ base: "column", md: "row" }}
					flexWrap={"wrap"}
					mt={"3%"}
					mb={"3%"}
				>
					<Box ml={"4%"}>
						<Box display={"flex"}>
							<Box ml={{ base: "5%", md: "0%" }}>
								<Image src={"findmydoc logo.svg"} />
							</Box>
						</Box>

						<Box height={144} width={232} mt={"7%"}>
							<Text
								fontSize="16px"
								fontWeight={500}
								color="#888888"
								textAlign={{ base: "center", md: "left" }}
							>
								FindmyDoc simplifies the search for the perfect healthcare
								professional. Connect with skilled physicians and specialists
								effortlessly for your well-being.
							</Text>
						</Box>
					</Box>
					<Box
						display={"flex"}
						flexDirection={"column"}
						mt={{ base: "5%", sm: "5%", md: "1%" }}
					>
						<Box>
							<Text
								fontSize={18}
								fontWeight={700}
								color="#51A7D6"
								textAlign={"center"}
							>
								Menu
							</Text>
							<Link href="/">
								<Text
									fontSize={15}
									fontWeight={500}
									color="#888888"
									mt={5}
									textAlign={"center"}
								>
									Home
								</Text>
							</Link>
							<Text
								fontSize={15}
								fontWeight={500}
								color="#888888"
								mt={5}
								textAlign={"center"}
							>
								About Us
							</Text>
						</Box>
					</Box>

					<Box
						display={"flex"}
						flexDirection={"column"}
						mt={{ base: "5%", sm: "5%", md: "1%" }}
					>
						<Text
							fontSize={18}
							fontWeight={700}
							color="#51A7D6"
							textAlign={"center"}
						>
							Quick Links
						</Text>
						<Text
							fontSize={15}
							fontWeight={500}
							color="#888888"
							mt={5}
							textAlign={"center"}
						>
							Pi-Health
						</Text>

						<Text
							fontSize={15}
							fontWeight={500}
							color="#888888"
							mt={5}
							textAlign={"center"}
						>
							Rx.Pi-Health
						</Text>
					</Box>

					<Box
						display={"flex"}
						flexDirection={"column"}
						mt={{ base: "5%", sm: "5%", md: "1%" }}
					>
						<Box>
							<Text
								fontSize={18}
								fontWeight={700}
								color="#51A7D6"
								textAlign={{ base: "center", md: "left" }}
							>
								Contact
							</Text>

							<Box display={"flex"} flexDirection={"row"} gap={2} mt={15}>
								<Image src={"mail.svg"} />
								<Text
									fontSize={18}
									fontWeight={500}
									color="#888888"
									textAlign={"center"}
								>
									support@findmydoc.com
								</Text>
							</Box>
						</Box>
					</Box>

					<Box mt={{ base: "5%", sm: "5%", md: "1%" }}>
						<Text
							fontSize={18}
							fontWeight={700}
							color="#51A7D6"
							textAlign={{ base: "center", md: "left" }}
							ml={{ base: "0%", md: "25%", lg: "0%" }}
						>
							Lets Work Together
						</Text>

						<Box
							display={"flex"}
							flexDirection={"row"}
							mt={"5%"}
							width={{ base: "80%", md: "100%" }}
							ml={{ base: "11%", md: "25%", lg: "0%" }}
						>
							<Input
								variant={"outline"}
								borderColor={"#51A7D6"}
								type={"text"}
								placeholder="Enter Email Address"
								_placeholder={{ color: "black" }}
							/>

							<Button bgColor={"#FED337"} color={"white"} ml={"3%"}>
								Subscribe
							</Button>
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
				bgColor={"#F6FBFD"}
			>
				<Text
					textAlign={"center"}
					color={"#8890A4"}
					fontWeight={"500"}
					p={"1%"}
				>
					© Copyright 2023 - FindmyDoc, designed and developed by fuzzy cloud
				</Text>
			</Box>
		</div>
	);
};

export default Footer;
