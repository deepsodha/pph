import {
	Box,
	Card,
	CardBody,
	Flex,
	Link,
	Text,
	VStack,
} from "@chakra-ui/react";
import EditButton from "../../../../components/edit-button";
import DeleteButton from "../../../../components/delete-button";

type ImplantableDeviceCardProps = {
	result_1: string | null;
	result_2: string | null;
	result_3: string | null;
	editpath: string;
	open_model?: () => void;
	handleDelete: () => void;
};
const ImplantableDeviceCard = ({
	result_1,
	result_2,
	result_3,
	editpath,
	open_model,
	handleDelete,
}: ImplantableDeviceCardProps) => {
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
										<Text as="span" color="#095FBA">
											{"Status: "}
										</Text>
										<Text
											fontSize="14px"
											fontWeight={400}
											as="span"
											color="#121224"
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
										<Text as="span" color="#095FBA">
											{"UDI: "}
										</Text>
										<Text
											fontSize="14px"
											fontWeight={400}
											as="span"
											color="#121224"
										>
											{result_2}
										</Text>
									</Text>
									<Text
										marginLeft="6px"
										fontSize="14px"
										mb="2%"
										fontWeight={500}
									>
										<Text as="span" color="#095FBA">
											{"Comments: "}
										</Text>
										<Text
											fontSize="14px"
											fontWeight={400}
											as="span"
											color="#121224"
										>
											{result_3}
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

export default ImplantableDeviceCard;
