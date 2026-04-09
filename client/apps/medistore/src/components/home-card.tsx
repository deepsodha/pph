import {
	Box,
	Card,
	CardBody,
	Divider,
	Text,
	Image,
	Flex,
	Center,
} from "@chakra-ui/react";

import React from "react";
import OrderIcon from "../assets/order Icon.svg";
import BlackRupee from "../assets/Black Rupee.svg";
const sectionsData = [
	{ id: 0, heading: "sold", textContent: "500" },
	{ id: 1, heading: "return", textContent: "500" },
];

const HomeCard = () => {
	return (
		<div>
			<Box mt={{ base: "10%", md: "6%" }} mb={{ base: "10%", md: "6%" }}>
				<Card
					size={"lg"}
					variant={"outline"}
					//border={"1px solid #1A998E"}
					width="95%"
					ml={"3%"}
				>
					<CardBody pt={"4%"} pl={"4%"} pb={"4%"} bgColor={"#DDF0EE"}>
						<Box display={"flex"} flexDirection={"column"}>
							<Box display={"flex"} flexDirection={"row"}>
								<Box mr={"5%"}>
									<Image src={OrderIcon} height={"100%"} alt="order icon" />
								</Box>
								<Box>
									<Text fontWeight={500} fontSize={20}>
										Today’s Orders
									</Text>
								</Box>
							</Box>
							<Box
								display={"flex"}
								justifyContent={"space-between"}
								mt={"10%"}
								alignItems={"center"}
							>
								<Box>
									<Text fontWeight={400} fontSize={16} color={"#2D3748"}>
										Wednesday 12th Sep, 2023
									</Text>
								</Box>

								<Box>
									<Flex alignItems="center" mt={"2%"}>
										<Box display="flex" alignItems="center" mt={"5%"}>
											<Image src={BlackRupee} alt="icon" height={5} />
											<Text ml={"1%"} fontSize={20}>
												10,000
											</Text>
										</Box>
									</Flex>
								</Box>
							</Box>
							<Box mt={"3%"}>
								<Divider
									orientation="horizontal"
									borderColor={"#D9D9D9"}
									border="1px"
								/>
							</Box>
							<Box display={"flex"} justifyContent={"space-around"}>
								{sectionsData.map((section) => (
									<React.Fragment key={section.id}>
										<Box
											display={"flex"}
											flexDirection={"column"}
											alignItems={"center"}
											mt={"5%"}
										>
											<Box>
												<Text fontWeight={500} fontSize={14}>
													{section.heading}
												</Text>
											</Box>
											<Box mt={"2%"}>
												<Text fontWeight={500} fontSize={16}>
													{section.textContent}
												</Text>
											</Box>
										</Box>
										{section.id !== sectionsData.length - 1 && (
											<Center height="30px" mt="6%">
												<Divider
													orientation="vertical"
													borderColor={"#D9D9D9"}
													border="1px"
												/>
											</Center>
										)}
									</React.Fragment>
								))}
							</Box>
						</Box>
					</CardBody>
				</Card>
			</Box>
		</div>
	);
};

export default HomeCard;
