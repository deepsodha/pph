import {
	Box,
	Image,
	Card,
	Heading,
	CardHeader,
	Text,
	CardBody,
} from "@chakra-ui/react";
interface CarouseCard {
	heading: string;
	content: string;
	margin: string;
}

const CarouselCard: React.FC<CarouseCard> = ({ heading, content, margin }) => {
	return (
		<Box
			mt="5%"
			width={{ base: "80%", md: "100%" }}
			key={margin}
			ml={{ base: "10%", md: "0%" }}
		>
			<Card
				color={"black"}
				variant="outline"
				bgColor="transparent"
				border="1px solid black"
				width={{ base: "100%", md: "107%" }}
				ml={{ base: "0%", md: margin }}
				mb={{ base: "15%", lg: "%" }}
			>
				<CardHeader pb="0%" mb="0%">
					<Box display="flex" flexDirection="row">
						<Image src="card image.svg" alt="doctor" />
						<Heading size="md" fontWeight={500} fontSize="32px" ml="5%">
							{heading}
						</Heading>
					</Box>
				</CardHeader>
				<CardBody ml="25%" p="0%" mt="0%" mb="5%">
					<Text
						fontSize="16px"
						fontWeight={400}
						textAlign="left"
						width="80%"
						lineHeight="24px"
					>
						{content}
					</Text>
				</CardBody>
			</Card>
		</Box>
	);
};

export default CarouselCard;
