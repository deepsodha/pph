import {
	ChakraProvider,
	Input,
	Button,
	Link,
	Box,
	Text,
	Image,
} from "@chakra-ui/react";

interface Props {
	backgroundImage: string;
	logoImage: string;
	heading: string;
	continueButtonColour: string;
	linkTextColour: string;
}

export const SignIn: React.FC<Props> = ({
	backgroundImage,
	logoImage,
	heading,
	continueButtonColour,
	linkTextColour,
}) => {
	return (
		<ChakraProvider>
			{/* <Header/> */}
			<form
				style={{
					backgroundImage: `url("${backgroundImage}")`,
					backgroundSize: "cover",
					backgroundRepeat: "no-repeat",
					backgroundPosition: "center",
					minHeight: "100vh",
					paddingBottom: "8%",
				}}
			>
				<Link href="/">
					<Image src={logoImage} paddingTop="4%" marginLeft="10%" />
				</Link>

				<Box
					width={{ lg: "38%", md: "80%", base: "80%" }}
					marginLeft="auto"
					marginRight="auto"
					display="flex"
					flexDirection={{ md: "column", base: "column" }}
				>
					<Text
						height="8%"
						marginTop="12%"
						marginLeft="0"
						marginRight="0"
						fontWeight={"bold"}
						fontSize={{ md: "40px", base: "30px" }}
						textAlign="left"
					>
						{heading}
					</Text>
					<Box mt="10%" height="10%">
						<Text mb={"2%"} fontSize={{ md: "16px", base: "20px" }}>
							Email/Phone Number
						</Text>
						<Input
							type="email"
							backgroundColor={"white"}
							height="50px"
							placeholder={"Enter your Phone Number or Email"}
							_placeholder={{ color: "gray.500" }}
						/>
					</Box>
					<Box mt="6%" height="10%">
						<Text mb={"2%"} fontSize={{ md: "16px", base: "20px" }}>
							Password
						</Text>
						<Input
							type="password"
							backgroundColor={"white"}
							height="50px"
							placeholder={"Enter your Password"}
							_placeholder={{ color: "gray.500" }}
						/>
					</Box>
					<Box>
						<Button
							size="lg"
							backgroundColor={continueButtonColour}
							color="white"
							width="100%"
							fontWeight={600}
							fontSize={{ base: "15px", md: "16px" }}
							_hover={{ backgroundColor: "#095FBA" }}
							mt="9%"
							mb="8%"
						>
							Continue
						</Button>
					</Box>
					<Box mt="2%" display="flex" alignItems="center" width="100%">
						<Box flex={1} height="1px" bg="#CCCCCC" mx="2" />
						<Box>or</Box>
						<Box flex={1} height="1px" bg="#CCCCCC" mx="2" />
					</Box>
					<Button
						mt="8%"
						backgroundColor="white"
						border="solid"
						fontSize={{ md: "small", base: "small" }}
						borderWidth="1px"
						borderColor="#095FBA"
						display="flex"
						justifyContent={"flex-start"}
						_hover={{ backgroundColor: "white" }}
						width="100%"
					>
						<Image src="Vector.svg" boxSize={6} />
						<Text
							ml={{ md: "35%", base: "18%", sm: "35%" }}
							fontSize={{ md: "16px", base: "10px" }}
							color={"black"}
						>
							{" "}
							Continue With Google
						</Text>
					</Button>

					<Box display="flex" flexWrap={"wrap"} justifyContent="center" mt="4%">
						<Text size="30%">New to FindmyDoc? </Text>
						<Link href="/sign-up" color={linkTextColour} ml={"1%"}>
							{" "}
							Create an account
						</Link>
					</Box>
				</Box>
			</form>
		</ChakraProvider>
	);
};
