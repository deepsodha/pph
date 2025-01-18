"use client";
import {
	Box,
	Text,
	Button,
	Image,
	Heading,
	SimpleGrid,
	Link,
	useColorModeValue,
} from "@chakra-ui/react";
import Advertise from "./component/advertise";
import { ChevronRightIcon } from "@chakra-ui/icons";
import CustomBox from "./component/custom-box";

const CustomBoxesData: CustomBox[] = [
	{
		id: 1,
		icon: "Clinic Icon.svg",
		title: "Clinic",
		description:
			"Pi - Health Pi - Health Pi - Health Pi - Health Pi - Health Pi - Health Pi - Health Pi - Health Pi - Health Pi - Health",
	},
	{
		id: 2,
		icon: "Hospital Icon.svg",
		title: "Hospital",
		description:
			"Pi - Health Pi - Health Pi - Health Pi - Health Pi - Health Pi - Health Pi - Health Pi - Health Pi - Health Pi - Health",
	},
	{
		id: 3,
		icon: "findmydoclogo.svg",
		title: "Find My Doc",
		description:
			"Pi - Health Pi - Health Pi - Health Pi - Health Pi - Health Pi - Health Pi - Health Pi - Health Pi - Health Pi - Health",
	},
	{
		id: 4,
		icon: "pipharmacy.svg",
		title: "Pi-Pharmacy",
		description:
			"Pi - Health Pi - Health Pi - Health Pi - Health Pi - Health Pi - Health Pi - Health Pi - Health Pi - Health Pi - Health",
	},
	{
		id: 5,
		icon: "Tx-pi.svg",
		title: "Rx.Pi-Health",
		description:
			"Pi - Health Pi - Health Pi - Health Pi - Health Pi - Health Pi - Health Pi - Health Pi - Health Pi - Health Pi - Health",
	},
	{
		id: 6,
		icon: "Radiologist Icon.svg",
		title: "Radiologist",
		description:
			"Pi - Health Pi - Health Pi - Health Pi - Health Pi - Health Pi - Health Pi - Health Pi - Health Pi - Health Pi - Health",
	},
];

const page: React.FC = () => {
	// const {colormode}  = useColorMode();
	const color = useColorModeValue("black", "white");

	return (
		<>
			<Box display={"flex"} flexDirection={"column"}>
				<Box
					display={"flex"}
					flexDirection={"row"}
					justifyContent={"space-around"}
					bgColor={"#DDEDFA"}
					flexWrap={"wrap"}
				>
					<Box mt={"5%"}>
						<Heading
							as="h1"
							size="6xl"
							fontWeight={600}
							fontSize={{ base: "38px", md: "64px" }}
							textAlign={{ base: "center", md: "left" }}
							justifyContent={"center"}
							flexWrap={"wrap"}
							color={"black"}
							// flexDirection={"column"}
						>
							Your Journey <br /> to{" "}
							<Text as="span" color="#008037" marginRight="5px">
								Wellness
							</Text>{" "}
							Begins Here !<br />
						</Heading>
						<br />
						<Text
							fontWeight={400}
							fontSize="24px"
							color={"black"}
							textAlign={{ base: "center", md: "left" }}
						>
							Digitizing India's Healthcare.
						</Text>
						<br />
						<Link href="/free-trial">
							<Button
								bgColor={"#095FBA"}
								//variant="outline"
								width={{ base: "40%", md: "20%", lg: "24%" }}
								height={{ base: "15%", md: "20%", lg: "14%" }}
								border={"1px solid #095FBA"}
								borderRadius={"4px"}
								color={"white"}
								ml={{ base: "30%", md: "0" }}
								fontSize={"20px"}
							>
								Free Trial
								<ChevronRightIcon ml="6%" />
							</Button>
						</Link>
					</Box>

					<Box mt={{ base: "10%", md: "0" }}>
						<Image src="Doctor image 1.svg" />
					</Box>
				</Box>

				<Box mt={"5%"}>
					<Text
						fontSize="36px"
						fontWeight={600}
						color="#095FBA"
						textAlign={"center"}
					>
						Our Products
					</Text>
				</Box>

				<SimpleGrid
					columns={[1, 2, 3]}
					spacing={4}
					paddingLeft={"10%"}
					paddingRight={"10%"}
					mt={"3%"}
					mb={"4%"}
					color={color}
				>
					{CustomBoxesData.map((box) => (
						<CustomBox key={box.id} {...box} />
					))}
				</SimpleGrid>
			</Box>
			<Box marginY={"5%"}>
				<Advertise />
			</Box>

			<Box
				display="flex"
				justifyContent="center"
				marginY={{ base: "20px", lg: "4%" }}
				marginX={{ base: "20px", lg: "52px" }}
			>
				<Box position="relative" width={{ lg: "80%", md: "100%" }}>
					<Image
						src={"CTA Image.svg"}
						alt=""
						width="100%"
						borderRadius={"6px"}
					/>

					{/* Add text */}
					<Text
						align={"center"}
						position="absolute"
						top={{ lg: "36%", md: "30%", base: "25%" }}
						left="50%"
						transform="translate(-50%, -50%)"
						color="white"
						fontSize={{ lg: "34px", md: "20px", base: "15px" }}
						fontWeight="600"
						lineHeight={"normal"}
					>
						Online consultations with Certified doctors
					</Text>
					<Link href="/contact-us">
						<Button
							position="absolute"
							bottom={{ lg: "15%", md: "5%", base: "10%" }}
							left="50%"
							transform="translateX(-50%)"
							color="#095FBA"
							fontSize={{ lg: "16px", md: "12px", base: "8px" }}
							backgroundColor={"white"}
							size={{ md: "lg", base: "xs", sm: "xs" }}
							_hover={{ bg: "#095FBA", color: "white" }}
						>
							Contact Us
						</Button>
					</Link>
				</Box>
			</Box>
		</>
	);
};

export default page;
