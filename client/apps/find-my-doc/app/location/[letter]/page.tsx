"use client";
import React, { useCallback, useEffect, useState } from "react";
import {
	Box,
	Card,
	CardBody,
	CardHeader,
	SimpleGrid,
	Text,
	Button,
} from "@chakra-ui/react";
import axios from "axios";
interface Doctor {
	Id: number;
	DoctorName: string;
	Speciality: string;
	Experience: string;
	HospitalAddress: string;
	City: string;
	Pincode: number;
	id: string;
}
interface PostResult {
	value: Doctor[];
}

async function getPosts(params: { letter: string }): Promise<PostResult> {
	try {
		const res = await axios.post("http://localhost:8080/doctors", {
			query: params.letter,
		});
		const hits = res.data.hits;
		console.log(hits);
		const normalizedBrand = params.letter
			? params.letter.replace(/[+\s/]+/g, "-")
			: "";
		console.log(normalizedBrand);
		const filteredData = hits.filter(
			(doc: Doctor) => doc.City?.replace(/[+\s/]+/g, "-") === normalizedBrand,
		);
		return { value: filteredData };
	} catch (error) {
		console.error("Error fetching posts:", error);
		throw error;
	}
}

export default function Page({
	params,
}: {
	params: { letter: string };
}): JSX.Element {
	const [currentPage, setCurrentPage] = useState(1);
	const [visiblePosts, setVisiblePosts] = useState<Doctor[]>([]);
	const itemsPerPage = 11;
	const value = params.letter;

	const fetchData = useCallback(async () => {
		const recentPosts = await getPosts({ letter: value });
		const startIndex = (currentPage - 1) * itemsPerPage;
		const endIndex = currentPage * itemsPerPage;
		setVisiblePosts(recentPosts.value.slice(startIndex, endIndex));
	}, [value, currentPage]);

	const handleNextPage = () => {
		setCurrentPage((prevPage) => prevPage + 1);
	};

	const handlePrevPage = () => {
		setCurrentPage((prevPage) => Math.max(prevPage - 1, 1));
	};

	// Call fetchData directly when the component mounts or when value changes
	useEffect(() => {
		fetchData();
	}, [fetchData]); // Add currentPage to the dependencies array

	return (
		<Box ml={{ base: "2%", md: "5%" }} mr={{ base: "2%", md: "5%" }} mt={"2%"}>
			<SimpleGrid
				minChildWidth={{ base: "200px", lg: "400px" }}
				spacingX="28px"
				spacingY="5px"
			>
				{visiblePosts.map((doctor: Doctor) => (
					<div key={doctor.Id}>
						<Box mt={"5%"} mb={"5%"}>
							<Box mr={"3%"}>
								<Card variant="outline" border="1px solid black" boxShadow="md">
									<CardHeader bg="#FED337" color="white">
										<Text fontSize="lg" fontWeight="bold">
											{doctor.DoctorName}
										</Text>
									</CardHeader>
									<CardBody>
										<Text>
											<Text
												style={{ fontWeight: "bold", display: "inline-block" }}
											>
												Speciality:
											</Text>{" "}
											{doctor.Speciality}
										</Text>
										<Text>
											<Text
												style={{ fontWeight: "bold", display: "inline-block" }}
											>
												Experience:
											</Text>{" "}
											{doctor.Experience}
										</Text>
										<Text>
											<Text
												style={{ fontWeight: "bold", display: "inline-block" }}
											>
												City:
											</Text>{" "}
											{doctor.City}
										</Text>
										<Text>
											<Text
												style={{ fontWeight: "bold", display: "inline-block" }}
											>
												Pincode:
											</Text>{" "}
											{doctor.Pincode}
										</Text>
										<Text>
											<Text
												style={{ fontWeight: "bold", display: "inline-block" }}
											>
												HospitalAddress:
											</Text>{" "}
											{doctor.HospitalAddress}
										</Text>
									</CardBody>
								</Card>
							</Box>
						</Box>
					</div>
				))}
			</SimpleGrid>
			<Box mt={4} display="flex" justifyContent="center" alignItems="center">
				<Button
					disabled={currentPage === 1}
					onClick={handlePrevPage}
					mr={2}
					colorScheme="blue"
				>
					Previous
				</Button>
				<Button
					disabled={visiblePosts.length < itemsPerPage}
					onClick={handleNextPage}
					ml={2}
					colorScheme="blue"
				>
					Next
				</Button>
			</Box>
		</Box>
	);
}
