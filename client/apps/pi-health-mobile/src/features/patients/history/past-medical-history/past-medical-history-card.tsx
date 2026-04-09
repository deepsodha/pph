import { Card, Flex, CardBody, VStack, Box, Text } from "@chakra-ui/react";
import DeleteButton from "../../../../components/delete-button";
import EditButton from "../../../../components/edit-button";
type PastMedicalHistoryCardProps = {
	blood_type: string | null;
	head: string | null;
	respiratory: string | null;
	musculoskeletal: string | null;
	endocrine: string | null;
	eyes: string | null;
	gastrointestinal: string | null;
	skin: string | null;
	ears: string | null;
	noses: string | null;
	neurological: string | null;
	heme: string | null;
	mouth: string | null;
	infectious: string | null;
	cardiovascular: string | null;
	genitourinary: string | null;
	psychiatric: string | null;
	comments: string | null;
	editpath?: string;
	handleDelete?: () => void;
};
const PastMedicalHistoryCard = (props: PastMedicalHistoryCardProps) => {
	return (
		<Box>
			<Card variant="outline" mb="20px">
				<Flex>
					<CardBody>
						<Box>
							<Text marginLeft="6px" fontSize="14px" mb="2%" fontWeight={500}>
								<Text as="span" color="#095FBA">
									{"Blood Type: "}
								</Text>
								<Text
									fontSize="14px"
									fontWeight={400}
									as="span"
									color="#121224"
								>
									{props.blood_type}
								</Text>
							</Text>
							<Text marginLeft="6px" fontSize="14px" mb="2%" fontWeight={500}>
								<Text as="span" color="#095FBA">
									{"Head: "}
								</Text>
								<Text
									fontSize="14px"
									fontWeight={400}
									as="span"
									color="#121224"
								>
									{props.head}
								</Text>
							</Text>
							<Text marginLeft="6px" fontSize="14px" mb="2%" fontWeight={500}>
								<Text as="span" color="#095FBA">
									{"Respiratory: "}
								</Text>
								<Text
									fontSize="14px"
									fontWeight={400}
									as="span"
									color="#121224"
								>
									{props.respiratory}
								</Text>
							</Text>
							<Text marginLeft="6px" fontSize="14px" mb="2%" fontWeight={500}>
								<Text as="span" color="#095FBA">
									{"Musculoskeletal: "}
								</Text>
								<Text
									fontSize="14px"
									fontWeight={400}
									as="span"
									color="#121224"
								>
									{props.musculoskeletal}
								</Text>
							</Text>
							<Text marginLeft="6px" fontSize="14px" mb="2%" fontWeight={500}>
								<Text as="span" color="#095FBA">
									{"Endocrine: "}
								</Text>
								<Text
									fontSize="14px"
									fontWeight={400}
									as="span"
									color="#121224"
								>
									{props.endocrine}
								</Text>
							</Text>
							<Text marginLeft="6px" fontSize="14px" mb="2%" fontWeight={500}>
								<Text as="span" color="#095FBA">
									{"Eyes: "}
								</Text>
								<Text
									fontSize="14px"
									fontWeight={400}
									as="span"
									color="#121224"
								>
									{props.eyes}
								</Text>
							</Text>
							<Text marginLeft="6px" fontSize="14px" mb="2%" fontWeight={500}>
								<Text as="span" color="#095FBA">
									{"Gastrointestinal: "}
								</Text>
								<Text
									fontSize="14px"
									fontWeight={400}
									as="span"
									color="#121224"
								>
									{props.gastrointestinal}
								</Text>
							</Text>
							<Text marginLeft="6px" fontSize="14px" mb="2%" fontWeight={500}>
								<Text as="span" color="#095FBA">
									{"Skin: "}
								</Text>
								<Text
									fontSize="14px"
									fontWeight={400}
									as="span"
									color="#121224"
								>
									{props.skin}
								</Text>
							</Text>
							<Text marginLeft="6px" fontSize="14px" mb="2%" fontWeight={500}>
								<Text as="span" color="#095FBA">
									{"Ears: "}
								</Text>
								<Text
									fontSize="14px"
									fontWeight={400}
									as="span"
									color="#121224"
								>
									{props.ears}
								</Text>
							</Text>
							<Text marginLeft="6px" fontSize="14px" mb="2%" fontWeight={500}>
								<Text as="span" color="#095FBA">
									{"Noses: "}
								</Text>
								<Text
									fontSize="14px"
									fontWeight={400}
									as="span"
									color="#121224"
								>
									{props.noses}
								</Text>
							</Text>
							<Text marginLeft="6px" fontSize="14px" mb="2%" fontWeight={500}>
								<Text as="span" color="#095FBA">
									{"Neurological: "}
								</Text>
								<Text
									fontSize="14px"
									fontWeight={400}
									as="span"
									color="#121224"
								>
									{props.neurological}
								</Text>
							</Text>
							<Text marginLeft="6px" fontSize="14px" mb="2%" fontWeight={500}>
								<Text as="span" color="#095FBA">
									{"Heme: "}
								</Text>
								<Text
									fontSize="14px"
									fontWeight={400}
									as="span"
									color="#121224"
								>
									{props.heme}
								</Text>
							</Text>
							<Text marginLeft="6px" fontSize="14px" mb="2%" fontWeight={500}>
								<Text as="span" color="#095FBA">
									{"Mouth: "}
								</Text>
								<Text
									fontSize="14px"
									fontWeight={400}
									as="span"
									color="#121224"
								>
									{props.mouth}
								</Text>
							</Text>
							<Text marginLeft="6px" fontSize="14px" mb="2%" fontWeight={500}>
								<Text as="span" color="#095FBA">
									{"Infectious: "}
								</Text>
								<Text
									fontSize="14px"
									fontWeight={400}
									as="span"
									color="#121224"
								>
									{props.infectious}
								</Text>
							</Text>
							<Text marginLeft="6px" fontSize="14px" mb="2%" fontWeight={500}>
								<Text as="span" color="#095FBA">
									{"Cardiovascular: "}
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
									{"Genitourinary: "}
								</Text>
								<Text
									fontSize="14px"
									fontWeight={400}
									as="span"
									color="#121224"
								>
									{props.genitourinary}
								</Text>
							</Text>
							<Text marginLeft="6px" fontSize="14px" mb="2%" fontWeight={500}>
								<Text as="span" color="#095FBA">
									{"Psychiatric: "}
								</Text>
								<Text
									fontSize="14px"
									fontWeight={400}
									as="span"
									color="#121224"
								>
									{props.psychiatric}
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

export default PastMedicalHistoryCard;
