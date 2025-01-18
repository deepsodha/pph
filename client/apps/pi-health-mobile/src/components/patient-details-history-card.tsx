import { Box, Heading, Image } from "@chakra-ui/react";
import ForwardArrow from "../assets/ForwardArrow.svg";
import { Link } from "@tanstack/react-router";

type PatientDetailHistoryCardProps = {
	heading: string;
	paths: string;
};
const PatientDetailHistoryCard = ({
	heading,
	paths,
}: PatientDetailHistoryCardProps) => {
	return (
		<div>
			<Box>
				<Box border="10px solid #FFFFFF" mt={6} mb={6} borderRadius={"10px"}>
					<Box
						border="2px solid #095FBA"
						bgColor={"#095FBA"}
						height="60px"
						display={"flex"}
						alignItems={"center"}
						justifyContent={"space-between"}
						borderRadius={"10px"}
					>
						<Box
							display={"flex"}
							alignItems={"center"}
							bgColor={"#FFFFFF"}
							width="100%"
							height="97%"
							borderRightRadius="10px"
							borderLeftRadius="9px"
						>
							<Heading as="h4" size="md" color="#095FBA" ml="2">
								{heading}
							</Heading>
						</Box>
						<Box
							bg="#095FBA"
							rounded="md"
							width={16}
							height={8}
							cursor={"pointer"}
							pl={3}
						>
							<Link to={paths}>
								<Image src={ForwardArrow} width={35} height={30} />
							</Link>
						</Box>
					</Box>
				</Box>
			</Box>
		</div>
	);
};

export default PatientDetailHistoryCard;
