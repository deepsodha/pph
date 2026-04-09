"use client";
import {
	ChakraProvider,
	Box,
	Link,
	Heading,
	Input,
	RadioGroup,
	Stack,
	Radio,
	Button,
	Image,
	Text,
	Select,
	useDisclosure,
} from "@chakra-ui/react";
import { type SetStateAction, useState } from "react";

interface Props {
	backgroundImage: string;
	logoImage: string;
	heading: string;
	continueButtonColour: string;
	linkTextColour: string;
	inputHeading: string;
	inputType: string;
	inputPlaceHolder: string;
	isVisible?: boolean;
}
export const SignUp: React.FC<Props> = ({
	backgroundImage,
	logoImage,
	heading,
	continueButtonColour,
	linkTextColour,
	inputHeading,
	inputType,
	inputPlaceHolder,
	isVisible,
}) => {
	const [role, setRole] = useState(""); // State to store user's role
	const { onToggle } = useDisclosure();

	const handleRoleChange = (event: {
		target: { value: SetStateAction<string> };
	}) => {
		setRole(event.target.value);
		onToggle(); // Toggle visibility based on role selection
	};

	return (
		<ChakraProvider>
			<>
				<form>
					<Box mt="2%" pb="5%">
						<Link href="/">
							{" "}
							<Image src={logoImage} marginLeft="7%" />
						</Link>
					</Box>
					<Box
						display="flex"
						justifyContent="center"
						alignItems="center"
						flexDirection={"column"}
						pt="2%"
						style={{
							backgroundImage: `url("${backgroundImage}")`,
							backgroundSize: "cover",
							backgroundRepeat: "no-repeat",
							backgroundPosition: "center",
							minHeight: "100vh",
						}}
					>
						<Box width={{ sm: "40%", md: "55%", lg: "42%", base: "80%" }}>
							<Box>
								<Heading
									as="h3"
									size="xl"
									pb="5%"
									fontSize={{ md: "40px", base: "30px" }}
									fontWeight={"bold"}
								>
									{heading}
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
									size="md"
									variant="outline"
									type="email"
									height="50px"
									backgroundColor={"white"}
									placeholder="Enter your email"
									_placeholder={{ color: "gray.500" }}
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
										FirstName
									</Heading>
									<Input
										size="md"
										variant="outline"
										type="text"
										height="50px"
										backgroundColor={"white"}
										placeholder="Enter your First Name"
										_placeholder={{ color: "gray.500" }}
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
										LastName
									</Heading>
									<Input
										size="md"
										variant="outline"
										type="text"
										height="50px"
										backgroundColor={"white"}
										placeholder="Enter your Last Name"
										_placeholder={{ color: "gray.500" }}
									/>
								</Box>
							</Box>

							<Box mt={"5%"}>
								<Heading
									as="h6"
									size="sm"
									pb="1%"
									fontSize={"20px"}
									fontWeight={400}
								>
									{inputHeading}
								</Heading>

								<Input
									size="md"
									variant="outline"
									type={inputType}
									placeholder={inputPlaceHolder}
									color={"gray.500"}
									height="50px"
									_placeholder={{ color: "black" }}
									backgroundColor={"white"}
								/>
							</Box>
							<Box display={isVisible ? "block" : "none"}>
								<Box>
									<Heading
										as="h6"
										size="sm"
										pb="1%"
										fontSize={"20px"}
										fontWeight={400}
										mt="5%"
									>
										Role
									</Heading>

									<Select
										placeholder="Select Role"
										size="md"
										variant="outline"
										backgroundColor={"white"}
										color={"black"}
										// _placeholder={{ color: "#71717A" }}
										_placeholder={{ color: "gray.500" }}
										onChange={handleRoleChange}
									>
										<option value="true">Doctor</option>
										<option value="false">User</option>
									</Select>
								</Box>
								{role === "true" && (
									<>
										<Box>
											<Heading
												as="h6"
												size="sm"
												pb="1%"
												fontSize={"20px"}
												fontWeight={400}
												mt="5%"
											>
												Location
											</Heading>

											<Input
												size="md"
												variant="outline"
												type="text"
												height="50px"
												backgroundColor={"white"}
												placeholder="Enter Location"
												_placeholder={{ color: "#71717A" }}
											/>
										</Box>

										<Box>
											<Heading
												as="h6"
												size="sm"
												pb="1%"
												fontSize={"20px"}
												fontWeight={400}
												mt="5%"
											>
												Speciality
											</Heading>

											<Input
												size="md"
												variant="outline"
												height="50px"
												type="text"
												backgroundColor={"white"}
												placeholder="Enter Speciality"
												_placeholder={{ color: "#71717A" }}
											/>
										</Box>

										<Box>
											<Heading
												as="h6"
												size="sm"
												pb="1%"
												fontSize={"20px"}
												fontWeight={400}
												mt="5%"
											>
												Treatment Type
											</Heading>

											<Input
												size="md"
												variant="outline"
												type="text"
												height="50px"
												backgroundColor={"white"}
												placeholder="Enter Treatment Type"
												_placeholder={{ color: "#71717A" }}
											/>
										</Box>
									</>
								)}
							</Box>
							<Box mt="5%">
								<Heading
									as="h6"
									size="sm"
									pb="2%"
									fontSize={"20px"}
									fontWeight={400}
								>
									Gender
								</Heading>
								<Box
									border=" 1px solid #D9D9D9"
									px={"5%"}
									py={"3%"}
									backgroundColor={"white"}
									rounded={"lg"}
								>
									<RadioGroup defaultValue="1" color={"black"}>
										<Stack spacing={4} direction="column">
											<Radio color={"black"} value="1">
												{" "}
												Male
											</Radio>
											<Radio color={"black"} value="2">
												Female
											</Radio>
											<Radio color={"black"} value="3">
												Other
											</Radio>
										</Stack>
									</RadioGroup>
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
									backgroundColor={continueButtonColour}
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
									color={"black"}
									fontSize={{ base: "md", md: "sm" }}
									mt={{ base: "8%", md: "5%" }}
									mb={{ base: "8%", md: "4%" }}
									height="50px"
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
							<Link mb="8%" href="/sign-in" color={linkTextColour}>
								{" "}
								Login
							</Link>
						</Box>
					</Box>
				</form>
			</>
		</ChakraProvider>
	);
};
