import {
	Box,
	Card,
	CardBody,
	Flex,
	Link,
	Text,
	VStack,
} from "@chakra-ui/react";
import DeleteButton from "./delete-button";
import EditButton from "./edit-button";

type DashboardCardProps = {
	heading_1: string;
	result_1: string | null;
	heading_2: string;
	result_2: string | null;
	editpath: string;
	open_model?: () => void;
	handleDelete?: () => void;
};
const DashboardCard = ({
	heading_1,
	result_1,
	heading_2,
	result_2,
	open_model,
	editpath,
	handleDelete,
}: DashboardCardProps) => {
	return (
		<div>
			<Box>
				<Card variant="outline" mb="20px">
					<Flex>
						<CardBody>
							<Link onClick={open_model}>
								<Box>
									<Text
										marginLeft="6px"
										fontSize="14px"
										mb="2%"
										fontWeight={500}
									>
										<Text as="span" color="#121224">
											{heading_1}
										</Text>{" "}
										<Text
											fontSize="14px"
											fontWeight={400}
											as="span"
											color="#717B9E"
										>
											{result_1}
										</Text>
									</Text>
									<Text
										marginLeft="6px"
										fontSize="14px"
										mb="2%"
										fontWeight={500}
									>
										<Text as="span" color="#121224">
											{heading_2}
										</Text>{" "}
										<Text
											fontSize="14px"
											fontWeight={400}
											as="span"
											color="#717B9E"
										>
											{result_2}
										</Text>
									</Text>
								</Box>
							</Link>
						</CardBody>
						<Box display={"flex"} alignItems="center">
							<VStack>
								<EditButton path={editpath} />
								<DeleteButton onclick={handleDelete} />
							</VStack>
						</Box>
					</Flex>
				</Card>
			</Box>
		</div>
	);
};

export default DashboardCard;
