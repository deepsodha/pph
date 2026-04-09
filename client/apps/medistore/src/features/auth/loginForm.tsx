import {
	Box,
	Button,
	FormControl,
	Text,
	Input,
	InputGroup,
	NumberInput,
	NumberInputField,
	Center,
	Checkbox,
} from "@chakra-ui/react";

import { useForm } from "@felte/react";
export const LoginForm = () => {
	const { form } = useForm();
	return (
		<Box>
			<form ref={form}>
				<Box mt={35}>
					<Text fontSize="64px" marginLeft={"5%"} textColor="#FFFFFF">
						Login
					</Text>
					<Text
						marginLeft={"5%"}
						fontSize={"35px"}
						textColor="#FFFFFF"
						marginTop={4}
					>
						Please provide your Phone Number
						<br />
						to continue
					</Text>

					<Box marginLeft={"5%"} marginRight={"5%"} marginTop="25px">
						<FormControl>
							<Box>
								<InputGroup borderRadius={5}>
									<Box
										border={"1px solid #FFFFFF"}
										width="110px"
										borderRadius={5}
										display={"flex"}
										alignItems={"center"}
										justifyContent={"center"}
										color={"#FFFFFF"}
									>
										+91
									</Box>
									<NumberInput
										width="100%"
										ml={"17px"}
										borderRadius={5}
										border={"1px solid #FFFFFF"}
										borderColor={"white"}
										height={10}
									>
										<NumberInputField
											_placeholder={{ color: "#FFFFFF" }}
											border={"1px solid #FFFFFF"}
											color={"#FFFFFF"}
											borderColor={"white"}
											borderRadius={5}
											placeholder="Phone Number"
										/>
									</NumberInput>
								</InputGroup>
							</Box>
						</FormControl>
					</Box>
					<Center>
						<Box marginLeft={"5%"} marginRight={"5%"} minWidth={"90%"}>
							<FormControl mt={6}>
								<Input
									_placeholder={{ color: "#FFFFFF" }}
									type="password"
									placeholder="Password"
									name="password"
									border={"1px solid #FFFFFF"}
									color={"#FFFFFF"}
									width="100%"
								/>
							</FormControl>
							<Box>
								<Checkbox mt={6}>
									<Text color={"white"}>I agree to the Terms & Conditions</Text>
								</Checkbox>
							</Box>
							<Box>
								<Button
									marginTop={"30px"}
									marginBottom={34}
									border={"1px solid #FFFFFF"}
									color={"#FFFFFF"}
									type="submit"
									width="100%"
								>
									<Text color={"#095FBA"}>Login</Text>
								</Button>
							</Box>
						</Box>
					</Center>
				</Box>
			</form>
		</Box>
	);
};
