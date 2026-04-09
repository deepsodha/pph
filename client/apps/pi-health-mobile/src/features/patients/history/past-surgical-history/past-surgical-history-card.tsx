import { Card, Flex, CardBody, VStack, Box, Text } from "@chakra-ui/react";
import DeleteButton from "../../../../components/delete-button";
import EditButton from "../../../../components/edit-button";
type PastSurgicalHistoryCardProps = {
	common_surgeries: string[] | null;
	comments: string | null;

	editpath: string;
	handleDelete?: () => void;
};
const PastSurgicalHistoryCard = (props: PastSurgicalHistoryCardProps) => {
	return (
		<Box>
			<Card variant="outline" mb="20px">
				<Flex>
					<CardBody>
						<Box>
							<Text marginLeft="6px" fontSize="14px" mb="2%" fontWeight={500}>
								<Text as="span" color="#095FBA">
									{"Common Surgeries: "}
								</Text>
								<Text
									fontSize="14px"
									fontWeight={400}
									as="span"
									color="#121224"
								>
									{props.common_surgeries}
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

export default PastSurgicalHistoryCard;
