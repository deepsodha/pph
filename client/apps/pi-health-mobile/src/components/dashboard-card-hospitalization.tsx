import { Box, Card, CardBody, Flex, Text, VStack } from "@chakra-ui/react";
import DeleteButton from "./delete-button";
import EditButton from "./edit-button";

type DashboardCardHospitalizationProps = {
	result_1: string;
	result_2: string | null;
	result_3: string | null;
	result_4: number | null;
	result_5: string | null;
	result_6: string | null;
	editpath: string;
	handleDelete?: () => void;
};

const HospitalizationCard = ({
	result_1,
	result_2,
	result_3,
	result_4,
	result_5,
	result_6,
	editpath,
	handleDelete,
}: DashboardCardHospitalizationProps) => {
	return (
		<div>
			<Box>
				<Card variant="outline" mb="20px">
					<Flex>
						<CardBody>
							<Box>
								<Text marginLeft="6px" fontSize="14px" mb="2%" fontWeight={500}>
									<Text as="span" color="#121224">
										Admission Date:
									</Text>
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
										Related To:
									</Text>
									<Text
										fontSize="14px"
										fontWeight={400}
										as="span"
										color="#717B9E"
									>
										{result_2}
									</Text>
								</Text>
								<Text marginLeft="6px" fontSize="14px" mb="2%" fontWeight={500}>
									<Text as="span" color="#121224">
										Status:
									</Text>
									<Text
										fontSize="14px"
										fontWeight={400}
										as="span"
										color="#717B9E"
									>
										{result_3}
									</Text>
								</Text>
								<Text marginLeft="6px" fontSize="14px" mb="2%" fontWeight={500}>
									<Text as="span" color="#121224">
										Length Of Stay:
									</Text>
									<Text
										fontSize="14px"
										fontWeight={400}
										as="span"
										color="#717B9E"
									>
										{result_4}
									</Text>
								</Text>
								<Text marginLeft="6px" fontSize="14px" mb="2%" fontWeight={500}>
									<Text as="span" color="#121224">
										Procedure:
									</Text>
									<Text
										fontSize="14px"
										fontWeight={400}
										as="span"
										color="#717B9E"
									>
										{result_5}
									</Text>
								</Text>
								<Text marginLeft="6px" fontSize="14px" mb="2%" fontWeight={500}>
									<Text as="span" color="#121224">
										Comments:
									</Text>
									<Text
										fontSize="14px"
										fontWeight={400}
										as="span"
										color="#717B9E"
									>
										{result_6}
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
export default HospitalizationCard;
