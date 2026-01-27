import { Card, Flex, CardBody, VStack, Box, Text } from "@chakra-ui/react";
import DeleteButton from "../../../../components/delete-button";
import EditButton from "../../../../components/edit-button";
import type { GenderType } from "@repo/types/dto";
type SocialCardProps = {
	editpath: string;
	birth_gender: GenderType;
	tobacco: string[] | null;
	alcohol: string[] | null;
	cardiovascular: string[] | null;
	sexual_activity: string[] | null;
	drug_abuse: string[] | null;
	safety: string[] | null;
	comments: string | null;
	handleDelete: () => void;
};
const SocialHistoryCard = (props: SocialCardProps) => {
	return (
		<Box>
			<Card variant="outline" mb="20px">
				<Flex>
					<CardBody>
						<Box>
							<Text marginLeft="6px" fontSize="14px" mb="2%" fontWeight={500}>
								<Text as="span" color="#095FBA">
									{"Social Member: "}
								</Text>
								<Text
									fontSize="14px"
									fontWeight={400}
									as="span"
									color="#121224"
								>
									{props.birth_gender}
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
									{props.tobacco}
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
									{props.alcohol}
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
									{props.drug_abuse}
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
									{props.cardiovascular}
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
									{props.cardiovascular}
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
									{props.safety}
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
									{props.sexual_activity}
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

export default SocialHistoryCard;
