import {
	Box,
	Accordion,
	AccordionItem,
	AccordionButton,
	AccordionIcon,
	AccordionPanel,
	Card,
	CardBody,
	Text,
} from "@chakra-ui/react";

export type AccordianData = {
	id: number | 0;
	title: string | null | undefined;
	content: string | number | null | undefined;
};
export type AccordianProps = {
	data: AccordianData[] | undefined;
	title: string | undefined;
	//   last_update: Date | undefined;
};
const AccordianComp = (props: AccordianProps) => {
	return (
		<Box>
			<Accordion
				allowToggle
				bgColor={"white"}
				borderRadius="md"
				borderX="1px solid #095FBA"
				borderY={"1px solid #095FBA"}
			>
				<AccordionItem>
					<h2>
						<AccordionButton>
							<Box as="span" flex="1" textAlign="left">
								{props.title}
								{/* <Text>
                  (Last Updated at:{" "}
                  {props?.last_update
                    ? props.last_update.toLocaleDateString(undefined, {
                        weekday: "short",
                        year: "numeric",
                        month: "short",
                        day: "numeric",
                      })
                    : "N/A"}
                  )
                </Text> */}
							</Box>
							<AccordionIcon />
						</AccordionButton>
					</h2>
					<AccordionPanel>
						<Box width="100%" display="flex">
							<Card width={"100%"}>
								<CardBody>
									<Box display={"flex"} flexDirection={"column"}>
										{props.data?.map((item) => (
											<Box key={item.id}>
												<Text fontWeight={700} fontSize={"18px"}>
													{item.title}
												</Text>
												{Array.isArray(item.content) ? (
													item.content.map((contentItem) => (
														<Box key={`${item.id}-${contentItem.title}`}>
															<Text fontWeight={400} fontSize={"20px"}>
																{contentItem.title}: {contentItem.content}
															</Text>
														</Box>
													))
												) : (
													<Text fontWeight={400} fontSize={"20px"}>
														{item.content}
													</Text>
												)}
											</Box>
										))}
									</Box>
								</CardBody>
							</Card>
						</Box>
					</AccordionPanel>
				</AccordionItem>
			</Accordion>
		</Box>
	);
};
export default AccordianComp;
