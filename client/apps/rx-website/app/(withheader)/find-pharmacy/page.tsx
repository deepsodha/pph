import {
	Text,
	Box,
	Input,
	Button,
	Image,
	Table,
	Tbody,
	Td,
	Tr,
} from "@chakra-ui/react";
import Link from "next/link";
import type React from "react";

const page: React.FC = () => {
	return (
		<>
			<Box
				display={"flex"}
				flexDirection={"column"}
				justifyContent={"center"}
				alignItems={"center"}
				backgroundImage='url("findpharmacy bg.svg")'
			>
				<Box>
					<Text
						whiteSpace={"nowrap"}
						font-family=" Work Sans"
						fontSize={{ xl: "46px", lg: "28px", base: "20px" }}
						fontWeight={400}
						mr="10%"
						mt="10%"
						color={"black"}
					>
						Find your nearest
						<Text ml="2%" color="#095FBA" as="span">
							Pharmacy
						</Text>
					</Text>
				</Box>
				<Box>
					<Text
						mt="8%"
						fontSize={"32px"}
						fontWeight={400}
						color={"black"}
						textAlign={"center"}
					>
						Register your pharmacy today !
					</Text>
				</Box>
				<Box
					mt="2%"
					mr="3%"
					display={"flex"}
					flexWrap={{ base: "wrap", md: "nowrap", lg: "nowrap" }}
					justifyContent={"center"}
				>
					<Box mr="2%" mb="2%">
						<Button
							_hover={{ backgroundColor: "white" }}
							border="1px"
							borderRadius={"4px"}
							backgroundColor="#EBF4F9"
							marginRight="20%"
							color={"black"}
						>
							<Link href="/login">Login Pharmacy</Link>
						</Button>
					</Box>
					<Button
						borderRadius={"4px"}
						backgroundColor={"#095FBA"}
						color="white"
						fontSize={"16px"}
						fontWeight={400}
						ml={{ base: "", lg: "5%" }}
						_hover={{ backgroundColor: "#095FBA" }}
					>
						<Link href="/register">Register Pharmacy</Link>
					</Button>
				</Box>
				<Box mt="2%">
					<Text
						whiteSpace={"nowrap"}
						font-family=" Work Sans"
						fontSize={{ xl: "24px", md: "20px", base: "13px" }}
						fontWeight={400}
						color={"black"}
					>
						Our pharmacists are just a store visit away
					</Text>
				</Box>
				<Box
					display="flex"
					mt="3%"
					flexWrap={{ base: "wrap", lg: "nowrap" }}
					justifyContent={"center"}
				>
					<Input
						size={{ xl: "md", base: "sm" }}
						width={{ xl: "70%", lg: "70%", base: "50%" }}
						mx="3"
						my="3"
						name="pincode"
						backgroundColor={"white"}
						placeholder="Enter your Pincode"
						border="1px"
						borderColor={"#095FBA"}
						_placeholder={{ color: "gray.500" }}
					/>
					<Button
						backgroundColor={"#095FBA"}
						color="white"
						width="40%"
						size={{ xl: "md", base: "sm" }}
						mr="6%"
						mx="3"
						my="3"
						fontSize={"22px"}
						fontWeight={400}
						_hover={{
							backgroundColor: "white",
							color: "#095FBA",
							borderColor: "#095FBA",
							border: "1px",
						}}
					>
						Search
					</Button>
					<Button
						color="#008037"
						backgroundColor="white"
						border="1px"
						fontSize={{ xl: "16px", base: "14px" }}
						mx="3"
						my="3"
						size={{ xl: "md", base: "sm" }}
						width={{ xl: "70%", lg: "70%", base: "50%" }}
						display="flex"
						alignItems="center"
						justifyContent="center"
					>
						<Image src="Location icon.svg" marginRight="2%" />
						Use my location
					</Button>
				</Box>
				<Box
					width={{ xl: "70%", md: "100%", base: "100%" }}
					paddingTop="4%"
					paddingBottom="5%"
					overflowX="auto"
				>
					<Table
						backgroundColor="white"
						variant="simple"
						align="center"
						border="1px"
						borderColor="rgba(9, 95, 186, 0.18)"
					>
						<Tbody>
							<Tr>
								{["Pharmacy Name", "Address", "Contact", "Timings"].map(
									(label) => (
										<Td
											key={label}
											textAlign="center"
											fontSize={{ xl: "20px", base: "15px" }}
											lineHeight={{ lg: "24px", base: "none" }}
											paddingTop={{ lg: "2%", base: "1%" }}
											paddingBottom={{ lg: "1%", base: "0%" }}
										>
											{label}
										</Td>
									),
								)}
							</Tr>
						</Tbody>
						<Tbody>
							{[
								{
									id: 1,
									name: "Umiya Pharma",
									address:
										"Near, 9, 10, 11, Vishala 111 Industrial Estate, B/H Grand Vishala Industrial Estate, Odhav Rd, Odhav, Ahmedabad, Gujarat 382415",
									phone: "9856348765",
									hours: "9 am - 8 pm",
								},
								{
									id: 2,
									name: "Umiya Pharma",
									address:
										"Near, 9, 10, 11, Vishala 111 Industrial Estate, B/H Grand Vishala Industrial Estate, Odhav Rd, Odhav, Ahmedabad, Gujarat 382415",
									phone: "9876543210",
									hours: "10 am - 7 pm",
								},
							].map((data) => (
								<Tr key={data.id}>
									<Td
										textAlign="center"
										fontSize={{ xl: "18px", base: "14px" }}
										color="rgba(0, 0, 0, 0.49)"
										lineHeight={{ lg: "24px", base: "none" }}
									>
										{data.name}
									</Td>
									<Td
										textAlign="center"
										fontSize={{ xl: "18px", base: "14px" }}
										color="rgba(0, 0, 0, 0.49)"
										lineHeight={{ lg: "24px", base: "none" }}
									>
										{data.address}
									</Td>
									<Td
										textAlign="center"
										fontSize={{ xl: "18px", base: "14px" }}
										color="rgba(0, 0, 0, 0.49)"
										lineHeight={{ lg: "24px", base: "none" }}
									>
										{data.phone}
									</Td>
									<Td
										textAlign="center"
										fontSize={{ xl: "18px", base: "14px" }}
										color="rgba(0, 0, 0, 0.49)"
										lineHeight={{ lg: "24px", base: "none" }}
									>
										{data.hours}
									</Td>
								</Tr>
							))}
						</Tbody>
					</Table>
				</Box>
			</Box>
		</>
	);
};

export default page;
