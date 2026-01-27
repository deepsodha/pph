"use client";
import {
	Box,
	Text,
	Image,
	Heading,
	Input,
	InputGroup,
	InputRightElement,
	Button,
	Link,
	Card,
	CardBody,
	SimpleGrid,
	useColorModeValue,
} from "@chakra-ui/react";

import { SearchIcon } from "@chakra-ui/icons";
import axios from "axios";
import { useState } from "react";
import type { DrugDetail } from "../components/drug-search-keyboard";
import { useDebouncedEffect } from "@react-hookz/web";
import { baseUrl } from "@repo/services/src";

const boxData = [
	{
		id: 1,
		href: "/drug-search",
		src: "findatozdrug.svg",
		alt: "Find A-Z Drug",
		text: "Find A-Z Drug",
	},
	{
		id: 2,
		href: "/ndc",
		src: "ndc.svg",
		alt: "NDC",
		text: "NDC",
		mb: "10%",
		mt: "15%",
	},
	{
		id: 3,
		href: "/find-pharmacy",
		src: "pharmacy.svg",
		alt: "Find Pharmacy",
		text: "Find Pharmacy",
	},
];

const backgroundImageStyle = {
	backgroundImage: 'url("Hero section left side image.svg")',
	backgroundRepeat: "no-repeat",
	backgroundPosition: "top-left",
};
const page: React.FC = () => {
	const image1 = useColorModeValue("A - Z Drug.svg", "atozwhite.svg");
	const image2 = useColorModeValue("NDC.svg", "ndcwhite.svg");
	const image3 = useColorModeValue("Find pharmacy.svg", "findphamacywhite.svg");
	const image4 = useColorModeValue(
		"Drug Insights icon.svg",
		"DRUG INSIGHTS.svg",
	);

	const featuresData = [
		{
			id: "a-z-drug",
			title: "A - Z Drug",
			content:
				"Easily find info on meds from A to Z. Learn uses, side effects, and interactions. Empower yourself for better health decisions. Your go-to resource for medication knowledge.",
			image: image1,
		},
		{
			id: "ndc",
			title: "NDC",
			content:
				"Quickly access information on medications. Identify drugs, dosage, and manufacturers. Simplify healthcare with easy NDC search. Your trusted tool for medication details.",
			image: image2,
		},
		{
			id: "find-pharmacy",
			title: "Find Pharmacy",
			content:
				"Locate pharmacies quickly. Access addresses, hours, and contact information. Simplify medication pickup with our convenient feature. Your go-to for finding the nearest pharmacy.",
			image: image3,
		},
		{
			id: "side-effects",
			title: "Drug Insights",
			content:
				"Gain valuable information on medications. Understand usage, benefits, and risks. Stay informed for informed decision-making. Your key to unlocking comprehensive drug knowledge.",
			image: image4,
		},
	];
	const [searchTerm, setSearchTerm] = useState("");
	const [searchResults, setSearchResults] = useState<Array<DrugDetail>>([]);
	const [showDropdown, setShowDropdown] = useState(false);

	const handleSearch = async () => {
		try {
			const response = await axios.post(`${baseUrl()}/drugs`, {
				query: searchTerm,
			});

			const hits = response.data.hits;

			if (!hits || hits.length === 0) {
				setSearchResults([]);
				setShowDropdown(false);
			} else {
				setSearchResults(hits);
				setShowDropdown(true);
			}

			console.log("Search result:", hits[0]?.Brand);
			console.log("hits", hits);
		} catch (error) {
			console.error("Error searching data:", error);
		}
	};

	useDebouncedEffect(
		() => {
			handleSearch();
		},
		[searchTerm],
		300,
	);
	return (
		<div>
			<Box
				display={"flex"}
				flexDirection={"column"}
				style={backgroundImageStyle}
			>
				<Box
					display={"flex"}
					flexDirection={{ base: "column", md: "row" }}
					justifyContent="space-evenly"
					ml={"5%"}
					mr={"5%"}
					mt="5%"
				>
					<Box display={"flex"} flexDirection={"column"}>
						<Heading
							as="h1"
							size="6xl"
							fontWeight={500}
							fontSize={{ base: "38px", md: "64px" }}
							textAlign={{ base: "center", md: "left" }}
						>
							Identify &nbsp;
							<Box as="span" color="#095FBA" marginRight="5px">
								Drug &nbsp;
							</Box>
							& <br />
							<Box as="span" color="#095FBA" marginRight="5px">
								Medication &nbsp;
							</Box>
							needs !
						</Heading>
						<InputGroup mt="10%">
							<Input
								variant="outline"
								placeholder="Enter drug name, medication"
								_placeholder={{
									fontSize: { base: "12px", md: "12px", lg: "18px" },
								}}
								size={{ base: "md", lg: "lg" }}
								value={searchTerm}
								onChange={(e) => setSearchTerm(e.target.value)}
							/>
							<InputRightElement
								width={{ base: "30%", sm: "30%", md: "32%", lg: "20%" }}
							>
								<Button
									bgColor={"#095FBA"}
									color="white"
									size={{ base: "md", lg: "lg" }}
									leftIcon={<SearchIcon />}
									mt={{ base: "%", md: "7%", lg: "5%" }}
									_hover={{
										bgColor: "white",
										color: "#095FBA",
										border: "1px solid #095FBA",
									}}
								>
									Search
								</Button>
							</InputRightElement>
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
										key={hit.Brand}
										href={`/drug-overview/${hit.Brand.replace(
											/[+\s/]+/g,
											"-",
										)}`}
									>
										<Box
											padding="2"
											borderBottom="1px solid #095FBA"
											cursor="pointer"
											_hover={{ bgColor: "#F3F3F3" }}
											onClick={() => {
												console.log("Selected suggestion:", hit);
												setSearchTerm(hit.Brand);
												setShowDropdown(false);
											}}
										>
											{hit.Brand}
										</Box>
									</Link>
								))}
							</Box>
						)}
						<Box
							display={"flex"}
							flexDirection="row"
							mt="10%"
							flexWrap={"wrap"}
							justifyContent={{
								base: "space-evenly",
								md: "space-evenly",
								lg: "space-around",
							}}
						>
							{boxData.map(({ id, href, src, alt, text, mb, mt }) => (
								<Box
									key={id}
									display="flex"
									flexDirection="column"
									justifyContent={"center"}
									alignItems="center"
									mb="3%"
								>
									<Link href={href}>
										<Image src={src} alt={alt} />
									</Link>
									<Text width="70%" textAlign="center" mb={mb || "0%"} mt={mt}>
										{text}
									</Text>
								</Box>
							))}
						</Box>
					</Box>

					<Box mb={{ base: "10%", md: "0%" }}>
						<Image src={"doctor img.svg"} />
					</Box>
				</Box>
			</Box>
			<Box
				border="1px"
				borderColor="gray.200"
				width={"w-full"}
				mt={{ base: "10%", lg: "7%" }}
				pb={{ base: "10%", lg: "2%" }}
				textAlign={"center"}
			>
				<Text fontSize={"32px"} color={"white"}>
					Ads
				</Text>
				<Text>
					Lorem ipsum dolor sit amet, consectetur adipisicing elit. Distinctio
					earum error eius amet sequi veniam deserunt, natus? Illo, fugit.
				</Text>
			</Box>
			<Box
				display="flex"
				alignItems="center"
				justifyContent="center"
				backgroundImage={"our features bg.svg"}
				py="10%"
			>
				<Box display="flex" flexDirection="column" alignItems="center">
					<Text color="#095FBA" fontSize="46px" fontWeight={400}>
						Our Features
					</Text>
					<Box
						display="flex"
						justifyContent={"center"}
						flexDirection={{ lg: "column", base: "column" }}
						mt="2%"
					>
						<SimpleGrid columns={{ base: 1, sm: 2, lg: 2 }} spacing={5}>
							{featuresData.map((feature) => (
								<Card
									key={feature.id}
									id={feature.id}
									mt="3%"
									// mb="5%"
									// mr="10%"
									border="1px"
									width={{ lg: "400px", base: "100%" }}
								>
									<CardBody>
										<Box display="flex">
											<Box width="200px" mr="4%">
												<Image src={feature.image} alt="" />
											</Box>
											<Box>
												<Text fontSize="32px" fontWeight={500}>
													{feature.title}
												</Text>
												<Text fontSize="16px" fontWeight={400}>
													{feature.content}
												</Text>
											</Box>
										</Box>
									</CardBody>
								</Card>
							))}
						</SimpleGrid>
					</Box>
				</Box>
			</Box>
		</div>
	);
};
export default page;
