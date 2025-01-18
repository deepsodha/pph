import { Box, Card, CardBody, Text, Image, Flex } from "@chakra-ui/react";
import DeleteIcon from "../assets/Delete Icon.svg";
import EditIcon from "../assets/Edit Icon.svg";
type drugsCardProps = {
	name: string;
	qty_left: number;
	amt: number;
};
const DrugsCard = (props: drugsCardProps) => {
	return (
		<div>
			<Box mt={"5%"} mb={"5%"}>
				<Card
					//   size={"md"}
					variant={"outline"}
					border={"1px solid #1A998E"}
					//   width="85%"
					//   ml={"7%"}
				>
					<CardBody>
						<Box
							display={"flex"}
							justifyContent={"space-between"}
							alignItems={"center"}
						>
							<Box display={"flex"} flexDirection={"column"}>
								<Text fontWeight={500} fontSize={"24px"}>
									{props.name}
								</Text>

								<Text
									fontWeight={700}
									fontSize={"18px"}
									color={"#1A998E"}
									mt={"10%"}
								>
									Rs. {props.amt}
								</Text>
							</Box>

							<Box display={"flex"} flexDirection={"column"}>
								<Flex alignItems="center">
									<Text fontWeight={500} fontSize="16px" color="black">
										QTY LEFT:
									</Text>
									<Box bgColor="#DDF0EE" px={2} py={1} borderRadius="md" ml={2}>
										<Text fontWeight={500} fontSize="16px" color="#1A998E">
											{props.qty_left}
										</Text>
									</Box>
								</Flex>

								<Box
									display={"flex"}
									justifyContent={"space-evenly"}
									mt={"10%"}
								>
									<Box>
										<Image
											src={EditIcon}
											alt="edit Icon"
											width="30px"
											height="30px"
										/>
									</Box>
									<Box>
										<Image
											src={DeleteIcon}
											alt="delete Icon"
											width="30px"
											height="30px"
										/>
									</Box>
								</Box>
							</Box>
						</Box>
					</CardBody>
				</Card>
			</Box>
		</div>
	);
};

export default DrugsCard;
