import { InfoIcon } from "@chakra-ui/icons";
import { Box, Flex, Input, Tooltip } from "@chakra-ui/react";

type PhotoUrlInputProps = {
	name: string;
};
const PhotoUrlInput = ({ name }: PhotoUrlInputProps) => {
	return (
		<div>
			<Flex>
				<Input
					type="text"
					name={name}
					width="100%"
					bgColor="#FFFFFF"
					borderColor="#095FBA"
					placeholder={"Add Photo Url"}
				/>
				<Box alignItems={"center"} justifyItems={"center"} ml={"5%"}>
					<Tooltip label="uploaded photo url is valid ex(google photos, avatar)">
						<InfoIcon height={"8"} width={"8"} />
					</Tooltip>
				</Box>
			</Flex>
		</div>
	);
};

export default PhotoUrlInput;
