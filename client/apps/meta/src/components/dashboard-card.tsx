import { Box, Button, Heading, Image } from "@chakra-ui/react";
import { Link } from "@tanstack/react-router";
import Arrow from "../assets/Side Arrow Icon.svg";
type DashboardCardProps = {
	heading: string;
	img_src: string;
	paths: string;
};
const DashboardCard = ({ heading, img_src, paths }: DashboardCardProps) => {
	return (
		<div>
			<Box width={"100%"}>
				<Link to={paths}>
					<Box border="15px solid #F7F7F9" width={"100%"}>
						<Box
							border="2px solid #095FBA"
							bgColor={"#FFFFFF"}
							height="60px"
							width={"100%"}
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
								>
									<Image src={img_src} height="56px" width="50px" />
								</Button>
								<Box>
									<Heading as="h2" size="md" color="#095FBA" ml="12">
										{heading}
									</Heading>
								</Box>
							</Box>
							<Box pr={2}>
								<Image src={Arrow} width={35} height={30} />
							</Box>
						</Box>
					</Box>
				</Link>
			</Box>
		</div>
	);
};

export default DashboardCard;
