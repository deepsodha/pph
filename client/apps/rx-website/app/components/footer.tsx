import { Box, Image, Text } from "@chakra-ui/react";
import React from "react";

const Footer = () => {
	return (
		<div>
			<>
				<Box
					display={"flex"}
					flexDirection={{ base: "column", md: "row" }}
					justifyContent={{ base: "center", md: "space-around" }}
					alignItems={{ base: "center", md: "flex-start" }}
					mt={"5%"}
					mr={{ base: "0%", md: "0%", lg: "10%" }}
					ml={{ base: "0%", md: "0%", lg: "10%" }}
				>
					<Box>
						<Image src="/Logo - 2 3.svg" />
						<Text mt="15%" fontSize={"20px"} fontWeight={700}>
							Rx. Pihealth
						</Text>
					</Box>

					<Box
						display={"flex"}
						flexDirection={"column"}
						ml={{ base: "15%", md: "0%" }}
					>
						<Text
							fontWeight={500}
							fontSize={"16px"}
							mb="10%"
							ml={{ base: "10%", md: "0%" }}
							pt={{ base: "5%", md: "0%" }}
						>
							Contact us
						</Text>
						<Box display={"flex"} flexDirection={"row"}>
							<Image src="/Gmail Icon .svg" mr={"2%"} mb="10%" />
							<Text fontWeight={400} fontSize={"14px"} mt="1.5%">
								support@Rx.pihealth.com
							</Text>
						</Box>
						<Box display={"flex"} flexDirection={"row"}>
							<Image src="/call footer icon.svg" mr={"2%"} mb="10%" />
							<Text fontWeight={400} fontSize={"14px"} mt="1.5%">
								+91 2717 453 330
							</Text>
						</Box>
						<Box display={"flex"} flexDirection={"row"}>
							<Image src="/Facebook icon.svg" mr={"2%"} mb="10%" />
							<Text fontWeight={400} fontSize={"14px"} mt="1.5%">
								Rx.Pihealth
							</Text>
						</Box>
						<Box display={"flex"} flexDirection={"row"}>
							<Image src="/Twitter icon.svg" mr={"2%"} mb="10%" />
							<Text fontWeight={400} fontSize={"14px"} mt="1.5%">
								Rx.Pihealth
							</Text>
						</Box>
					</Box>

					<Box
						display={"flex"}
						flexDirection={"column"}
						alignItems={{ base: "center", md: "flex-start" }}
						justifyContent={{ base: "center", md: "start" }}
					>
						<Text fontWeight={500} fontSize={"16px"} mb="10%">
							Quick Links
						</Text>
						<Text fontWeight={500} fontSize={"16px"} mb="10%" color={"#888888"}>
							pi - Health
						</Text>
						<Text fontWeight={500} fontSize={"16px"} mb="10%" color={"#888888"}>
							Findmydoc
						</Text>
					</Box>

					<Box
						display={"flex"}
						flexDirection={"column"}
						alignItems={{ base: "center", md: "flex-start" }}
						justifyContent={{ base: "center", md: "start" }}
					>
						<Text fontWeight={500} fontSize={"16px"} mb="10%">
							Drugs & Suppliments
						</Text>
						<Text fontWeight={500} fontSize={"16px"} mb="10%" color={"#888888"}>
							Find Your Drug
						</Text>
						<Text fontWeight={500} fontSize={"16px"} mb="10%" color={"#888888"}>
							National Drug Code
						</Text>
						<Text fontWeight={500} fontSize={"16px"} mb="10%" color={"#888888"}>
							Find Pharmacy
						</Text>
					</Box>
				</Box>

				<Box
					display={"flex"}
					flexDirection={"row"}
					width={"100%"}
					border={"1px solid #8890A4"}
					justifyContent={"center"}
				>
					<Text textAlign={"center"} fontSize={"16px"} fontWeight={"500"} p={5}>
						Copyright © Rx - Pihealth | Designed by FuzzyCloud
					</Text>
				</Box>
			</>
		</div>
	);
};

export default Footer;
