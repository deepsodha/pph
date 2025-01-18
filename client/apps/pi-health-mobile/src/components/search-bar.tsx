import { Box, Center, Image, Input, InputGroup } from "@chakra-ui/react";
import Search from "../assets/White Search Icon.svg";

type SearchBarProps = {
	value?: string;
	onchange?: (e: React.ChangeEvent<HTMLInputElement>) => void;
	placeholder: string;
};
const SearchBar = ({ value, onchange, placeholder }: SearchBarProps) => {
	return (
		<div>
			<Box mt={4}>
				<Center>
					<InputGroup
						border="1px"
						width="100%"
						height={{
							md: "50px",
							base: "50px",
							lg: "50px",
							xl: "50px",
						}}
						rounded="md"
						borderColor="#095FBA"
						bgColor="#095FBA"
						display={"flex"}
						alignItems={"center"}
						justifyItems={"center"}
						justifyContent={"center"}
						alignContent={"center"}
					>
						<Input
							type="text"
							_placeholder={{ opacity: 1, color: "#717B9E" }}
							height="95%"
							bgColor="white"
							focusBorderColor="#095FBA"
							color="#717B9E"
							placeholder={placeholder}
							value={value}
							onChange={onchange}
						/>
						<Box
							bg="#095FBA"
							rounded="md"
							width={16}
							height={8}
							cursor={"pointer"}
						>
							<Center>
								<Image src={Search} mt={1} color="#2D3748" />
							</Center>
						</Box>
					</InputGroup>
				</Center>
			</Box>
		</div>
	);
};

export default SearchBar;
