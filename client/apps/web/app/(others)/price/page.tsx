"use client";
import { Box, Button, Text } from "@chakra-ui/react";
import React, { useState } from "react";
import Advertise from "../component/advertise";
import Contact from "../component/contact";
import SharedPlan from "../component/shared-plan";
import DedicatedPlan from "../component/dedicated-plan";

const Page = () => {
	const [activeButton, setActiveButton] = useState(0);

	const handleTabChange = (index: number) => {
		setActiveButton(index);
	};

	return (
		<>
			<Box
				mt="0%"
				width="100%"
				display={"flex"}
				flexDirection={"column"}
				justifyContent={"center"}
				alignItems={"center"}
				backgroundImage='url("Background Image.svg")'
				backgroundSize="cover"
				backgroundRepeat="no-repeat"
				backgroundPosition="center"
				minHeight="100vh"
			>
				<Box
					display={"flex"}
					pt="2%"
					flexDirection={"column"}
					width="35%"
					height="3%"
					justifyContent={"center"}
					alignItems={"center"}
				>
					<Text
						fontSize={{ md: "40px", base: "20px" }}
						fontWeight={700}
						textAlign={"center"}
					>
						Our pricing plans
					</Text>
					<Text pt="3%" color="#6F6C90" textAlign={"center"}>
						Lorem ipsum dolor sit amet consectetur adipiscing elit dolor posuere
						vel venenatis eu sit massa volutpat.
					</Text>
				</Box>

				<Box
					mt="3%"
					borderRadius="25px"
					boxShadow="0px 4px 4px 5px rgba(0, 0, 0, 0.25) "
					backgroundColor={"white"}
				>
					<Button
						borderRadius="70px"
						boxShadow="md"
						bg={activeButton === 0 ? " #095FBA" : "white"}
						color={activeButton === 0 ? "white" : " #095FBA"}
						onClick={() => handleTabChange(0)}
					>
						Shared
					</Button>
					<Button
						borderRadius="70px"
						boxShadow="md"
						bg={activeButton === 1 ? "#095FBA" : "white"}
						color={activeButton === 1 ? "white" : "#095FBA"}
						onClick={() => handleTabChange(1)}
					>
						Dedicated
					</Button>
				</Box>

				<Box>{activeButton === 0 ? <SharedPlan /> : <DedicatedPlan />}</Box>

				<Box display={"flex"} flexDirection={"column"} m="3%">
					<Text>* Local law taxes must be paid </Text>
					<Text>* Plans must be paid annually </Text>
					<Text>
						* The aforementioned INR prices apply only to India, whereas the USD
						pricing applies to countries other than India.
					</Text>
					<Text>
						* Additional providers with or without login access will incur
						additional charges
					</Text>
					<Text>* Patient App will incur additional charges </Text>
					<Text>* Telehealth App pricing excludes Telecall expenses </Text>
					<Text>
						* Yearly Subscription charges are subject to annual change to
						account for inflation costs
					</Text>
				</Box>
			</Box>
			<Contact />
			<Advertise />
		</>
	);
};

export default Page;
