import { Box, Button, Heading, Input, Text, Textarea } from "@chakra-ui/react";
import React from "react";

const page = () => {
	return (
		<>
			<Box pb="10" pt="10" bgColor={"blue"}>
				<Heading
					height="10%"
					width="100%"
					marginLeft="0"
					marginRight="0"
					fontSize={{ md: "40px", base: "26px", sm: "20px" }}
					textAlign="center"
					fontWeight={"bold"}
					color={"white"}
					pb="6"
					pt="6"
				>
					Contact Us
				</Heading>
				<Text
					fontSize={{ lg: "20px", base: "20px", sm: "20px" }}
					textAlign="center"
					color={"white"}
				>
					Feel free to contact us any time. We will get back to you as soon as
					we can.
				</Text>
			</Box>
			<Box
				display={"flex"}
				paddingBottom={{ sm: "100px", base: "100px", lg: "23px" }}
				flexWrap={"wrap"}
				justifyContent={"center"}
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
				<Box>
					<form>
						<Box
							width={{ lg: "700px", md: "", base: "" }}
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
								color={"#095FBA"}
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
										height="50px"
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
										height="50px"
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
									height="50px"
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
									height="50px"
									_placeholder={{ color: "gray.500" }}
									required
								/>
							</Box>

							<Box>
								<Button
									size="lg"
									backgroundColor="#095FBA"
									color="white"
									width={{ sm: "100%", base: "100%", md: "100%", lg: "220px" }}
									fontWeight={600}
									height="50px"
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
				<Box mr={"3%"} ml={"3%"} width={"700px"}>
					<Heading
						height="10%"
						width="100%"
						marginTop="10%"
						marginLeft="0"
						marginRight="0"
						fontSize={{ md: "40px", base: "30px", sm: "30px" }}
						textAlign={{ lg: "left", base: "center", sm: "center" }}
						fontWeight={"bold"}
						color={"#095FBA"}
					>
						Get in Touch
					</Heading>
					<Text
						marginLeft="0"
						marginRight="0"
						fontSize={{ lg: "17px", base: "12px", sm: "10px" }}
						textAlign={{ lg: "left", base: "center", sm: "center" }}
					>
						Pi-Health is revolutionizing the medical sector. With advanced
						features like Electronic Health Records (EHR), patient management
						tools, and more, Pi-Health streamlines workflows and enhances
						patient care. Our user-friendly interface ensures seamless
						integration into your practice, while robust security measures
						safeguard patient data.Join the growing community of healthcare
						providers who trust Pi-Health to streamline their operations,
						improve patient care, and drive better outcomes. Get in touch with
						us today to learn more about how Pi-Health can transform your
						practice. Together, let's redefine healthcare for the digital age.
					</Text>
					<Text
						marginTop="2%"
						marginLeft="0"
						marginRight="0"
						fontSize={{ lg: "17px", base: "12px", sm: "10px" }}
						textAlign={{ lg: "left", base: "center", sm: "center" }}
						mb="6"
					>
						Join the growing community of healthcare providers who trust
						Pi-Health to streamline their operations, improve patient care, and
						drive better outcomes. Get in touch with us today to learn more
						about how Pi-Health can transform your practice. Together, let's
						redefine healthcare for the digital age.
					</Text>
					<Heading
						height="10%"
						width="100%"
						marginLeft="0"
						marginRight="0"
						fontSize={{ md: "40px", base: "25px", sm: "20px" }}
						textAlign={{ lg: "left", base: "center", sm: "center" }}
						fontWeight={"bold"}
						color={"#095FBA"}
					>
						Our Office
					</Heading>
					<Box
						display={"flex"}
						flexDirection={"column"}
						alignItems={{ lg: "flex-start", md: "center", sm: "center" }}
						justifyContent={{ lg: "center" }}
						marginBottom={{ lg: "23px", md: "center", sm: "100px" }}
					>
						<Box display={"flex"} mt="2px" mb="20px">
							<img src="locationiconpi.png" alt="Location" />
							<Text ml="8px">1001, Satyamev Elite, Ahmedabad</Text>
						</Box>
						<Box display={"flex"} mt="2px" mb="20px">
							<img src="emailiconpi.png" alt="Location" />
							<Text ml="8px">support@pihealth.com</Text>
						</Box>
						<Box display={"flex"} mt="2px" mb="20px">
							<img src="calliconpi.png" alt="Location" />
							<Text ml="8px">(+91) 9775437689</Text>
						</Box>
					</Box>
				</Box>
			</Box>
		</>
	);
};

export default page;
