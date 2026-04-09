"use client";
import { Box, Button, Heading, Input, Select, Text } from "@chakra-ui/react";
import React from "react";

const page = () => {
	return (
		<>
			<form
				style={{
					backgroundImage: 'url("Background Image.svg")',
					backgroundSize: "cover",
					backgroundRepeat: "no-repeat",
					backgroundPosition: "center",
					minHeight: "100vh",
				}}
			>
				<Box
					width={{ lg: "40%", md: "80%", base: "80%" }}
					marginLeft="auto"
					marginRight="auto"
					display="flex"
					flexDirection={{ md: "column", base: "column" }}
				>
					<Heading
						height="10%"
						width="100%"
						marginTop="10%"
						marginLeft="0"
						marginRight="0"
						fontSize={{ md: "32px", base: "12px", sm: "14px" }}
						textAlign="left"
						fontWeight={400}
					>
						Begin your Pi-Health Journey now !
					</Heading>
					<Box mt="10%" height="10%">
						<Text mb={"8px"}>Purpose</Text>
						<Select placeholder="Purpose" bg="white" color="gray.500">
							<option value="option1">
								I’m looking Software for Hospital or clinic.
							</option>
							<option value="option2">
								I’m a doctor, looking for patients.
							</option>
							<option value="option3">
								I’m a patient, want to book doctor's appointment.
							</option>
						</Select>
					</Box>
					<Box display={"flex"} flexDirection={{ md: "row", base: "column" }}>
						<Box mt="5%" height="6%" mr="6%" width={"100%"}>
							<Text mb={"8px"}>First Name</Text>
							<Input
								type="text"
								backgroundColor={"white"}
								height="50px"
								placeholder="Enter First Name"
								_placeholder={{ color: "gray.500" }}
							/>
						</Box>
						<Box mt="5%" height="6%" width={"100%"}>
							<Text mb={"8px"}>Last Name</Text>
							<Input
								type="text"
								backgroundColor={"white"}
								height="50px"
								placeholder=" Enter Last Name"
								_placeholder={{ color: "gray.500" }}
							/>
						</Box>
					</Box>

					<Box mt="5%" height="6%">
						<Text mb={"8px"}>Email Address</Text>
						<Input
							type="email"
							backgroundColor={"white"}
							_placeholder={{ color: "gray.500" }}
							placeholder="Enter Your Email Address"
						/>
					</Box>
					<Box mt="5%" height="6%">
						<Text mb={"8px"}>Phone Number</Text>
						<Input
							type="number"
							backgroundColor={"white"}
							placeholder="Enter Your Phone Number"
							_placeholder={{ color: "gray.500" }}
						/>
					</Box>
					<Box mt="5%" height="6%">
						<Text mb={"8px"}>Organization</Text>
						<Input
							type="text"
							backgroundColor={"white"}
							placeholder="Enter Your Organization"
							_placeholder={{ color: "gray.500" }}
						/>
					</Box>

					<Box>
						<Button
							size="lg"
							backgroundColor="#095FBA"
							color="white"
							width="100%"
							fontWeight={600}
							fontSize={{ md: "16px", base: "10px" }}
							_hover={{ backgroundColor: "#095FBA" }}
							mb="10%"
							mt="10%"
						>
							Get Started For Free
						</Button>
					</Box>
				</Box>
			</form>
		</>
	);
};

export default page;
