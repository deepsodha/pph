import { Box, Center, Heading, Image } from "@chakra-ui/react";
import { Link } from "@tanstack/react-router";
import SideArrow from "../assets/Side Arrow White.svg";

type serviceLocationProps = {
	path: string;
};
const ServiceLocationCard = ({ path }: serviceLocationProps) => {
	return (
		<div>
			<Center>
				<Box width={"100%"} height="100%">
					<Box mt={5} border="15px solid #FFFFFF" borderRadius={"8px"}>
						<Box
							border="2px solid #095FBA"
							bgColor={"#FFFFFF"}
							height="60px"
							display={"flex"}
							alignItems={"center"}
							justifyContent={"space-between"}
							borderRadius={"10px"}
						>
							<Box display={"flex"} alignItems={"center"}>
								<Box>
									<Heading as="h4" size="md" color="#095FBA" ml="2">
										Service Location
									</Heading>
								</Box>
							</Box>
							<Box
								pr={2}
								width={16}
								height="100%"
								display="flex"
								alignItems="center"
								justifyContent="center"
								borderBottomRightRadius={"8px"}
								borderTopRightRadius={"8px"}
								backgroundColor={"#095FBA"}
							>
								<Link to={path}>
									<Image src={SideArrow} height={38} width={39} ml={"4%"} />
								</Link>
							</Box>
						</Box>
					</Box>
				</Box>
			</Center>
		</div>
	);
};

export default ServiceLocationCard;
