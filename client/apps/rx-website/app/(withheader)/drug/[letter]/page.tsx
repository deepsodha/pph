import { Box, Text } from "@chakra-ui/react";
import DrugSearchAtoZ from "../../../components/drug-search-keyboard";
import Link from "next/link";
import axios from "axios";
interface Drug {
	Brand: string;
}

interface PostResult {
	drugs: Drug[];
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
// export async function generateStaticParams() {
// 	const alphabets = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".split("");
// 	const paths = alphabets.map((letter) => ({ letter }));
// 	return paths;
// }

async function getPost(params: { letter: string }): Promise<PostResult> {
	try {
		const response = await axios.post("http://localhost:8080/drugs", {
			query: params.letter,
		});
		const hits = response.data.hits;

		return { drugs: hits };
	} catch (error) {
		console.error("Error fetching drugs data:", error);
		return { drugs: [] };
	}
}

const Page: React.FC<{ params: { letter: string } }> = async ({
	params,
}: {
	params: { letter: string };
}) => {
	const post = await getPost(params);
	let index = 0;
	return (
		<Box>
			<Box
				display="flex"
				flexDir={"column"}
				alignItems={"center"}
				justifyContent={"center"}
			>
				<Box
					pt="1%"
					pb="2%"
					width="100%"
					alignItems="center"
					display="flex"
					justifyContent="center"
					flexDir="column"
					backgroundColor={"rgba(9, 95, 186, 0.18)"}
				>
					<Text fontSize={"46px"} textAlign={"center"}>
						{" "}
						Drugs Starting from '
						<Text as="span" color="#095FBA">
							{params.letter}
						</Text>
						'
					</Text>
					<Text pt="1%">
						List of drugs and medications that start with the letter{" "}
						{params.letter}
					</Text>
				</Box>
				<Box
					position="absolute"
					top={{ lg: "300", md: "200", sm: "100", base: "226" }}
					left={{ lg: "400", md: "200", sm: "100" }}
					paddingLeft="0.5%"
					paddingRight={"0.5%"}
					zIndex="1"
					fontSize="20px"
					backgroundColor={"rgba(209, 224, 242, 1)"}
					color="black"
				>
					{params.letter}
				</Box>

				<Box
					backgroundImage={"/drugs bg.svg"}
					// backgroundRepeat="no-repeat"
					backgroundPosition="center"
				>
					<Box
						borderTop="8px"
						borderTopColor={"rgba(209, 224, 242, 1)"}
						display="flex"
						backgroundColor="white"
						mt="5%"
						mb="5%"
						overflowY={"scroll"}
						overflowX={"auto"}
						justifyContent="center"
						marginLeft="20%"
						marginRight="20%"
						flexWrap={"wrap"}
						width="60%"
						height="600px"
						paddingRight="3%"
						paddingBottom="0%"
						paddingTop="0%"
						css={{
							"&::-webkit-scrollbar": {
								width: "5px",
							},
							"&::-webkit-scrollbar-track": {
								background: "rgba(209, 224, 242, 1)",
							},
							"&::-webkit-scrollbar-thumb": {
								background: "#095FBA",
								height: "23px",
							},
						}}
					>
						{post.drugs.map((drug: Drug) => (
							<Box
								key={index++}
								style={{ width: "200px", padding: "2%" }}
								color={"black"}
							>
								{" "}
								<Link
									href={`/drug-overview/${drug.Brand.replace(/[+\s/]+/g, "-")}`}
								>
									<Text color={"black"} fontSize={"16px"}>
										{drug.Brand}
									</Text>
								</Link>
							</Box>
						))}
					</Box>
				</Box>
			</Box>
			<DrugSearchAtoZ
				heading="Browse medications by letter"
				textkey="Search Drugs by A - Z Letter"
			/>
		</Box>
	);
};

export default Page;
