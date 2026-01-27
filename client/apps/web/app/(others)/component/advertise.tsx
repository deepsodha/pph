"use client";
import { Box, Image, Text, useColorModeValue } from "@chakra-ui/react";

const Advertise: React.FC = () => {
	const color = useColorModeValue("black", "white");

	return (
		<>
			<Box
				width="100%"
				display={"flex"}
				flexDirection={{ md: "column", base: "column", lg: "row" }}
				mt="3%"
				mb={"5%"}
			>
				<Box
					border="1px"
					borderColor="gray.200"
					width={{ sm: "w-full", lg: "150px" }}
					mt={{ base: "2%", lg: "0px" }}
					textAlign={"center"}
				>
					<Text fontSize={"32px"} color={color}>
						Ads
					</Text>
					<Text>
						Lorem ipsum dolor sit amet, consectetur adipisicing elit. Distinctio
						earum error eius amet sequi veniam deserunt, natus? Illo, fugit.
					</Text>
				</Box>
				<Box
					display={"flex"}
					alignItems={"center"}
					justifyContent={"center"}
					flexDirection={{ md: "column", base: "column", lg: "row" }}
					backgroundColor={"#DDEDFA"}
					pb={{ base: "10%", lg: "0%" }}
					height={{ md: "100%", lg: "auto" }}
					width="100%"
				>
					<Image ml="5%" mr="5%" src="winapp.svg" alt=" " />
					<Box
						display={"flex"}
						flexDirection={"column"}
						justifyContent={"center"}
						alignItems="center"
					>
						<Box height="100%">
							<Box ml={{ base: "3%", lg: "auto" }}>
								<Text
									fontSize={"24px"}
									fontWeight={600}
									color={"black"}
									textAlign={{ base: "center", lg: "left" }}
								>
									Your{" "}
									<Text fontWeight={600} as="span" color="#249053">
										Health
									</Text>{" "}
									On Your Finger Tips
								</Text>
								<Text
									fontSize={"48px"}
									pt="4%"
									fontWeight={600}
									lineHeight={"55px"}
									color={"black"}
									textAlign={{ base: "center", lg: "left" }}
								>
									Your Ultimate{" "}
									<Text
										as="span"
										color="#008037"
										textAlign={{ base: "center", lg: "left" }}
									>
										Health
									</Text>{" "}
									Partner
								</Text>
								<Box
									display={"flex"}
									justifyContent={{ base: "center", lg: "start" }}
								>
									<Text
										fontSize={"16px"}
										fontWeight={600}
										width={"70%"}
										mt="4%"
										lineHeight={"20px"}
										color={"black"}
										textAlign={{ base: "center", lg: "left" }}
									>
										Pi - Health Pi - Health Pi - Health Pi - Health Pi - Health
										Pi - Health Pi - Health Pi - Health Pi - Health Pi - Health
									</Text>
								</Box>
							</Box>

							<Box
								display={"flex"}
								flexWrap={"wrap"}
								mt={{ lg: "5%", base: "5%" }}
								ml={{ base: "10%", lg: "0" }}
								justifyContent={{ base: "center", lg: "start" }}
							>
								<Image
									borderRadius={"md"}
									bg={"#3A7AFE"}
									px={{ lg: 4, base: 4 }}
									py={3}
									mr="3%"
									mt="1"
									src="Linux.svg"
									alt=" "
								/>
								<Image
									borderRadius={"md"}
									bg={"#3A7AFE"}
									pr={10}
									pl={4}
									py={4}
									mr="4.5%"
									mt="1"
									src="mac.svg"
									alt=" "
								/>
							</Box>
							<Box
								display={"flex"}
								flexWrap={"wrap"}
								ml={{ base: "10%", lg: "0" }}
								mt={{ lg: "1%", md: "2%" }}
								justifyContent={{ base: "center", lg: "start" }}
							>
								<Image
									mr={{ lg: "6%", base: "2%" }}
									mt="2"
									src="Playstore button.svg"
									alt=" "
								/>
								<Image mr="5%" mt="2" src="Appstore button.svg" alt=" " />
							</Box>
						</Box>
					</Box>
				</Box>
				<Box
					border="1px"
					borderColor="gray.200"
					width={{ sm: "w-full", lg: "150px" }}
					mt={{ base: "2%", lg: "0px" }}
					textAlign={"center"}
				>
					<Text fontSize={"32px"} color={color}>
						Ads
					</Text>
					<Text>
						Lorem ipsum dolor sit amet, consectetur adipisicing elit. Distinctio
						earum error eius amet sequi veniam deserunt, natus? Illo, fugit.
					</Text>
				</Box>
			</Box>
		</>
	);
};

export default Advertise;
