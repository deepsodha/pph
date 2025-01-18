import {
	Card,
	Flex,
	CardBody,
	Box,
	VStack,
	Image,
	Text,
	Center,
	Badge,
} from "@chakra-ui/react";
import { Link as RouterLink } from "@tanstack/react-router";
import DeleteButton from "./delete-button";
import EditButton from "./edit-button";
import ProfileFemale from "../assets/Girl Icon.svg";
import ProfileMale from "../assets/Boy Icon.svg";

type PatientDetailCardProps = {
	profile_img: string;
	name: string;
	result: string;
	editpath: string;
	cardPath?: string;
	heading: string;
	gender: string;
	emergency?: boolean;
	open_model?: () => void;
	handleDelete?: () => void;
};

const DashboardCardWithImage = ({
	name,
	heading,
	result,
	open_model,
	editpath,
	handleDelete,
	cardPath,
	gender,
	emergency,
}: PatientDetailCardProps) => {
	const chosenProfileImage = gender === "Female" ? ProfileFemale : ProfileMale;

	return (
		<div>
			<Card
				border="1px"
				borderColor="#095FBA"
				borderRadius="8px"
				variant="outline"
				mb="20px"
			>
				<Flex>
					<Center>
						<Image
							src={chosenProfileImage}
							width="80px"
							height="80px"
							borderRadius="full"
							ml={5}
							mt={7}
							mb={7}
						/>
					</Center>
					<CardBody>
						<Box
							as={RouterLink}
							to={cardPath}
							height="100%"
							onClick={open_model}
							ml="5%"
							display={"flex"}
							flexDirection={"row"}
							alignItems={"center"}
						>
							<Box>
								<Text
									fontSize="18px"
									fontWeight={500}
									width={{
										sm: "100px",
										base: "100px",
										md: "150px",
										lg: "auto",
									}}
								>
									{name}
								</Text>
								<Text fontSize="12px" fontWeight={400} mt={"5px"}>
									<Text as="span">{heading}</Text> {result}
								</Text>
								{emergency && (
									<Box borderRadius={"8px"}>
										<Badge colorScheme="red">
											<Text color={"black"} fontWeight={400}>
												Emergency
											</Text>
										</Badge>
									</Box>
								)}
							</Box>
						</Box>
					</CardBody>
					<Box display="flex" justifyContent="center" alignItems={"center"}>
						<VStack>
							<EditButton path={editpath} />
							<DeleteButton onclick={handleDelete} />
						</VStack>
					</Box>
				</Flex>
			</Card>
		</div>
	);
};

export default DashboardCardWithImage;
