import { Box, Button, Heading, Image } from "@chakra-ui/react";
import SideArrow from "../assets/Side Arrow Icon.svg";
import { Link } from "@tanstack/react-router";

type PatientDetailImmunizationsCardProps = {
	heading: string;
	img_src: string;
	paths: string;
};
const PatientDetailImmunizationsCard = ({
	heading,
	img_src,
	paths,
}: PatientDetailImmunizationsCardProps) => {
	return (
		<div>
			<Box width={"100%"}>
				<Link to={paths}>
					<Box border="15px solid #F7F7F9">
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
								<Button
									border="1px solid #095FBA"
									bgColor="#095FBA"
									size="lg"
									height="100%"
									width="60px"
									padding={0}
								>
									<Image src={img_src} height="57px" width="30px" />
								</Button>
								<Box>
									<Heading as="h4" size="md" color="#095FBA" ml="2">
										{heading}
									</Heading>
								</Box>
							</Box>
							<Box pr={2}>
								<Image src={SideArrow} width={35} height={30} />
							</Box>
						</Box>
					</Box>
				</Link>
			</Box>
		</div>
	);
};

export default PatientDetailImmunizationsCard;
