import { Divider, Heading } from "@chakra-ui/react";

type HeadingTagProps = {
	label_text: string;
};

export const HeadingTag = ({ label_text }: HeadingTagProps) => {
	return (
		<div>
			<Heading
				as="h2"
				size="5%"
				noOfLines={1}
				color="#474D6A"
				fontWeight="normal"
			>
				{label_text}
			</Heading>
			<Divider
				pt={2}
				orientation="horizontal"
				style={{ color: "#474D6A" }}
				borderColor="#474D6A"
			/>
		</div>
	);
};
