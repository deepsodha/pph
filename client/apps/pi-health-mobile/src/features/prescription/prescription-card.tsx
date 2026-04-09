import { Box, Card, CardBody, Flex, Link, Text } from "@chakra-ui/react";

type PrescriptionCardProps = {
	heading: string;
	date: string;
	Desc: string;
	open_model: () => void;
};
const PrescriptionCard = ({
	heading,
	date,
	Desc,
	open_model,
}: PrescriptionCardProps) => {
	return (
		<div>
			<Card variant="outline" mb="20px" mt={4}>
				<CardBody>
					<Link onClick={open_model}>
						<Box>
							<Flex justifyItems={"flex-end"} justifyContent={"space-between"}>
								<Box mb="2%">
									<Text fontSize="lg" color="#121224" fontWeight="bold">
										{heading}
									</Text>
								</Box>
								<Box mb="2%">
									<Text fontSize="md" color="#909294">
										{date}
									</Text>
								</Box>
							</Flex>
							<Text fontSize="sm" my={1} color="#2D3748">
								<Text as="span" /> {Desc}
							</Text>
						</Box>
					</Link>
				</CardBody>
			</Card>
		</div>
	);
};

export default PrescriptionCard;
