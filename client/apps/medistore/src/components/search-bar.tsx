import {
	Box,
	InputGroup,
	Input,
	InputRightElement,
	InputLeftElement,
	Image,
} from "@chakra-ui/react";

import search from "../assets/search.svg";
type SearchBarProps = {
	img: string;
	placeholder: string;
};

export const SearchBar = ({ img, placeholder }: SearchBarProps) => {
	return (
		<div>
			<Box marginLeft="6%" mr="6%" mb="6%">
				<InputGroup borderRadius="8px" borderColor="#1A998E">
					<InputLeftElement>
						<Image src={search} height="50%" mt="15%" ml="10%" />
					</InputLeftElement>

					<Input placeholder={placeholder} size="lg" />

					<InputRightElement>
						<Box
							backgroundColor={"#1A998E"}
							borderTopRightRadius="5px"
							borderBottomRightRadius={" 5px"}
							height="115%"
							mt="20%"
							mr="2%"
							display="flex"
							alignItems="center"
							justifyContent="center"
							width="100%"
						>
							<Image src={img} />
						</Box>
					</InputRightElement>
				</InputGroup>
			</Box>
		</div>
	);
};
