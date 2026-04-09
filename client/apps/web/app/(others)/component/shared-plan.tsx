import type React from "react";
import { useState } from "react";
import { Flex, Box, Text, Img } from "@chakra-ui/react";
import PriceCard from "./price-card";

const SharedPlan: React.FC = () => {
	const [selectedCard, setSelectedCard] = useState<string | null>(null);
	const [activeId, setActiveId] = useState<number | null>(0);
	const handleChange = (num: number) => {
		setActiveId(num);
	};

	return (
		<>
			<Flex
				flexDirection={{ md: "row", base: "column" }}
				justify="space-around"
				flexWrap={"wrap"}
				mt="8"
			>
				<Box
					// maxW="sm"
					// p="6"
					// onClick={handleCardClick}
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
							borderBottom="1px"
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
						<Img height={"300px"} src="ladydoc.svg" />
					</Box>
				</Box>

				{activeId === 0 ? (
					<>
						<PriceCard
							title="Single User"
							setSelectedCard={setSelectedCard}
							isSelected={selectedCard === "Single User"}
							amount={3456}
							popularTag={false}
							userange={"1"}
						/>
						<PriceCard
							title="Single Clinic"
							amount={3456}
							setSelectedCard={setSelectedCard}
							isSelected={selectedCard === "Single Clinic"}
							popularTag={true}
							userange={"9"}
						/>
					</>
				) : (
					<>
						<PriceCard
							title="Single  1"
							setSelectedCard={setSelectedCard}
							isSelected={selectedCard === "Single  1"}
							amount={3456}
							popularTag={false}
							userange={"1"}
						/>
						<PriceCard
							title="Single Clinic 1"
							amount={3456}
							setSelectedCard={setSelectedCard}
							isSelected={selectedCard === "Single Clinic 1"}
							popularTag={true}
							userange={"9"}
						/>
					</>
				)}
			</Flex>
		</>
	);
};

export default SharedPlan;
