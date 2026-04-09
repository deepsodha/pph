import {
	Box,
	Button,
	Card,
	CardBody,
	Center,
	Drawer,
	DrawerBody,
	DrawerCloseButton,
	DrawerContent,
	DrawerHeader,
	DrawerOverlay,
	Flex,
	Heading,
	IconButton,
	Image,
	Spacer,
	useToast,
} from "@chakra-ui/react";
import { useAtom } from "jotai";
import {
	addPath,
	addValue,
	dashboardValue,
	displayMenu,
	formValue,
	headerText,
} from "../atoms/header";
import Backarrow from "../assets/Back Arrow Icon.svg";
import { Link, useNavigate, useRouter } from "@tanstack/react-router";
import image from "../assets/Docter List Image.svg";
import { useState } from "react";
import account from "../assets/Account icon.svg";
import { CloseIcon, HamburgerIcon } from "@chakra-ui/icons";
import dashboardimg from "../assets/Dashboard icon.svg";
import logout from "../assets/Logout icon.svg";

const Header = () => {
	const [heading] = useAtom(headerText);
	const route = useRouter();
	const [path] = useAtom(addPath);
	const [add] = useAtom(addValue);
	const [form] = useAtom(formValue);
	const [menu] = useAtom(displayMenu);
	const [dashboard] = useAtom(dashboardValue);
	const [isOpen, setIsOpen] = useState(false);

	const toggleMenu = () => {
		setIsOpen(!isOpen);
	};
	const closeMenu = () => {
		setIsOpen(false);
	};
	const navigate = useNavigate();
	const toast = useToast();
	const handleLogout = () => {
		try {
			localStorage.removeItem("token");
			navigate({ to: "/" });
		} catch (error) {
			toast({
				title: "Error",
				description: error as string,
				status: "error",
				duration: 9000,
				isClosable: true,
			});
		}
		console.log();
	};
	const renderMenuItems = () => {
		return (
			<>
				<Link to={"/dashboard"} onClick={closeMenu}>
					<Box height="70px" display="flex">
						<Image src={dashboardimg} boxSize={5} alt="dashboard" mr="6%" />
						Dashboard
					</Box>
				</Link>
				<Link to={"/profile"} onClick={closeMenu}>
					<Box height="70px" display="flex">
						<Image src={account} boxSize={5} mr="6%" alt="account" />
						Account
					</Box>
				</Link>
			</>
		);
	};

	return (
		<>
			{form && (
				<Flex align="center" justify="space-between">
					<Box
						pt="4"
						pb="4"
						ml="4"
						display="flex"
						alignItems="center"
						justifyContent="center"
					>
						<Center>
							<Link onClick={() => route.history.back()}>
								<Image height={35} width={38} src={Backarrow} />
							</Link>
							<Heading as="h3" size="lg" color="#095FBA" ml="2">
								{heading}
							</Heading>
						</Center>
					</Box>
				</Flex>
			)}
			{!add && (
				<Box>
					<Box
						bg="#E9F5FC"
						height={190}
						alignItems="center"
						flexDirection={"column"}
						justifyContent={"center"}
					>
						<Center>
							<Box width={{ md: "80%", base: "90%", lg: "70%" }}>
								<Link onClick={() => route.history.back()}>
									<Image mt={4} height={35} width={38} src={Backarrow} />
								</Link>
								<Center>
									<Link to={path}>
										<Center>
											<Card
												mt={-4}
												borderRadius="10px"
												border={"1px"}
												height="118px"
												borderColor={"#095FBA"}
												bgColor={"#E9F5FC"}
											>
												<CardBody>
													<Center>
														<Image src={image} />
													</Center>
													<Center>
														<Heading m={2} fontSize={"sm"}>
															{heading}
														</Heading>
													</Center>
												</CardBody>
											</Card>
										</Center>
									</Link>
								</Center>
							</Box>
						</Center>
					</Box>
				</Box>
			)}
			{dashboard && (
				<Box>
					<Box bg="#FFFFFF">
						<Center>
							<Box width={{ md: "80%", base: "90%", lg: "70%" }}>
								<Box
									mt="12px"
									display={"flex"}
									alignItems={"center"}
									alignContent={"center"}
									justifyItems={"center"}
									justifyContent={"center"}
								>
									<Box>
										<IconButton
											aria-label="Options"
											icon={isOpen ? <CloseIcon /> : <HamburgerIcon />}
											onClick={toggleMenu}
											variant="outline"
										/>
									</Box>
									<Box flex="1">
										<Center>
											<Heading as="h3" size="lg" color="#095FBA" ml="2">
												Dashboard
											</Heading>{" "}
										</Center>
									</Box>
									<Box>
										<Button bgColor="#FFFFFF" onClick={() => handleLogout()}>
											<Image height="32px" width="25px" src={logout} />
										</Button>
									</Box>
									<Drawer
										placement="left"
										onClose={closeMenu}
										size={{ md: "md", lg: "md", base: "full" }}
										isOpen={isOpen}
									>
										<DrawerOverlay />
										<DrawerContent>
											<DrawerHeader>
												{/* <Heading>logo</Heading>	*/}
											</DrawerHeader>
											<DrawerCloseButton />
											<DrawerBody>{renderMenuItems()}</DrawerBody>
										</DrawerContent>
									</Drawer>
								</Box>
							</Box>
						</Center>
					</Box>
				</Box>
			)}
			{menu && (
				<Box
					bg="#E9F5FC"
					height={190}
					alignItems="center"
					flexDirection={"column"}
					justifyContent={"center"}
				>
					<Center>
						<Box width={{ md: "80%", base: "90%", lg: "70%" }}>
							<Flex mt="12px" mb="12px">
								<Box display="flex" alignItems="center">
									<IconButton
										aria-label="Options"
										icon={isOpen ? <CloseIcon /> : <HamburgerIcon />}
										onClick={toggleMenu}
										variant="outline"
									/>
								</Box>
								<Spacer />
								<Drawer
									placement="left"
									onClose={closeMenu}
									size={{ md: "md", lg: "md", base: "full" }}
									isOpen={isOpen}
								>
									<DrawerOverlay />
									<DrawerContent>
										<DrawerHeader>
											<Heading>Dashboard</Heading>
										</DrawerHeader>
										<DrawerCloseButton />
										<DrawerBody>{renderMenuItems()}</DrawerBody>
									</DrawerContent>
								</Drawer>
							</Flex>
							<Center>
								<Box
									margin="auto"
									display={"flex"}
									alignItems={"center"}
									justifyContent={"center"}
									flexDirection={"column"}
								>
									<Link to={path}>
										<Card
											borderRadius="10px"
											border={"1px"}
											borderColor={"#095FBA"}
											marginTop="-20px"
											bgColor={"#E9F5FC"}
										>
											<CardBody>
												<Center>
													<Image src={image} height={10} width={10} />
												</Center>
												<Center>
													<Heading mt={2} fontSize={"sm"}>
														{heading}
													</Heading>
												</Center>
											</CardBody>
										</Card>
									</Link>
								</Box>
							</Center>
						</Box>
					</Center>
				</Box>
			)}
		</>
	);
};
export default Header;
