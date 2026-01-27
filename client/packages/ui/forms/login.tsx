import {
	Box,
	Button,
	FormControl,
	FormErrorMessage,
	Heading,
	Input,
	Image,
	// NumberInput,
	// NumberInputField,
	InputGroup,
	// Select,
} from "@chakra-ui/react";
import { Text } from "@chakra-ui/react";
import type { z } from "zod";
import { useForm } from "@felte/react";
import { validator } from "@felte/validator-zod";
import reporterDom from "@felte/reporter-dom";
import type { LoginMobile, PhoneCode } from "@repo/types/dto";
import { loginMobileSchema } from "@repo/types/validation";

export type LoginProps = {
	onSubmit: (l: LoginMobile) => void;
	image_url: string;
	phonecode?: PhoneCode[] | undefined;
};
export const Login = ({ onSubmit, image_url }: LoginProps) => {
	const {
		form,
		errors,
		isSubmitting,
		setData,
		setIsDirty,
		setTouched,
		isDirty,
		isValid,
	} = useForm<z.infer<typeof loginMobileSchema>>({
		onSubmit: (values) => {
			onSubmit(values);
			console.log(values);
		},
		extend: [validator({ schema: loginMobileSchema }), reporterDom()],
	});
	//

	return (
		<Box
			display={"flex"}
			flexWrap={"wrap"}
			justifyContent={"center"}
			mt={{ base: "5%", md: "10%" }}
		>
			<Box
			// position="relative"
			// display="flex"
			// justifyContent="start"
			// flexDirection={"column"}
			>
				<Image
					width={{ lg: "600px", base: "auto" }}
					height={{ lg: "400px", base: "auto" }}
					objectFit={"contain"}
					src={image_url}
					alt=""
				/>
			</Box>

			<Box
				boxShadow="dark-lg"
				rounded="2xl"
				mt={{ base: "10px", md: "", lg: "" }}
				mb={{ base: "5%", md: "3%" }}
				ml={{ base: 0, md: "1%" }}
				width={{ lg: "400px", base: "95%" }}
				height={{ lg: "auto", base: "auto" }}
			>
				<form ref={form}>
					<Box gap={6} mt={{ base: "10%", md: "5%", lg: "" }}>
						<Heading
							as="h1"
							size="2xl"
							marginLeft={{ base: "10%", md: "10%", lg: "6%" }}
						>
							Login
						</Heading>
						<Text
							marginLeft={{ base: "10%", md: "10%", lg: "6%" }}
							fontSize={{ base: "18px", md: "21px" }}
							textColor="#575858"
							marginTop={4}
						>
							Please provide your Phone Number
							<br />
							to continue
						</Text>
						<Box
							display="flex"
							marginLeft={{ base: "10%", lg: "24px" }}
							marginRight={{ base: "10%", lg: "24px" }}
							marginTop="25px"
						>
							<FormControl
								isInvalid={(errors().mobile_number || []).length !== 0}
							>
								<InputGroup borderRadius={5}>
									{/* <Select placeholder="+91" width="110px" borderColor="#095FBA">
										{phonecode?.map((items) => (
											<option key={indexId++} value={items.phone_code}>
												{items.phone_code}
											</option>
										))}
									</Select> */}
									{/* <NumberInput
										onChange={(valueAsString) => {
											setData("mobile_number", valueAsString);
											setIsDirty(true);
											setTouched("mobile_number", true);
										}}
										width={"100%"}
										onBlur={() => setTouched("mobile_number", true)}
										ml={3}
										focusBorderColor="#095FBA"
										border="1px"
										borderRadius={5}
										borderColor={"#095FBA"}
										height={10}
									>
										<NumberInputField
											placeholder="Phone Number"
											height={10}
											border="none"
											maxLength={10}
										/>
									</NumberInput> */}
									<Input
										onChange={(e) => {
											setData("mobile_number", e.target.value);
											setTouched("mobile_number", true);
											setIsDirty(true);
										}}
										placeholder="+91  |  Phone Number"
										width="100%"
										onBlur={() => setTouched("mobile_number", true)}
										focusBorderColor="#095FBA"
										borderColor={"#095FBA"}
										height={10}
										maxLength={10}
									/>
								</InputGroup>
								<FormErrorMessage ml={{ base: 6, md: 0 }} mr={5}>
									{errors().mobile_number}
								</FormErrorMessage>
							</FormControl>
						</Box>
						<Box
							marginLeft={{ base: "10%", lg: "24px" }}
							marginRight={{ base: "10%", lg: "24px" }}
						>
							<Box>
								<FormControl
									mt={{ base: "6%", md: 6 }}
									isInvalid={(errors().password || []).length !== 0}
								>
									<Input
										type="password"
										placeholder="Password"
										name="password"
										borderColor="#095FBA"
										width="100%"
									/>
									<FormErrorMessage mr={5}>
										{errors().password}
									</FormErrorMessage>
								</FormControl>
							</Box>
							<Box>
								<Button
									marginTop={{ base: "10%", md: "38px", lg: "30px" }}
									marginBottom={34}
									bgColor="#095FBA"
									color="white"
									type="submit"
									width="100%"
									isLoading={isSubmitting()}
									loadingText="Submitting"
									isDisabled={!(isDirty() && isValid())}
								>
									Login
								</Button>
							</Box>
						</Box>
					</Box>
				</form>
			</Box>
		</Box>
	);
};
