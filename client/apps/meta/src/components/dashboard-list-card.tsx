import { Box, Card, CardBody, Flex, Text, VStack } from "@chakra-ui/react";
import DeleteButton from "./delete-button";
import EditButton from "./edit-button";

type DashboardListCardProps = {
	heading_1: string;
	result_1: string | null;
	heading_2: string;
	result_2: string | null;
	editpath: string;
	handleDelete?: () => void;
};
const DashboardListCard = ({
	heading_1,
	result_1,
	heading_2,
	result_2,
	editpath,
	handleDelete,
}: DashboardListCardProps) => {
	return (
		<div>
			<Box>
				<Card variant="outline" mb="20px">
					<Flex>
						<CardBody>
							<Box>
								<Text marginLeft="6px" fontSize="14px" mb="2%" fontWeight={500}>
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
								<Text marginLeft="6px" fontSize="14px" mb="2%" fontWeight={500}>
									<Text as="span" color="#121224">
										{heading_2}{" "}
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

export default DashboardListCard;
