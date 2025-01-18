import { CheckCircleIcon, CheckIcon } from "@chakra-ui/icons";
import { Box, Heading, Icon, Button, Text } from "@chakra-ui/react";

interface PriceCardProps {
	title: string;
	setSelectedCard: React.Dispatch<React.SetStateAction<string | null>>;
	isSelected: boolean;
	amount: number;
	popularTag: boolean;
	userange?: string;
}

const PriceCard: React.FC<PriceCardProps> = ({
	title,
	setSelectedCard,
	isSelected,
	amount,
	popularTag,
	userange,
}) => {
	// const userRange = "100-999";
	const description = "For most businesses that want to optimize web queries";
	const features = [
		{ id: 1, name: "Patient Portal" },
		{ id: 2, name: "Prescription" },
		{ id: 3, name: "EHR" },
	];

	const handleCardClick = () => {
		setSelectedCard(title);
	};

	return (
		<Box
			maxW="sm"
			borderWidth="1px"
			borderRadius="lg"
			overflow="hidden"
			mt="18px"
			mr="18px"
			ml="18px"
			borderColor={"black"}
			textAlign="center"
			bg={isSelected ? "#095FBA" : "white"}
			onClick={handleCardClick}
			cursor="pointer"
			transform={isSelected ? "translateY(-10px) scale(1.05)" : "none"}
			transition="box-shadow 0.3s ease-in-out, transform 0.3s ease-in-out"
		>
			{popularTag && (
				<Box display={"flex"} justifyContent={"end"}>
					<Text
						fontWeight={"bold"}
						size="lg"
						borderColor={isSelected ? "white" : "black"}
						border="2px"
						bg={"white"}
						color={"blue"}
						borderTop={"none"}
						p="2"
						borderRadius={"lg"}
						borderRight={"none"}
					>
						MOST POPULAR
					</Text>
				</Box>
			)}
			<Box p="2" m="2">
				<Heading
					textAlign={"center"}
					marginTop={popularTag ? "0px" : "40px"}
					marginX={{ sm: "", md: "", lg: "4" }}
					as="h3"
					size="lg"
					color={isSelected ? "white" : "black"}
				>
					{title}
				</Heading>
				<Text
					fontSize="4xl"
					fontWeight="bold"
					marginX={{ sm: "", md: "", lg: "4" }}
					color={isSelected ? "white" : "black"}
					textAlign={"center"}
				>
					₹{amount}
				</Text>
				<Text
					fontSize="sm"
					mb="4%"
					color={isSelected ? "white" : "black"}
					textAlign={"center"}
				>
					Per month / <br />
					User billed annually
				</Text>
				<Text
					fontSize={isSelected ? "lg" : "md"}
					mb="3%"
					p="2"
					marginX={{ sm: "", md: "", lg: "4" }}
					color={isSelected ? "white" : "black"}
					textAlign={"center"}
				>
					(Upto {userange} Users)
				</Text>
				<Text
					fontSize={isSelected ? "lg" : "md"}
					mb="10%"
					marginX={{ sm: "", md: "", lg: "4" }}
					color={isSelected ? "white" : "#192631E8"}
				>
					{description}
				</Text>
				<Box
					display="flex"
					flexDirection="column"
					alignItems="flex-start"
					ml="22%"
					mb="10%"
				>
					{features.map((feature) => (
						<Box key={feature.id} display="flex" alignItems="center">
							<Icon marginX={{ sm: "", md: "", lg: "4" }} boxSize={"16px"}>
								{isSelected ? (
									<CheckCircleIcon color={"white"} />
								) : (
									<CheckIcon color={"blue"} />
								)}{" "}
							</Icon>
							<Text
								// pb="4%"
								fontSize={isSelected ? "lg" : "md"}
								color={isSelected ? "white" : "black"}
							>
								{feature.name}
							</Text>
						</Box>
					))}
				</Box>
				<Button
					width="180px"
					height="56px"
					flex-shrink="0"
					border-radius=" 10px"
					border="1px solid #095FBA"
					backgroundColor={"white"}
					mb="2%"
				>
					<Text
						color=" #000"
						font-family="Open Sans"
						font-size="25px"
						font-style="normal"
						font-weight="400"
						line-height="18px"
						letter-spacing="0.078px"
					>
						Choose Plan
					</Text>
				</Button>
			</Box>
		</Box>
	);
};

export default PriceCard;
