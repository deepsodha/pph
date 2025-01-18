import {
	ChakraProvider,
	Box,
	Link,
	Heading,
	Input,
	Button,
	Image,
	Text,
	Select,
} from "@chakra-ui/react";

const page: React.FC = () => {
	return (
		<ChakraProvider>
			<form
				style={{
					backgroundImage: `url("Sign bg.svg")`,
					backgroundSize: "cover",
					backgroundRepeat: "no-repeat",
					backgroundPosition: "center",
					minHeight: "100vh",
				}}
			>
				<Box
					display="flex"
					justifyContent="center"
					alignItems="center"
					flexDirection={"column"}
					pt="2%"
				>
					<Box mr={"80%"} mt="2%" pb="3%">
						<Link href="/">
							{" "}
							<Image src="Logo - 2 3.svg" marginLeft="20%" />
						</Link>
					</Box>

					<Box width={{ sm: "40%", md: "55%", lg: "42%", base: "80%" }}>
						<Box>
							<Heading
								as="h5"
								size="xl"
								pb="8%"
								fontSize={"40px"}
								fontWeight={400}
							>
								Create an account
							</Heading>
							<Heading
								as="h6"
								size="sm"
								pb="1%"
								fontSize={"20px"}
								fontWeight={400}
							>
								Email
							</Heading>

							<Input
								size="lg"
								variant="outline"
								type="email"
								backgroundColor={"white"}
								placeholder="Enter your Email id"
							/>
						</Box>

						<Box
							display="flex"
							flexDirection={{ base: "column", md: "row" }}
							mt={{ base: "4%", md: "2%" }}
							gap={{ base: 0, md: 6 }}
							justifyContent="space-around"
							//ml={{ base: "25%", md: "2%" }}
							pt={"4%"}
						>
							<Box
								display="flex"
								flexDirection="column"
								width={{ base: "100%", md: "55%", lg: "50%" }}
							>
								<Heading
									as="h6"
									size="sm"
									pb={{ base: "4%", md: "2%" }}
									fontSize={"20px"}
									fontWeight={400}
								>
									First Name
								</Heading>
								<Input
									size="lg"
									variant="outline"
									type="text"
									backgroundColor={"white"}
									placeholder="Enter your first name"
								/>
							</Box>

							<Box
								display="flex"
								flexDirection="column"
								width={{ base: "100%", md: "55%", lg: "50%" }}
								mt={{ base: "5%", md: 0 }}
							>
								<Heading
									as="h6"
									size="sm"
									pb={{ base: "4%", md: "2%" }}
									fontSize={"20px"}
									fontWeight={400}
								>
									Last Name
								</Heading>
								<Input
									size="lg"
									variant="outline"
									type="text"
									backgroundColor={"white"}
									placeholder="Enter your last name"
								/>
							</Box>
						</Box>
						<Box
							display="flex"
							flexDirection={{ base: "column", md: "row" }}
							mt={{ base: "4%", md: "2%" }}
							gap={{ base: 0, md: 6 }}
							justifyContent="space-around"
							//ml={{ base: "25%", md: "2%" }}
							pt={"4%"}
						>
							<Box
								display="flex"
								flexDirection="column"
								width={{ base: "100%", md: "55%", lg: "50%" }}
							>
								<Heading
									as="h6"
									size="sm"
									pb={{ base: "4%", md: "2%" }}
									fontSize={"20px"}
									fontWeight={400}
								>
									Your Phone Number
								</Heading>
								<Input
									size="lg"
									variant="outline"
									type="number"
									backgroundColor={"white"}
									placeholder="Enter your phone number"
								/>
							</Box>

							<Box
								display="flex"
								flexDirection="column"
								width={{ base: "100%", md: "55%", lg: "50%" }}
								mt={{ base: "5%", md: 0 }}
							>
								<Heading
									as="h6"
									size="sm"
									pb={{ base: "4%", md: "2%" }}
									fontSize={"20px"}
									fontWeight={400}
								>
									Role
								</Heading>
								<Select
									backgroundColor="white"
									size="lg"
									placeholder="Enter your role"
								>
									<option value="owner">Owner</option>
									<option value="staff">Staff</option>
								</Select>
							</Box>
						</Box>
						<Box
							display="flex"
							flexDirection={{ base: "column", md: "row" }}
							mt={{ base: "4%", md: "2%" }}
							gap={{ base: 0, md: 6 }}
							justifyContent="space-around"
							//ml={{ base: "25%", md: "2%" }}
							pt={"4%"}
						>
							<Box
								display="flex"
								flexDirection="column"
								width={{ base: "100%", md: "55%", lg: "50%" }}
							>
								<Heading
									as="h6"
									size="sm"
									pb={{ base: "4%", md: "2%" }}
									fontSize={"20px"}
									fontWeight={400}
								>
									Pharmacy Name
								</Heading>
								<Input
									size="lg"
									variant="outline"
									type="text"
									backgroundColor={"white"}
									placeholder="Enter pharmacy name"
								/>
							</Box>

							<Box
								display="flex"
								flexDirection="column"
								width={{ base: "100%", md: "55%", lg: "50%" }}
								mt={{ base: "5%", md: 0 }}
							>
								<Heading
									as="h6"
									size="sm"
									pb={{ base: "4%", md: "2%" }}
									fontSize={"20px"}
									fontWeight={400}
								>
									Pharmacy Contact Number
								</Heading>
								<Input
									size="lg"
									variant="outline"
									type="text"
									backgroundColor={"white"}
									placeholder="Enter  contact number"
								/>
							</Box>
						</Box>

						<Heading
							as="h6"
							size="sm"
							pt={{ base: "4%", md: "2%" }}
							pb={{ base: "2%", md: "1%" }}
							fontSize={"20px"}
							fontWeight={400}
							mt={{ base: "4%", md: "2%" }}
						>
							Pharmacy Address
						</Heading>
						<Input
							size="lg"
							variant="outline"
							type="text"
							backgroundColor={"white"}
							placeholder="Enter Pharmacy address"
						/>
						<Box
							display="flex"
							flexDirection={{ base: "column", md: "row" }}
							mt={{ base: "4%", md: "2%" }}
							gap={{ base: 0, md: 6 }}
							justifyContent="space-around"
							//ml={{ base: "25%", md: "2%" }}
							pt={"4%"}
						>
							<Box
								display="flex"
								flexDirection="column"
								width={{ base: "100%", md: "55%", lg: "50%" }}
							>
								<Heading
									as="h6"
									size="sm"
									pb={{ base: "4%", md: "2%" }}
									fontSize={"20px"}
									fontWeight={400}
								>
									Pharmacy Timings
								</Heading>
								<Box display="flex">
									<Input
										size="lg"
										variant="outline"
										type="text"
										width="120px"
										backgroundColor={"white"}
										placeholder="Start Time"
									/>
									<Text
										fontSize={"18px"}
										fontWeight={400}
										ml="6%"
										mr="6%"
										mt="2%"
									>
										To
									</Text>
									<Input
										size="lg"
										variant="outline"
										type="text"
										width="120px"
										backgroundColor={"white"}
										placeholder="End Time"
									/>
								</Box>
							</Box>

							<Box
								display="flex"
								flexDirection="column"
								width={{ base: "100%", md: "55%", lg: "50%" }}
								mt={{ base: "5%", md: 0 }}
								ml="3%"
							>
								<Heading
									as="h6"
									size="sm"
									pb={{ base: "4%", md: "2%" }}
									fontSize={"20px"}
									fontWeight={400}
								>
									Days
								</Heading>
								<Box display="flex">
									<Input
										size="lg"
										variant="outline"
										type="text"
										backgroundColor={"white"}
										width="120px"
										placeholder="Monday"
									/>{" "}
									<Text
										fontSize={"18px"}
										ml="6%"
										mr="6%"
										mt="2%"
										fontWeight={400}
									>
										To
									</Text>
									<Input
										size="lg"
										variant="outline"
										type="text"
										width="120px"
										backgroundColor={"white"}
										placeholder="Friday"
									/>
								</Box>
							</Box>
						</Box>

						<Box
							width="100%"
							display="flex"
							justifyContent={"center"}
							alignItems="center"
							pt={"5%"}
						>
							<Button
								size="lg"
								backgroundColor="#095FBA"
								width="100%"
								color="white"
								mt={{ base: "12%", md: "5%" }}
								mb={{ base: "8%", md: "5%" }}
							>
								Continue
							</Button>
						</Box>

						<Box
							display="flex"
							alignItems="center"
							pb={{ base: "1%", md: "1%" }}
						>
							<Box flex={1} height="1px" bg="#CCCCCC" mx="2" />
							<Box>or</Box>
							<Box flex={1} height="1px" bg="#CCCCCC" mx="2" />
						</Box>

						<Box
							width="100%"
							display="flex"
							justifyContent="center"
							alignItems="center"
						>
							<Button
								size="lg"
								backgroundColor="white"
								border=" 1px solid #D9D9D9"
								width="100%"
								fontSize={{ base: "md", md: "sm" }}
								mt={{ base: "8%", md: "5%" }}
								mb={{ base: "8%", md: "4%" }}
								leftIcon={<Image src="Vector.svg" />}
							>
								Continue With Google
							</Button>
						</Box>
					</Box>

					<Box display="flex" justifyContent="center">
						<Text fontSize="16px" mr="5px" mb="8%">
							Already have an account?{" "}
						</Text>
						<Link mb="8%" href="/login" color="#095FBA">
							{" "}
							Login
						</Link>
					</Box>
				</Box>
			</form>
		</ChakraProvider>
	);
};
export default page;
