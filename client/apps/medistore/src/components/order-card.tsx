import { Box, Card, Divider, Image, Text } from "@chakra-ui/react";

import medicine from "../assets/medicine.svg";
import rupee from "../assets/rupee.svg";
type OrderCardProps = {
	id: string;
	date: string;
	name: string;
	tablet: string;
	quantity: number;
	price: number;
};
const OrderCard = ({
	id,
	date,
	name,
	tablet,
	quantity,
	price,
}: OrderCardProps) => {
	return (
		<Card
			marginLeft="6%"
			mr="6%"
			border="1px"
			borderRadius="8px"
			borderColor={"#1A998E"}
			padding="1%"
			mb="6%"
		>
			<Box
				display={"flex"}
				justifyContent={"space-between"}
				mb="1%"
				padding="2%"
			>
				<Box
					backgroundColor={"#DDF0EE"}
					borderRadius={"4px"}
					pl="3%"
					pr="3%"
					pt="0%"
					pb="0%"
				>
					<Text color="#1A998E" fontSize={"14px"}>
						ID: {id}
					</Text>
				</Box>
				<Box>
					<Text color="#909294" fontSize={"16px"}>
						{date}
					</Text>
				</Box>
			</Box>
			<Divider orientation="horizontal" border="1px" borderColor="#D9D9D9" />
			<Box display={"flex"} justifyContent={"space-between"} padding="2%">
				<Box mt="1%">
					<Box display="flex" flexDirection={"column"}>
						<Text fontSize="18px" fontWeight={500}>
							{name}
						</Text>
						<Box mt="6%" display="flex">
							<Box
								backgroundColor={"#DDF0EE"}
								borderRadius={"4px"}
								width="20%"
								mr="4%"
							>
								<Image height="100%" width="100%" src={medicine} />
							</Box>

							<Text fontSize={"19px"} fontWeight={500} width="60%">
								{tablet}
							</Text>
						</Box>
					</Box>
				</Box>
				<Box>
					<Box
						display={"flex"}
						flexDirection={"column"}
						mt="10%"
						alignItems={"center"}
					>
						<Text fontSize={"16px"} fontWeight={400}>
							Qty: {quantity}
						</Text>
						<Box display={"flex"} mt="60%" mr="50%">
							<Image src={rupee} mr="15%" mt="10%" />
							<Text color="#1A998E" fontSize={"18px"} fontWeight={500}>
								{price}
							</Text>
						</Box>
					</Box>
				</Box>
			</Box>
		</Card>
	);
};

export default OrderCard;
