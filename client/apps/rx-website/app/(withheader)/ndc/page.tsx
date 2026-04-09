import {
	Box,
	Button,
	HStack,
	PinInput,
	PinInputField,
	Text,
} from "@chakra-ui/react";
import type React from "react";

import DrugSearchTable from "../../components/drug-search-table";

const page: React.FC = () => {
	const data = [
		{ id: "1", defaultValue: "90439", label: "Laberela Code" },
		{ id: "2", defaultValue: "832", label: "Product Code" },
		{ id: "3", defaultValue: "11", label: "Package Code" },
	];

	return (
		<>
			<Box
				// mt="0%"
				m={{ lg: "0%", base: "2%" }}
				display="flex"
				flexDirection="column"
				justifyContent={"center"}
				alignItems={"center"}
				backgroundImage='url("findpharmacy bg.svg")'
			>
				<Box mt="7%">
					<Text fontSize={"46px"} color={"black"} textAlign={"center"}>
						National Drug Code{" "}
						<Text color="#095FBA" as="span">
							(NDC)
						</Text>
					</Text>
				</Box>
				<Box mt="2%">
					<Text fontSize={"24px"} color={"black"} textAlign={"center"}>
						Find your drugs & medicines through NDC
					</Text>
				</Box>
				<Box
					gap={8}
					flexWrap="wrap"
					display="flex"
					justifyContent="center"
					alignItems="center"
					mt="3%"
				>
					{data.map((code) => (
						<Box key={code.id}>
							<HStack spacing={3}>
								<PinInput defaultValue={code.defaultValue} isDisabled>
									{Array.from({ length: code.defaultValue.length }, (_) => (
										<PinInputField
											key={code.id}
											fontSize="24px"
											backgroundColor="white"
											border="none"
											color="#095FBA"
										/>
									))}
								</PinInput>
							</HStack>
							<Text
								mt="5%"
								textAlign="center"
								fontSize="16px"
								color="rgba(9, 95, 186, 0.6)"
							>
								{code.label}
							</Text>
						</Box>
					))}
					<Box>
						<Button
							size="lg"
							fontWeight={400}
							mb="20%"
							backgroundColor="#095FBA"
							fontSize="22px"
							color="white"
							_hover={{
								backgroundColor: "white",
								border: "1px",
								borderColor: "#095FBA",
								color: "#095FBA",
							}}
						>
							Search
						</Button>
					</Box>
				</Box>

				<Box>
					<Box
						backgroundColor={"white"}
						mt="7%"
						mb="10%"
						border="1px"
						borderColor={"rgba(9, 95, 186, 0.18)"}
						width={{ lg: "700px", base: "100%" }}
						height={{ lg: "125px", base: "100%" }}
						display={"flex"}
						flexWrap={"wrap"}
						justifyContent={"center"}
						alignItems={"center"}
					>
						<Box>
							<Box
								fontSize={"20px"}
								color={"black"}
								alignItems={"center"}
								flexWrap={"wrap"}
								display={"flex"}
								flexDirection={"row"}
								justifyContent={"center"}
							>
								{/* <Tr> */}
								<Box ml="20px">
									<Text>Drug Name</Text>
									<Text
										margin="0"
										padding="0"
										fontSize={"16px"}
										color="rgba(0, 0, 0, 0.49)"
									>
										TYLENOL
									</Text>
								</Box>
								<Box ml="20px">
									<Text>Short Description</Text>
									<Text
										margin="0"
										padding="0"
										fontSize={"16px"}
										color="rgba(0, 0, 0, 0.49)"
									>
										NA
									</Text>
								</Box>
								<Box ml="20px">
									<Text>Laberel Name</Text>
									<Text
										margin="0"
										padding="0"
										fontSize={"16px"}
										color="rgba(0, 0, 0, 0.49)"
									>
										Johnson Consumer Inc.
									</Text>
								</Box>
							</Box>
						</Box>
					</Box>
				</Box>
			</Box>
			<DrugSearchTable backgroundcolor="#EBF4F9" />
		</>
	);
};

export default page;
