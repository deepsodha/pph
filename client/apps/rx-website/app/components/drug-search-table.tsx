import { Table, Tbody, Td, Tr, Text, Box } from "@chakra-ui/react";
import React from "react";
interface DrugSearchTableProps {
	backgroundcolor?: string;
}

const DrugSearchTable = ({ backgroundcolor }: DrugSearchTableProps) => {
	const medications = [
		{
			id: 1,
			name: [
				"Abilify",
				"Buspirone",
				"Flagyl",
				"Abilify",
				"Buspirone",
				"Flagyl",
			],
		},
		{
			id: 2,
			name: [
				"Actos",
				"Bystolic",
				"Gabapentin",
				"Actos",
				"Bystolic",
				"Gabapentin",
			],
		},
		{
			id: 3,
			name: [
				"Acyclovir",
				"Celexa",
				"Hydromet",
				"Acyclovir",
				"Celexa",
				"Hydromet",
			],
		},
		{
			id: 4,
			name: [
				"Adderall",
				"Cialis",
				"Ibuprofen",
				"Adderall",
				"Cialis",
				"Ibuprofen",
			],
		},
		{ id: 5, name: ["Ativan", "Cipro", "Keppra", "Ativan", "Cipro", "Keppra"] },
		{
			id: 6,
			name: [
				"Baclofen",
				"Citalopram",
				"Lexapro",
				"Baclofen",
				"Citalopram",
				"Lexapro",
			],
		},
		{
			id: 7,
			name: [
				"Bactrim",
				"Clindamycin",
				"Lisinopril",
				"Bactrim",
				"Clindamycin",
				"Lisinopril",
			],
		},
		{
			id: 8,
			name: [
				"Benadryl",
				"Cymbalta",
				"Losartan",
				"Benadryl",
				"Cymbalta",
				"Losartan",
			],
		},
		{
			id: 9,
			name: [
				"Benzonatate",
				"Dexilant",
				"Lyrica",
				"Benzonatate",
				"Dexilant",
				"Lyrica",
			],
		},
		{
			id: 10,
			name: [
				"Bupropion",
				"Dicyclomine",
				"Morphine",
				"Bupropion",
				"Dicyclomine",
				"Morphine",
			],
		},
		{
			id: 11,
			name: [
				"Abilify",
				"Buspirone",
				"Flagyl",
				"Abilify",
				"Buspirone",
				"Flagyl",
			],
		},
	];

	return (
		<>
			<Box
				margin={{ lg: "0%", md: "2%", base: "4%" }}
				backgroundColor={backgroundcolor}
				display="flex"
				flexDirection={"column"}
				justifyContent={"center"}
				alignItems={"center"}
			>
				<Text fontSize={"40px"} mt="3%" textAlign={"center"} color={"black"}>
					Popular{" "}
					<Text color="#095FBA" as="span">
						Drug
					</Text>{" "}
					Searches
				</Text>
				<Box
					width={{ lg: "60%", md: "100%", base: "100%" }}
					overflowX="auto"
					color={"black"}
				>
					<Table mt="2%" mb="3%" variant="unstyled">
						<Tbody>
							{medications.map((row) => (
								<Tr key={row.id}>
									{row.name.map((cell) => (
										<Td padding="1%" key={row.id}>
											{cell}
										</Td>
									))}
								</Tr>
							))}
						</Tbody>
					</Table>
				</Box>
			</Box>
		</>
	);
};

export default DrugSearchTable;
