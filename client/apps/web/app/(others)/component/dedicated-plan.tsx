import type React from "react";
import { useState } from "react";
import { Box, Flex, Text, Img } from "@chakra-ui/react";
import PriceCard from "./price-card";

const DedicatedPlan = () => {
	const [selectedCard, setSelectedCard] = useState<string | null>(null);
	const [acticveId, setActiveId] = useState<number | null>(0);
	const handleChange = (num: number) => {
		setActiveId(num);
	};

	return (
		<Flex
			flexDirection={{ md: "row", base: "column" }}
			justify="space-around"
			flexWrap={"wrap"}
			mt="8"
		>
			<Box
				// maxW="sm"
				// p="6"
				borderWidth="1px"
				borderRadius="lg"
				overflow="hidden"
				mt="18px"
				mr="18px"
				ml="18px"
				width={{ sm: "auto", md: "385px", lg: "385px" }}
				height={{ sm: "", md: "", lg: "530px" }}
				borderColor={"black"}
				textAlign="center"
				bgColor={"white"}
				color="black"
				// onClick={handleCardClick}
				cursor="pointer"
			>
				<Box border="1px">
					<Text
						textAlign={"left"}
						fontSize={"2xl"}
						as="h3"
						size="lg"
						pl={2}
						py={4}
						bgColor={"black"}
						color={"white"}
					>
						Select Category
					</Text>
					<Text
						textAlign={"left"}
						borderBottom={"1px"}
						onClick={() => handleChange(0)}
						fontSize={"2xl"}
						as="h3"
						size="lg"
						pl={2}
						py={2}
						bgColor={"white"}
						color={"black"}
						_hover={{ bg: "#095FBA", color: "white" }}
					>
						Patient Management
					</Text>
					<Text
						textAlign={"left"}
						borderTop={"1px"}
						borderBottom={"1px"}
						onClick={() => handleChange(1)}
						fontSize={"2xl"}
						as="h3"
						size="lg"
						pl={2}
						py={2}
						bgColor={"white"}
						color={"black"}
						_hover={{ bg: "#095FBA", color: "white" }}
					>
						EHR
					</Text>
				</Box>
				<Box mt={"56px"} display={"flex"} justifyContent={"center"}>
					<Img
						height={{ sm: "auto", md: "auto", lg: "300px" }}
						src="ladydoc.svg"
					/>
				</Box>
			</Box>
			{acticveId === 0 ? (
				<>
					<PriceCard
						title="Basic Plan"
						setSelectedCard={setSelectedCard}
						isSelected={selectedCard === "Basic Plan"}
						amount={3456}
						popularTag={false}
						userange={"10-99"}
					/>
					<PriceCard
						title="Premium Plan"
						setSelectedCard={setSelectedCard}
						isSelected={selectedCard === "Premium Plan"}
						amount={3456}
						popularTag={true}
						userange={"100-999"}
					/>
					<PriceCard
						title="Corporate Plan"
						setSelectedCard={setSelectedCard}
						isSelected={selectedCard === "Corporate Plan"}
						amount={3456}
						popularTag={false}
						userange={"1000-9999"}
					/>
				</>
			) : (
				<>
					<PriceCard
						title="Basic Plan 1"
						setSelectedCard={setSelectedCard}
						isSelected={selectedCard === "Basic Plan 1"}
						amount={3456}
						popularTag={false}
						userange={"10-99"}
					/>
					<PriceCard
						title="Premium Plan 1"
						setSelectedCard={setSelectedCard}
						isSelected={selectedCard === "Premium Plan 1"}
						amount={3456}
						popularTag={true}
						userange={"100-999"}
					/>
					<PriceCard
						title="Corporate Plan 1"
						setSelectedCard={setSelectedCard}
						isSelected={selectedCard === "Corporate Plan 1"}
						amount={3456}
						popularTag={false}
						userange={"1000-9999"}
					/>
				</>
			)}
		</Flex>
	);
};

export default DedicatedPlan;
