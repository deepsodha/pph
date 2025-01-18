import { Box, Text } from "@chakra-ui/react";
import type React from "react";
import DrugSearchTable from "../../components/drug-search-table";
import DrugSearchAtoZ from "../../components/drug-search-keyboard";

const page: React.FC = () => {
	return (
		<>
			<Box margin={{ lg: "0%", md: "2%", base: "4%" }}>
				<Box
					display={"flex"}
					justifyContent={"center"}
					flexDirection="column"
					alignItems={"center"}
					mt="3%"
				>
					<Text fontSize={"48px"} textAlign={{ base: "center", lg: "left" }}>
						Drugs & Medications{" "}
						<Text color="#095FBA" as="span">
							A
						</Text>{" "}
						to{" "}
						<Text color="#095FBA" as="span">
							Z
						</Text>
					</Text>
					<Text
						mt="2%"
						fontSize={"24px"}
						textAlign={{ base: "center", lg: "left" }}
					>
						“ The dependable{" "}
						<Text color="#095FBA" as="span">
							{" "}
							Hub
						</Text>{" "}
						for prescription{" "}
						<Text color="#095FBA" as="span">
							Drug
						</Text>{" "}
						insights and information “
					</Text>
				</Box>
				<DrugSearchAtoZ textkey="Search Drugs by A - Z Letter" />
				<DrugSearchTable backgroundcolor="#EBF4F9" />
			</Box>
		</>
	);
};

export default page;
