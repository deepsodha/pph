"use client";
import {
	Box,
	Button,
	Center,
	Flex,
	Input,
	Image,
	Text,
	InputGroup,
} from "@chakra-ui/react";
import type React from "react";
import { useState } from "react";
import { chunk } from "lodash";
import Link from "next/link";
import { useDebouncedEffect } from "@react-hookz/web";
import axios from "axios";

interface DrugSearchProps {
	textkey: string;
	heading?: string;
}

export interface MeiliSearchResponse {
	hits: DrugDetail[];
	offset: number;
	limit: number;
	estimatedTotalHits: number;
	processingTimeMs: number;
	query: string;
}

export interface DrugDetail {
	Brand: string;
	Generic: string;
	Details: string;
	Category: string;
	SideEffects: string;
	DrugDoseInfo: string;
	Precautions: string;
	ManufacturerName: string;
	Medicines: string;
	ContraIndications: string;
	Diseases: string;
	Interactions: string;
	Contains: string;
}

const DrugSearchAtoZ: React.FC<DrugSearchProps> = ({ textkey, heading }) => {
	const alphabets = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".split("");
	let uniqueIdCounter = 0;
	let indexId = 1;
	const [searchTerm, setSearchTerm] = useState("");
	const [searchResults, setSearchResults] = useState<Array<DrugDetail>>([]);
	const [showDropdown, setShowDropdown] = useState(false);

	useDebouncedEffect(
		() => {
			const searchIndex = async () => {
				try {
					const response = await axios.post("http://localhost:8080/drugs", {
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

			searchIndex();
		},
		[searchTerm],
		300,
	);

	return (
		<>
			<Box
				backgroundColor=" rgba(9, 95, 186, 0.7)"
				display={"flex"}
				justifyContent={"center"}
				mt="3%"
				flexDirection="column"
				alignItems={"center"}
			>
				<Text mt="2%" color="#FFFFFF" fontSize={"46px"}>
					{heading}
				</Text>
				<Box mt="3%" width={{ lg: "25% ", base: "80%" }}>
					<InputGroup rounded="md" bgColor="#095FBA">
						<Input
							type="text"
							height="58px"
							bgColor="white"
							focusBorderColor="#095FBA"
							color="#717B9E"
							roundedRight={"unset"}
							placeholder="Enter drug name, medication"
							_placeholder={{ color: "gray.500" }}
							value={searchTerm}
							onChange={(e) => setSearchTerm(e.target.value)}
						/>
						<Box bg="#095FBA" rounded="md" width={16} height={8}>
							<Center>
								<Image src="/Search icon.svg" mt={4} />
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
									href={`/drug-overview/${hit.Brand.replace(/[+\s/]+/g, "-")}`}
								>
									<Box
										key={indexId++}
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
				</Box>
				<Box>
					<Text color="rgba(255, 255, 255, 1)" fontWeight={400} mt="6%">
						{textkey}
					</Text>
				</Box>
				<Box
					display="flex"
					flexDirection={"column"}
					marginBottom="3%"
					justifyContent={"center"}
					alignItems={"center"}
					marginTop="1%"
				>
					{chunk(alphabets, 10).map((row) => (
						<Flex
							alignItems={"center"}
							justifyContent={{ base: "center", lg: "" }}
							key={`flex-${uniqueIdCounter++}`}
							flexWrap={{ base: "wrap", lg: "nowrap" }}
						>
							{row.map((letter: string) => (
								<Link key={letter} href={`/drug/${letter}`}>
									<Button
										key={letter}
										fontSize="24px"
										fontWeight="400"
										marginX="4"
										marginY="4"
									>
										{letter}
									</Button>
								</Link>
							))}
						</Flex>
					))}
				</Box>
			</Box>
		</>
	);
};

export default DrugSearchAtoZ;
