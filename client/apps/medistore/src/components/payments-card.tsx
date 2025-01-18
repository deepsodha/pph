import {
	Box,
	Card,
	CardBody,
	Divider,
	Text,
	Image,
	Flex,
	Center,
} from "@chakra-ui/react";
import capsule from "../assets/capsule Icon.svg";
import rupee from "../assets/Rupee.svg";

type PaymentCardProps = {
	id: string;
	name: string;
	drug: string;
	date: string;
	price: number;
};
const PaymentsCard = ({ id, name, drug, date, price }: PaymentCardProps) => {
	return (
		<div>
			<Box
				mt={"5%"}
				mb={"5%"}
				display="flex"
				justifyContent={"center"}
				alignItems={"center"}
			>
				<Card
					variant={"outline"}
					border={"1px solid #1A998E"}
					width="100%"
					ml="6%"
					mr="6%"
				>
					<CardBody pt={"4%"} pl={"4%"} pb={"2%"}>
						<Box display={"flex"} flexDirection={"column"}>
							<Box display={"flex"} justifyContent={"space-between"}>
								<Box display={"flex"} flexDirection={"column"}>
									<Box bgColor="#DDF0EE" px={2} py={1} borderRadius="md">
										<Text
											fontWeight={500}
											fontSize="14px"
											color="#1A998E"
											textAlign={"center"}
										>
											ID: {id}
										</Text>
									</Box>
									<Text fontWeight={500} fontSize={18} mt={"10%"}>
										{name}
									</Text>
								</Box>
								<Box>
									<Text fontWeight={400} fontSize={"16px"} color={"#909294"}>
										{date}
									</Text>
								</Box>
							</Box>
							<Box mt={"4%"}>
								<Divider
									orientation="horizontal"
									border="1px"
									borderColor={"#D9D9D9"}
								/>
							</Box>
							<Box display={"flex"} justifyContent={"space-around"}>
								<Flex alignItems="center" mt={"2%"}>
									<Box display="flex" alignItems="center" mr={1}>
										<Image src={capsule} alt="capsule" height={19} />
										<Text ml={2} fontSize={20}>
											{drug}
										</Text>
									</Box>
								</Flex>

								<Center height="40px" mt="3%">
									<Divider
										orientation="vertical"
										border="1px"
										borderColor={"#D9D9D9"}
									/>
								</Center>

								<Flex alignItems="center" mt={"2%"}>
									<Box display="flex" alignItems="center" mt={"5%"} mr={2}>
										<Image src={rupee} alt="capsule" height={15} />
										<Text ml={2} fontSize={20}>
											{price}
										</Text>
									</Box>
								</Flex>
							</Box>
						</Box>
					</CardBody>
				</Card>
			</Box>
		</div>
	);
};

export default PaymentsCard;
