import { Box, type CSSObject, List, ListItem, Text } from "@chakra-ui/react";
import axios from "axios";
interface Drug {
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

interface PostResult {
	drugs: Drug[];
}
async function getPost(params: { drug: string }): Promise<PostResult> {
	try {
		const allDrugsData = await axios.post("http://localhost:8080/drugs", {
			query: params.drug,
		});

		const normalizedBrand = params.drug.replace(/[+\s/]+/g, "-"); // Replace spaces, plus signs, hyphens, and slashes with hyphens
		const hits = allDrugsData.data.hits;

		const drugs = hits.filter(
			(drug: Drug) => drug.Brand.replace(/[+\s/]+/g, "-") === normalizedBrand,
		);
		return { drugs };
	} catch (error) {
		console.error("Error reading drug data file:", error);
		return { drugs: [] };
	}
}

const scrollbarStyles: CSSObject = {
	"&::-webkit-scrollbar-thumb": {
		color: "#095FBA",
		height: "40px",
	},
	"&::-webkit-scrollbar-track": {
		backgroundColor: "#D9D9D9",
	},
};

const Page: React.FC<{ params: { drug: string } }> = async ({
	params,
}: {
	params: { drug: string };
}) => {
	const post = await getPost(params);
	let index = 0;

	return (
		<>
			{post.drugs.map((drug: Drug) => (
				<Box key={index++}>
					<Box
						display={"flex"}
						flexDirection={"column"}
						justifyContent={"center"}
						alignItems={"center"}
						mt="2%"
					>
						<Text color="#095FBA" fontSize={"36px"}>
							{drug.Brand.toUpperCase()}
						</Text>
						<Text fontSize={"24px"} mt="1%">
							GENERIC NAME(S):
							<Text as="span" color="#095FBA">
								{drug.Generic.toUpperCase()}
							</Text>
						</Text>
						<Box
							mt="2%"
							backgroundColor={"#D3E2F2"}
							width="60%"
							paddingLeft="2%"
							pt="1%"
							pb="1%"
						>
							<Text fontSize={"28px"} fontWeight={700} color={"black"}>
								Basic information
							</Text>
						</Box>

						<Box
							backgroundColor={"#EBF4F9"}
							width="60%"
							mt="1%"
							display="flex"
							flexDirection={"column"}
							paddingLeft="2%"
							pt="1%"
							pb="1%"
							my="1%"
						>
							<Text fontSize="22px" fontWeight={700} mb="1%" color="#095FBA">
								Use Of Drug :
							</Text>
							<Text fontSize={"20px"} mb="2%" color={"black"}>
								{drug.Details}
							</Text>
							<Text fontSize="22px" fontWeight={700} mb="1%" color="#095FBA">
								Category Of Drug :
							</Text>
							<Text fontSize={"20px"} mb="2%" color={"black"}>
								{drug.Category}
							</Text>
							<Text fontSize="22px" fontWeight={700} mb="1%" color="#095FBA">
								Side Effects :
							</Text>
							<Box
								maxH="150px"
								overflowY="auto"
								mr="8%"
								mb="2%"
								__css={scrollbarStyles}
							>
								<List>
									{drug.SideEffects.split(/(?=[A-Z])/).map((effects) => (
										<ListItem key={index++}>
											<Text fontSize={"20px"} color={"black"}>
												{effects}
											</Text>
										</ListItem>
									))}
								</List>
							</Box>
							<Text fontSize="22px" fontWeight={700} mb="1%" color="#095FBA">
								Drug Dose Information :
							</Text>
							<Text fontSize={"20px"} mb="2%" color={"black"}>
								{drug.DrugDoseInfo}
							</Text>
							<Text fontSize="22px" fontWeight={700} mb="1%" color="#095FBA">
								Precautions :
							</Text>
							<Box
								maxH="150px"
								overflowY="auto"
								mr="8%"
								mb="2%"
								__css={scrollbarStyles}
							>
								<List>
									{drug.Precautions.split(",").map((precaution) => (
										<ListItem key={index++}>
											<Text fontSize={"20px"} color={"black "}>
												{precaution}
											</Text>
										</ListItem>
									))}
								</List>
							</Box>

							<Text fontSize="22px" fontWeight={700} mb="2%" color="#095FBA">
								Brand Information :
							</Text>
							<Text fontSize={"20px"} color={"black"}>
								Brand :{" "}
								<Text as="span" color={"black"}>
									{drug.Brand}
								</Text>
							</Text>
							<Text fontSize={"20px"} color={"black"}>
								Contains :{" "}
								<Text as="span" color={"black"}>
									{drug.Contains}
								</Text>
							</Text>
							<Text fontSize={"20px"} color={"black"}>
								Manufacturer Name :{" "}
								<Text as="span"> {drug.ManufacturerName}</Text>
							</Text>
							<Text fontSize={"20px"} mb="1%" color={"black"}>
								Medicines :{" "}
								<Text as="span" color={"black"}>
									{drug.Medicines}
								</Text>
							</Text>
							<Text fontSize="22px" fontWeight={700} mb="1%" color="#095FBA">
								Contra Indications :
							</Text>
							<Box mb="2%">
								<List>
									{drug.ContraIndications.split(",").map((indications) => (
										<ListItem key={index++}>
											<Text fontSize={"20px"} color={"black"}>
												{indications}
											</Text>
										</ListItem>
									))}
								</List>
							</Box>
							<Text fontSize="22px" fontWeight={700} mb="1%" color="#095FBA">
								Diseases :
							</Text>
							<Box mb="2%">
								<List>
									{drug.Diseases.split(",").map((diseases) => (
										<ListItem key={index++}>
											<Text fontSize={"20px"} color={"black"}>
												{diseases}
											</Text>
										</ListItem>
									))}
								</List>
							</Box>
							<Text fontSize="22px" fontWeight={700} mb="1%" color="#095FBA">
								Interactions :
							</Text>
							<Box mb="2%">
								<List>
									{drug.Interactions.split(".").map((interactions) => (
										<ListItem key={index++}>
											<Text fontSize={"20px"} mb="1%" color={"black"}>
												{interactions}
											</Text>
										</ListItem>
									))}
								</List>
							</Box>
						</Box>
					</Box>
				</Box>
			))}
		</>
	);
};
export default Page;
