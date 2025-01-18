"use client";
import {
	Box,
	Input,
	Image,
	Flex,
	Button,
	Center,
	Text,
	ListItem,
	UnorderedList,
	Accordion,
	AccordionButton,
	AccordionIcon,
	AccordionItem,
	AccordionPanel,
	InputGroup,
	InputLeftElement,
	Select,
	Grid,
	SimpleGrid,
} from "@chakra-ui/react";

import { SearchIcon } from "@chakra-ui/icons";
import Header from "./components/header";
import Footer from "./components/footer";
import { useState } from "react";
import Slider from "./components/slider";
import Link from "next/link";
const locations = [
	{ id: 1, name: "Ahmedabad" },
	{ id: 2, name: "Vadodara" },
	{ id: 3, name: "Surat" },
	{ id: 4, name: "Pune" },
	{ id: 5, name: "Ahmedabad" },
	{ id: 6, name: "Ahmedabad" },
];
const categories = [
	{ id: 1, name: "Primary Care", image: "primary care icon.svg" },
	{ id: 2, name: "Dentist", image: "Dentist icon.svg" },
	{ id: 3, name: "Gynecology", image: "Gynecology Icon.svg" },
	{ id: 4, name: "Dermatologist", image: "Dermatologist Icon.svg" },
	{ id: 5, name: "Psychiatrist", image: "Psychiatrist icon.svg" },
	{ id: 6, name: "Eye Doctor", image: "Eye doctor icon.svg" },
];
import MeiliSearch from "meilisearch";
import { useDebouncedEffect } from "@react-hookz/web";
// const cardsData = [
//   {
//     id: 1,
//     iconSrc: "appointment Icon (2).svg",
//     title: "Book an online appointment",
//     buttonLabel: "See Availability",
//   },
//   {
//     id: 2,
//     iconSrc: "reviews icon.svg",
//     title: "Read reviews from users",
//     buttonLabel: "See Providers",
//   },
// ];

const visitsData = [
	{ id: 1, visit: "Medical" },
	{ id: 2, visit: "Dental" },
	{ id: 3, visit: "Mental Health" },
	{ id: 4, visit: "Purpose" },
];

export interface DoctorDetails {
	Id: string;
	DoctorName: string;
	Speciality: string;
	Experience: string;
	HospitalAddress: string;
	City: string;
	Pincode: string;
	//id: string;
}
export default function Home(): JSX.Element {
	const [isVisible, setIsVisible] = useState(true); // State to control visibility

	const handleClose = () => {
		setIsVisible(false); // Hide the image and other elements
	};

	const [searchTerm, setSearchTerm] = useState("");
	const [searchResults, setSearchResults] = useState<Array<DoctorDetails>>([]);
	const [showDropdown, setShowDropdown] = useState(false);
	//let uniqueIdCounter = 0;
	let indexId = 1;
	useDebouncedEffect(
		() => {
			const searchIndex = async () => {
				try {
					const client = new MeiliSearch({
						host: "http://localhost:7700",
						apiKey: "GBowmX3uTYeTR87bohvqnwjSYZzFzSNdbhbk8y23Zz0",
					});

					const index = client.index("doctors");

					const response = await index.search(searchTerm);

					if (searchTerm.trim() === "") {
						setSearchResults([]);
						setShowDropdown(false);
					} else {
						setSearchResults(response.hits as DoctorDetails[]);
						setShowDropdown(true);
					}

					// console.log(
					// 	"Search result:",
					// 	(response.hits[0] as DoctorDetails).Speciality,
					// );
					// console.log("hits", response.hits);
				} catch (error) {
					console.error("Error searching data:", error);
				}
			};

			searchIndex();
		},
		[searchTerm],
		300,
	);
	return (
		<div>
			<Box backgroundColor="#F6FBFD" width="100%">
				<Header />
				<Flex justifyContent={"space-around"} flexWrap={"wrap"}>
					<Box m={{ lg: "6%", md: "5%", base: "5%" }}>
						<Text
							fontSize={{ lg: "48px", md: "32px" }}
							color={"black"}
							fontWeight={600}
							textAlign={{ lg: "left", base: "center" }}
						>
							Book local Doctors <br /> who takes care your health
						</Text>
						<Box
							mt="10%"
							height="100%"
							width={{ lg: "800px", md: "600px", sm: "450px", base: "100%" }}
						>
							<InputGroup rounded="md" bgColor="#51A7D6" border="1px">
								<InputLeftElement
									mr={{ base: "0%", md: "1%" }}
									ml={{ base: "0%", md: "1%" }}
								>
									<SearchIcon
										mt={{ lg: "60%", md: "30%" }}
										boxSize={6}
										color={"black"}
									/>
								</InputLeftElement>
								<Input
									type="text"
									bgColor="white"
									focusBorderColor="##51A7D6"
									color="gray.500"
									roundedRight={"unset"}
									padding="4%"
									placeholder="Search Doctor"
									_placeholder={{
										pl: { base: "10%", md: "10%", lg: "3%" },
										color: "gray.500",
									}}
									value={searchTerm}
									onChange={(e) => setSearchTerm(e.target.value)}
								/>
								<Box bg="#51A7D6" rounded="md" width={"13%"} height={8}>
									<Center>
										<Image
											src="Search.svg"
											width={{ base: "50px", lg: "100px" }}
											height="46px"
											mt={{ lg: "13%" }}
										/>
									</Center>
								</Box>
							</InputGroup>
							{showDropdown && (
								<Box
									maxH="100px"
									overflowY="auto"
									backgroundColor="white"
									border="1px solid #095FBA"
									borderRadius="md"
									boxShadow="md"
								>
									{" "}
									{searchResults.map((hit) => (
										<Link
											key={indexId++}
											href={`/doctor/${hit.Speciality.replace(
												/[+\s/]+/g,
												"-",
											)}`}
										>
											<Box
												key={indexId++}
												padding="2"
												borderBottom="1px solid #095FBA"
												cursor="pointer"
												_hover={{ bgColor: "#F3F3F3" }}
												onClick={() => {
													console.log("Selected suggestion:", hit);
													setSearchTerm(hit.Speciality);
													setShowDropdown(false);
												}}
											>
												{hit.Speciality}
											</Box>
										</Link>
									))}
								</Box>
							)}
						</Box>
					</Box>
					<Box>
						<Image src="hero section image.svg" alt="Dan Abramov" />
					</Box>
				</Flex>
			</Box>

			<Box ml={"8%"} mt="7%" mb="10%" mr="8%">
				<Text
					fontSize={{ lg: "36px", md: "32px", base: "24px" }}
					textAlign={{ lg: "left", base: "center" }}
					fontWeight={400}
				>
					Top Searched Specialities
				</Text>
				<Flex
					flexWrap={{ base: "wrap", md: "nowrap" }}
					justifyContent={{ base: "space-around" }}
				>
					{categories.map((category) => (
						<Box key={category.id} mt="4%">
							<Image src={category.image} />
							<Text fontSize="20px" textAlign="center" marginTop="10%">
								{category.name}
							</Text>
						</Box>
					))}
				</Flex>
			</Box>

			<Box
				backgroundColor="#FFFBEB"
				paddingBottom={{ base: "0%", md: "3%" }}
				display="flex"
				flexDirection={{ base: "column", md: "row" }}
				pt={{ base: "0%", md: "2%" }}
			>
				<Slider />

				{/* <Text
						fontSize={"36px"}
						textAlign="center"
						marginTop="5%"
						pt="4%"
						fontWeight={400}
					>
						Let's connect you with a doctor who understands you
					</Text>
					<Flex
						justifyContent="center"
						marginTop="3%"
						flexWrap={{ base: "wrap" }}
					>
						{cardsData.map((card) => (
							<Box
								key={card.id}
								marginRight={{ lg: "3%", md: "0%", base: "4%" }}
								marginLeft={{ base: "4%", lg: "0%" }}
								width={{ lg: "40%", md: "80%" }}
								marginTop={{ md: "10%", lg: "0%", base: "10%" }}
							>
								<Card
									align="center"
									backgroundColor="#FFFBEB"
									padding="10%"
									border="1px"
									borderColor="black"
									width="100%"
									height="100%"
									ml={{ lg: "6%", base: "auto" }}
								>
									<Image src={card.iconSrc} borderRadius="lg" />
									<Stack mt="3">
										<Box>
											<Text fontSize="32px" textAlign="center">
												{card.title}
											</Text>
										</Box>
									</Stack>
									<Button variant="outline" colorScheme="green" marginTop="8%">
										{card.buttonLabel}
									</Button>
								</Card>
							</Box>
						))}
					</Flex> */}
			</Box>

			<Box mt={{ base: "20%", lg: "10%" }} mb="10%">
				<Flex justifyContent={"space-around"} flexWrap={"wrap"}>
					<Box>
						<Image src="Rectangle 4930.svg" />
					</Box>
					<Box>
						<Text
							fontSize={"24px"}
							color="#888888"
							mt="3%"
							fontWeight={500}
							textAlign={{ lg: "left", base: "center" }}
						>
							FindmyDoc FOR PRIVATE PRACTICES
						</Text>
						<Text
							fontSize={{ lg: "36px", base: "26px" }}
							marginTop="3%"
							fontWeight={400}
							textAlign={{ lg: "left", base: "center" }}
						>
							Are you a provider interested in <br />
							reaching new patients?
						</Text>
						<UnorderedList
							marginTop="4%"
							fontSize={{ lg: "20px", base: "15px" }}
							m="4"
						>
							<ListItem
								ml="4%"
								paddingBottom="2%"
								textAlign={{ lg: "left", base: "center" }}
							>
								Reach patients in your area looking for a new provider
							</ListItem>
							<ListItem
								ml="4%"
								paddingBottom="2%"
								textAlign={{ lg: "left", base: "center" }}
							>
								Fill last-minute openings in your schedule
							</ListItem>
							<ListItem
								ml="4%"
								paddingBottom="2%"
								textAlign={{ lg: "left", base: "center" }}
							>
								Strengthen your online reputation with verified reviews
							</ListItem>
						</UnorderedList>
						<Box
							display={"flex"}
							justifyContent={{ base: "center", lg: "flex-start" }}
						>
							<Link href={"/sign-up"}>
								<Button
									variant="solid"
									backgroundColor="#5FC69B"
									color="white"
									marginTop="4%"
									// marginLeft="8%"
									_hover={{
										bgColor: "white",
										color: "#5FC69B",
										borderColor: "#5FC69B",
										border: "1px",
									}}
								>
									List your practice on FindmyDoc
								</Button>
							</Link>
						</Box>
					</Box>
				</Flex>
			</Box>

			<Box
				backgroundColor="#F6FBFD"
				paddingTop={"3%"}
				paddingBottom={"3%"}
				mt="6%"
				mb="6%"
			>
				<Box mt="2%">
					<Flex
						direction={{ base: "column", md: "row" }}
						justifyContent={"space-around"}
						flexWrap={"wrap"}
					>
						<Box>
							<Text
								fontSize={"24px"}
								color="#888888"
								marginTop="5%"
								fontWeight={500}
								textAlign={{ base: "center", md: "left" }}
							>
								FindmyDoc FOR HEALTH SYSTEM
							</Text>
							<Text
								fontSize={{ lg: "36px", base: "26px" }}
								color="#212B36"
								fontWeight={500}
								textAlign={{ base: "center", lg: "left" }}
							>
								We’re trusted by top <br />
								health systems
							</Text>
							<Box
								display={"flex"}
								justifyContent={{ lg: "flex-start", base: "center" }}
							>
								<Link href={"/sign-up"}>
									<Button
										variant="solid"
										backgroundColor="#5FC69B"
										color="white"
										marginTop="22px"
										_hover={{
											bgColor: "white",
											color: "#5FC69B",
											borderColor: "#5FC69B",
											border: "1px",
										}}
										// ml={{ base: "22%", md: "0%" }}
									>
										Partner with FindmyDoc
									</Button>
								</Link>
							</Box>
						</Box>
						<Box
							display={"flex"}
							justifyContent={{ lg: "flex-start", base: "center" }}
							mt={{ lg: "2%", base: "5%" }}
							// mb={{ lg: "2%", base: "5%" }}
						>
							<Image
								display={{ base: "hidden", lg: "block" }}
								src="happy.svg"
							/>
						</Box>
					</Flex>
				</Box>
			</Box>

			<Flex
				direction={{ base: "column", md: "row" }}
				justifyContent={"space-evenly"}
				alignItems={"center"}
				flex-wrap={"wrap"}
			>
				<Box width={{ lg: "50%", md: "100%" }}>
					<Box
						// ml={{ lg: "15%", md: "5%", base: "5%" }}
						mt="5%"
						mb="6%"
						mx="2%"
						// mr="5%"
					>
						<Text
							fontSize={"36px"}
							fontWeight={400}
							textAlign={{ base: "center", lg: "left" }}
						>
							Find doctors by city
						</Text>
						<Box mt="10%" height="100%">
							<InputGroup rounded="md" bgColor="#5FC69B">
								<Input
									type="text"
									height="45px"
									bgColor="white"
									focusBorderColor="#5FC69B"
									color="black"
									placeholder="Enter city name"
									_placeholder={{ color: "black" }}
									value={searchTerm}
									onChange={(e) => setSearchTerm(e.target.value)}
								/>
								<Box bg="#5FC69B" rounded="md" width={16} height={8}>
									<Center>
										<Image src="Search.svg" mt={2} />
									</Center>
								</Box>
							</InputGroup>
							{showDropdown && (
								<Box
									maxH="100px"
									overflowY="auto"
									backgroundColor="white"
									border="1px solid #095FBA"
									borderRadius="md"
									boxShadow="md"
								>
									{searchResults.map((hit) => (
										<Link
											key={indexId++}
											href={`/location/${hit.City.replace(/[+\s/]+/g, "-")}`}
										>
											<Box
												key={indexId++}
												padding="2"
												borderBottom="1px solid #095FBA"
												cursor="pointer"
												_hover={{ bgColor: "#F3F3F3" }}
												onClick={() => {
													console.log("Selected suggestion:", hit);
													setSearchTerm(hit.City);
													setShowDropdown(false);
												}}
											>
												{hit.City}
											</Box>
										</Link>
									))}
								</Box>
							)}
						</Box>
						<Grid mt="4%" templateColumns="repeat(2, 1fr)" gap={4}>
							{locations.map((location) => (
								<Box key={location.id}>
									<Select size="lg" placeholder={location.name}>
										<option value="option1">Download</option>
										<option value="option2">Create a Copy</option>
										<option value="option3">Mark as Draft</option>
										<option value="option3">Delete</option>
										<option value="option3">Attend a Workshop</option>
									</Select>
								</Box>
							))}
						</Grid>
					</Box>
				</Box>
				<Box
					ml={{ lg: "10%", md: "0%", base: "0%" }}
					mt={{ lg: "5%", md: "18%", base: "0%" }}
				>
					{isVisible && (
						<Box
							position="relative"
							textAlign="center"
							ml={{ base: "5%", md: "0%" }}
							mr={{ base: "3%", md: "0%" }}
						>
							<Box position="absolute" top="3%" left="0%" width="100%">
								<Button
									position="absolute"
									top="5%"
									right={{ base: "3%", md: "3%", lg: "3%" }}
									onClick={handleClose}
									bgColor="transparent"
								>
									<Image src="cross.svg" />
								</Button>
							</Box>
							<Box position="absolute" top="15%" left="10%" color="white">
								<Text fontWeight={600} fontSize="24px">
									Get Medicine Information
								</Text>
							</Box>
							<Box
								position="absolute"
								bottom="5%"
								left="50%"
								transform="translateX(-50%)"
							>
								<Button bgColor="#5FC69B" color="white">
									{" "}
									Contact Us{" "}
								</Button>
							</Box>
							<Image
								src="adertisment.svg"
								maxW="100%"
								height="auto"
								//ml={{ base: "3%", md: "0%" }}
							/>
						</Box>
					)}
				</Box>
			</Flex>

			<Box
				mt={{ lg: "10%", base: "20%" }}
				mb={{ lg: "10%", base: "20%" }}
				flexDirection={"column"}
				display={"flex"}
				flexWrap={"wrap"}
				alignItems={"center"}
				justifyContent={"center"}
			>
				<Text
					fontSize={"36px"}
					fontWeight={400}
					textAlign={{ base: "center", lg: "left" }}
				>
					Common visit reasons
				</Text>
				{/* <Flex
					flexDirection={ "row"}
					flexWrap={"wrap"}
					alignItems={"center"}
					justifyContent={"center"}
					padding="0"
					margin="0"
				> */}

				<SimpleGrid columns={[1, 2, 3, 4]} spacing={6}>
					{visitsData.map((category) => (
						<Box key={category.id} ml={"1%"} mt={"10%"}>
							<Accordion
								allowToggle
								width="282px"
								borderTopColor={"white"}
								borderBottom="none"
							>
								<AccordionItem>
									<h2>
										<AccordionButton>
											<Box as="span" flex="1" textAlign="left">
												{category.visit}
											</Box>
											<AccordionIcon />
										</AccordionButton>
									</h2>
									<AccordionPanel pb={4}>
										Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed
										do eiusmod tempor incididunt ut labore et dolore magna
										aliqua. Ut enim ad minim veniam, quis nostrud exercitation
										ullamco laboris nisi ut aliquip ex ea commodo consequat.
									</AccordionPanel>
								</AccordionItem>
							</Accordion>
						</Box>
					))}
				</SimpleGrid>
				{/* </Flex> */}
			</Box>
			<Footer />
		</div>
	);
}
