"use client";

import { Box, Image, Text } from "@chakra-ui/react";
import {
	Provider,
	Carousel,
	LeftButton,
	RightButton,
} from "chakra-ui-carousel";
import CarouselCard from "./carousel-card";
const Slider: React.FC = () => {
	return (
		<>
			<Provider>
				<Box mt="1%" display="flex" flexDirection="column" ml="8%">
					<Box boxSize="50px" mt="2%">
						<Image src="Quote Image.svg" alt="quote" />
					</Box>

					<Text fontSize="40px" fontWeight={600} color={"black"}>
						What our <br />
						Patient’s say
					</Text>
					<Box display="flex" flexDirection="row">
						<LeftButton
							mr={"7%"}
							bgColor={"#888888"}
							color="white"
							borderRadius={"100%"}
							boxSize={"28px"}
							_hover={{
								bgColor: "white",
								color: "#888888",
								border: "5px solid white",
							}}
						/>

						<RightButton
							mr={"7%"}
							bgColor={"#888888"}
							color="white"
							borderRadius={"100%"}
							boxSize={"28px"}
							_hover={{
								bgColor: "white",
								color: "#888888",
								border: "1px solid #095FBA",
							}}
						/>
					</Box>
				</Box>

				<Carousel gap={-180}>
					<CarouselCard
						heading={"Lorem Ipsum"}
						content={
							"Lorem Ipsum has been the industry's standard dummy text ever since the 1500s, when an unknown printer took a galley of type and scrambled it to make a type specimen book."
						}
						margin={"10%"}
					/>
					,
					<CarouselCard
						heading={"Lorem Ipsum"}
						content={
							"Lorem Ipsum has been the industry's standard dummy text ever since the 1500s, when an unknown printer took a galley of type and scrambled it to make a type specimen book."
						}
						margin={"30%"}
					/>
					,
					<CarouselCard
						heading={"Lorem Ipsum"}
						content={
							"Lorem Ipsum has been the industry's standard dummy text ever since the 1500s, when an unknown printer took a galley of type and scrambled it to make a type specimen book."
						}
						margin={"52%"}
					/>
					,
					<CarouselCard
						heading={"Lorem Ipsum"}
						content={
							"Lorem Ipsum has been the industry's standard dummy text ever since the 1500s, when an unknown printer took a galley of type and scrambled it to make a type specimen book."
						}
						margin={"75%"}
					/>
					,
					<CarouselCard
						heading={"Lorem Ipsum"}
						content={
							"Lorem Ipsum has been the industry's standard dummy text ever since the 1500s, when an unknown printer took a galley of type and scrambled it to make a type specimen book."
						}
						margin={"95%"}
					/>
					,
				</Carousel>
			</Provider>
		</>
	);
};

export default Slider;
