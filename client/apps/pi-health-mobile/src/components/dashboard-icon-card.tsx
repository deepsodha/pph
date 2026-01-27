import {
	Card,
	CardBody,
	Center,
	Box,
	Heading,
	Image,
	extendTheme,
	ChakraProvider,
} from "@chakra-ui/react";
import { Link } from "@tanstack/react-router";

type DashboardIconCardProps = {
	title: string;
	icon: string;
	heading: string;
};
const breakpoints = {
	base: "0px",
	sm: "313px",
	md: "500px",
};

const customTheme = extendTheme({ breakpoints });
const DashboardIconCard = ({
	title,
	icon,
	heading,
}: DashboardIconCardProps) => {
	return (
		<div>
			<ChakraProvider theme={customTheme}>
				<Link to={`/${title}`}>
					<Card
						borderRadius="10px"
						border={"1px"}
						height={{ base: "130px", sm: "120px" }}
						borderColor={"#095FBA"}
					>
						<CardBody
							display="flex"
							flexDirection="column"
							justifyContent="center"
							alignItems="center"
						>
							<Center>
								<Box display="flex" alignItems="center" justifyContent="center">
									<Center>
										<Image src={icon} height={14} mb="14%" width={14} />
									</Center>
								</Box>
							</Center>
							<Center>
								<Heading fontSize={"12"} textAlign={"center"}>
									{heading}
								</Heading>
							</Center>
						</CardBody>
					</Card>
				</Link>
			</ChakraProvider>
		</div>
	);
};

export default DashboardIconCard;
