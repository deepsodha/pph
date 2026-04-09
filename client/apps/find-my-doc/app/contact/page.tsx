import {
	Box,
	Button,
	Heading,
	Input,
	Text,
	Textarea,
	Image,
} from "@chakra-ui/react";
import React from "react";
import Header from "../components/header";

const page = () => {
	return (
		<>
			<Header />
			<Box
				display={"flex"}
				paddingBottom={{ sm: "100px", base: "100px", lg: "23px" }}
				flexWrap={"wrap"}
				justifyContent={"space-around"}
				mr={"2%"}
				ml={"2%"}
				style={{
					backgroundImage: 'url("Background Image.svg")',
					backgroundSize: "cover",
					backgroundRepeat: "no-repeat",
					backgroundPosition: "center",
					// minHeight: "100vh",
				}}
			>
				<Box
					mr={"3%"}
					ml={"3%"}
					width={{ lg: "500px", md: "400px", base: "" }}
					mt={"2%"}
				>
					<Heading
						height="10%"
						width="100%"
						marginTop="10%"
						marginLeft="0"
						marginRight="0"
						fontSize={{ md: "40px", base: "30px", sm: "30px" }}
						textAlign={{ lg: "left", base: "center", sm: "center" }}
						fontWeight={"bold"}
						color={" #000000"}
					>
						Contact us
					</Heading>
					<Text
						marginLeft="0"
						marginRight="0"
						fontSize={{ lg: "17px", base: "12px", sm: "10px" }}
						textAlign={{ lg: "left", base: "center", sm: "center" }}
						mt={"10px"}
					>
						FindmyDoc simplifies the search for the perfect healthcare <br />
						professional. Connect with skilled physicians and specialists
						<br />
						effortlessly for your well-being.
					</Text>
					<Heading
						height="10%"
						width="100%"
						marginLeft="0"
						marginRight="0"
						fontSize={{ md: "40px", base: "25px", sm: "20px" }}
						textAlign={{ lg: "left", base: "center", sm: "center" }}
						fontWeight={"bold"}
						color={"#000000"}
						mt={"10px"}
						mb={"10px"}
					>
						Our Office
					</Heading>
					<Box
						display={"flex"}
						flexDirection={"column"}
						alignItems={{ lg: "flex-start", md: "center", sm: "center" }}
						justifyContent={{ lg: "space-around" }}
						marginBottom={{ lg: "23px", md: "center", sm: "100px" }}
					>
						<Box display={"flex"} mt="2px" mb="20px">
							<Image src="locationIcon.svg " alt="Location" />
							<Text ml="8px">1001, Satyamev Elite, Ahmedabad</Text>
						</Box>
						<Box display={"flex"} mt="2px" mb="20px">
							<img src="email icon.svg" alt="Location" />
							<Text ml="8px">support@pihealth.com</Text>
						</Box>
						<Box display={"flex"} mt="2px" mb="20px">
							<img src="callIcon.svg" alt="Location" />
							<Text ml="8px">(+91) 9775437689</Text>
						</Box>
					</Box>
				</Box>
				<Box>
					<form>
						<Box
							width={{ lg: "650px", md: "", base: "" }}
							display="flex"
							flexDirection={{ md: "column", base: "column" }}
						>
							<Heading
								height="10%"
								width="100%"
								marginTop="10%"
								marginLeft="0"
								marginRight="0"
								fontSize={{ md: "40px", base: "20px", sm: "20px" }}
								textAlign="left"
								fontWeight={"bold"}
								color={"#51A7D6"}
								mb="6"
							>
								Let's Discuss
							</Heading>
							<Heading
								height="10%"
								width="100%"
								marginLeft="0"
								marginRight="0"
								fontSize={{ md: "32px", base: "20px", sm: "20px" }}
								textAlign="left"
								fontWeight={400}
							>
								Feel free to ask for details, don't save any questions!
							</Heading>

							<Box
								display={"flex"}
								flexDirection={{ md: "row", base: "column" }}
							>
								<Box mt="5%" height="6%" mr="6%" width={"100%"}>
									<Text mb={"8px"}>First Name</Text>
									<Input
										type="text"
										backgroundColor={"white"}
										placeholder="Enter First Name"
										_placeholder={{ color: "gray.500" }}
										required
									/>
								</Box>
								<Box mt="5%" height="6%" width={"100%"}>
									<Text mb={"8px"}>Email Address</Text>
									<Input
										type="email"
										backgroundColor={"white"}
										placeholder=" Enter your email"
										_placeholder={{ color: "gray.500" }}
										required
									/>
								</Box>
							</Box>

							<Box mt="5%" height="7%" width={"100%"}>
								<Text mb={"8px"}>Subject</Text>
								<Input
									type="text"
									backgroundColor={"white"}
									placeholder="Enter Subject"
									_placeholder={{ color: "gray.500" }}
									required
								/>
							</Box>
							<Box mt="5%" height="6%" width={"100%"}>
								<Text mb={"8px"}>Message</Text>
								<Textarea
									backgroundColor={"white"}
									placeholder="Enter Message"
									_placeholder={{ color: "gray.500" }}
									required
								/>
							</Box>

							<Box>
								<Button
									size="lg"
									backgroundColor="#51A7D6"
									color="white"
									width={{ sm: "100%", base: "100%", md: "100%", lg: "220px" }}
									fontWeight={600}
									fontSize={{ md: "16px", base: "10px" }}
									_hover={{ backgroundColor: "#095FBA" }}
									mb="10%"
									mt="6%"
								>
									Send Message
								</Button>
							</Box>
						</Box>
					</form>
				</Box>
			</Box>
		</>
	);
};

export default page;
