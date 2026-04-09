import { Card, Flex, CardBody, VStack, Box, Text } from "@chakra-ui/react";
import DeleteButton from "../../../../components/delete-button";
import EditButton from "../../../../components/edit-button";
type FamilyCardProps = {
	family_member: string;
	health_status: string;
	general: string[] | null;
	cancer: string[] | null;
	comments: string[] | null;
	editpath: string;
	handleDelete: () => void;
};
const FamilyHistoryCard = (props: FamilyCardProps) => {
	return (
		<Box>
			<Card variant="outline" mb="20px">
				<Flex>
					<CardBody>
						<Box>
							<Text marginLeft="6px" fontSize="14px" mb="2%" fontWeight={500}>
								<Text as="span" color="#095FBA">
									{"Family Member: "}
								</Text>
								<Text
									fontSize="14px"
									fontWeight={400}
									as="span"
									color="#121224"
								>
									{props.family_member}
								</Text>
							</Text>
							<Text marginLeft="6px" fontSize="14px" mb="2%" fontWeight={500}>
								<Text as="span" color="#095FBA">
									{"Health Status: "}
								</Text>
								<Text
									fontSize="14px"
									fontWeight={400}
									as="span"
									color="#121224"
								>
									{props.health_status}
								</Text>
							</Text>
							<Text marginLeft="6px" fontSize="14px" mb="2%" fontWeight={500}>
								<Text as="span" color="#095FBA">
									{"General: "}
								</Text>
								<Text
									fontSize="14px"
									fontWeight={400}
									as="span"
									color="#121224"
								>
									{props.general}
								</Text>
							</Text>
							<Text marginLeft="6px" fontSize="14px" mb="2%" fontWeight={500}>
								<Text as="span" color="#095FBA">
									{"Cancer: "}
								</Text>
								<Text
									fontSize="14px"
									fontWeight={400}
									as="span"
									color="#121224"
								>
									{props.cancer}
								</Text>
							</Text>
							<Text marginLeft="6px" fontSize="14px" mb="2%" fontWeight={500}>
								<Text as="span" color="#095FBA">
									{"Comments: "}
								</Text>
								<Text
									fontSize="14px"
									fontWeight={400}
									as="span"
									color="#121224"
								>
									{props.comments}
								</Text>
							</Text>
						</Box>
					</CardBody>
					<Box display={"flex"} alignItems="center">
						<VStack>
							<EditButton path={props.editpath} />
							<DeleteButton onclick={props.handleDelete} />
						</VStack>
					</Box>
				</Flex>
			</Card>
		</Box>
	);
};

export default FamilyHistoryCard;
