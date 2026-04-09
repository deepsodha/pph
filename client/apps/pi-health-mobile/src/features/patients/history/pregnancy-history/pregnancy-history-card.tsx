import { Card, Flex, CardBody, VStack, Box, Text } from "@chakra-ui/react";
import DeleteButton from "../../../../components/delete-button";
import EditButton from "../../../../components/edit-button";
type PregnancyHistoryCardProps = {
	age_onset_of_menses: number;
	age_at_menopause: number;
	comments_ob: string | null;
	total_pregnancy: number | null;
	full_term: number | null;
	pre_term: number | null;
	miscarriages: number | null;
	living: number | null;
	comments_pregnancy: string | null;
	editpath: string;
	handleDelete: () => void;
};
const PregnancyHistoryCard = (props: PregnancyHistoryCardProps) => {
	return (
		<Box>
			<Card variant="outline" mb="20px">
				<Flex>
					<CardBody>
						<Box>
							<Text marginLeft="6px" fontSize="14px" mb="2%" fontWeight={500}>
								<Text as="span" color="#000000">
									{"OB History: "}
								</Text>
							</Text>
							<Text marginLeft="6px" fontSize="14px" mb="2%" fontWeight={500}>
								<Text as="span" color="#095FBA">
									{"Age Onset of Menses: "}
								</Text>
								<Text
									fontSize="14px"
									fontWeight={400}
									as="span"
									color="#121224"
								>
									{props.age_onset_of_menses.toString()}
								</Text>
							</Text>
							<Text marginLeft="6px" fontSize="14px" mb="2%" fontWeight={500}>
								<Text as="span" color="#095FBA">
									{"Age at Menopause: "}
								</Text>
								<Text
									fontSize="14px"
									fontWeight={400}
									as="span"
									color="#121224"
								>
									{props.age_at_menopause.toString()}
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
									{props.comments_ob}
								</Text>
							</Text>

							<Text marginLeft="6px" fontSize="14px" mb="2%" fontWeight={500}>
								<Text as="span" color="#000000">
									{"Pregnancy History: "}
								</Text>
							</Text>
							<Text marginLeft="6px" fontSize="14px" mb="2%" fontWeight={500}>
								<Text as="span" color="#095FBA">
									{"Total Pregnancy: "}
								</Text>
								<Text
									fontSize="14px"
									fontWeight={400}
									as="span"
									color="#121224"
								>
									{props.total_pregnancy?.toString()}
								</Text>
							</Text>
							<Text marginLeft="6px" fontSize="14px" mb="2%" fontWeight={500}>
								<Text as="span" color="#095FBA">
									{"Full Term: "}
								</Text>
								<Text
									fontSize="14px"
									fontWeight={400}
									as="span"
									color="#121224"
								>
									{props.full_term?.toString()}
								</Text>
							</Text>
							<Text marginLeft="6px" fontSize="14px" mb="2%" fontWeight={500}>
								<Text as="span" color="#095FBA">
									{"Pre Term: "}
								</Text>
								<Text
									fontSize="14px"
									fontWeight={400}
									as="span"
									color="#121224"
								>
									{props.pre_term?.toString()}
								</Text>
							</Text>
							<Text marginLeft="6px" fontSize="14px" mb="2%" fontWeight={500}>
								<Text as="span" color="#095FBA">
									{"Miscarriages: "}
								</Text>
								<Text
									fontSize="14px"
									fontWeight={400}
									as="span"
									color="#121224"
								>
									{props.miscarriages?.toString()}
								</Text>
							</Text>
							<Text marginLeft="6px" fontSize="14px" mb="2%" fontWeight={500}>
								<Text as="span" color="#095FBA">
									{"Living: "}
								</Text>
								<Text
									fontSize="14px"
									fontWeight={400}
									as="span"
									color="#121224"
								>
									{props.living?.toString()}
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
									{props.comments_pregnancy}
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

export default PregnancyHistoryCard;
