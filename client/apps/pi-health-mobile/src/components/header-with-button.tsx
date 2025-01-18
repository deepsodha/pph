import {
	Card,
	CardBody,
	Center,
	Heading,
	Image,
	Flex,
	Spacer,
	IconButton,
	Box,
	Drawer,
	DrawerBody,
	DrawerContent,
	DrawerHeader,
	DrawerOverlay,
	DrawerCloseButton,
} from "@chakra-ui/react";

import Logo from "../assets/Pi-Health Logo.svg";
import { useAtom } from "jotai";
import { Link, useRouter } from "@tanstack/react-router";
import user from "../assets/User_icon(Grey).svg";
import dashboardimg from "../assets/Dashboard icon.svg";
import doctor from "../assets/Doctor icon (2).svg";
import patient from "../assets/Patient icon (2).svg";
import settings from "../assets/Setting icon.svg";
import staff from "../assets/Staff icon.svg";
import Backarrow from "../assets/Back Arrow Icon.svg";
import appointment from "../assets/Appointment icon (3).svg";
import prescription from "../assets/Prescription icon (2).svg";
import {
	addPath,
	addImage,
	headerText,
	addValue,
	dashboardValue,
	formValue,
	displayMenu,
} from "../atoms/header";
import { CloseIcon, HamburgerIcon } from "@chakra-ui/icons";
import { useState } from "react";
import {
	issuperadmin,
	issystemadmin,
	isdoctor,
	isofficestaff,
} from "../atoms/roles";
import LogoutButton from "./logout-button";

const HeaderWithButton = () => {
	const [superadmin] = useAtom(issuperadmin);
	const [systemadmin] = useAtom(issystemadmin);
	const [doctorRole] = useAtom(isdoctor);
	const [officestaff] = useAtom(isofficestaff);

	const [heading] = useAtom(headerText);
	const [path] = useAtom(addPath);
	const [image] = useAtom(addImage);
	const [add] = useAtom(addValue);
	const [form] = useAtom(formValue);
	const [menu] = useAtom(displayMenu);
	const [dashboard] = useAtom(dashboardValue);
	const route = useRouter();

	const renderMenuItems = () => {
		if (superadmin && !doctorRole && !systemadmin && !officestaff) {
			return (
				<>
					<Link to={"/dashboard"} onClick={closeMenu}>
						<Box height="70px" display="flex">
							<Image src={dashboardimg} boxSize={5} alt="dashboard" mr="6%" />
							Dashboard
						</Box>
					</Link>
				</>
			);
		}
		if (systemadmin && !officestaff && !doctorRole && !superadmin) {
			return (
				<>
					<Link to={"/dashboard"} onClick={closeMenu}>
						<Box height="70px" display="flex">
							<Image src={dashboardimg} boxSize={5} alt="dashboard" mr="6%" />
							Dashboard
						</Box>
					</Link>
					<Link to={"/doctor/list"} onClick={closeMenu}>
						<Box height="70px" display="flex">
							<Image src={doctor} boxSize={5} mr="6%" alt="doctor" />
							Doctor
						</Box>
					</Link>
					<Link to={"/staff/list"} onClick={closeMenu}>
						<Box height="70px" display="flex">
							<Image src={staff} boxSize={5} mr="6%" alt="staff" />
							Staff
						</Box>
					</Link>
					<Link to={"/settings/list"} onClick={closeMenu}>
						<Box height="70px" display="flex">
							<Image src={settings} boxSize={5} mr="6%" alt="settings" />
							Settings
						</Box>
					</Link>
					<Link to={"/user/list"} onClick={closeMenu}>
						<Box height="70px" display="flex">
							<Image src={user} boxSize={5} mr="6%" alt="users" />
							Users
						</Box>
					</Link>
				</>
			);
		}
		if (doctorRole && !officestaff && !superadmin && !systemadmin) {
			return (
				<>
					<Link to={"/dashboard"} onClick={closeMenu}>
						<Box height="70px" display="flex">
							<Image src={dashboardimg} boxSize={5} alt="dashboard" mr="6%" />
							Dashboard
						</Box>
					</Link>
					<Link to={"/doctor/list"} onClick={closeMenu}>
						<Box height="70px" display="flex">
							<Image src={doctor} boxSize={5} mr="6%" alt="doctor" />
							Doctor
						</Box>
					</Link>
					<Link to={"/patient/list"} onClick={closeMenu}>
						<Box height="70px" display="flex">
							<Image src={patient} boxSize={5} mr="6%" alt="patient" />
							Patient
						</Box>
					</Link>
					<Link to={"/appointment/list"} onClick={closeMenu}>
						<Box height="70px" display="flex">
							<Image src={appointment} boxSize={5} mr="6%" alt="appointment" />
							Appointment
						</Box>
					</Link>
					<Link to={"/prescription/list"} onClick={closeMenu}>
						<Box height="70px" display="flex">
							<Image
								src={prescription}
								boxSize={5}
								mr="6%"
								alt="prescription"
							/>
							Prescription
						</Box>
					</Link>
					<Link to={"/staff/list"} onClick={closeMenu}>
						<Box height="70px" display="flex">
							<Image src={staff} boxSize={5} mr="6%" alt="staff" />
							Staff
						</Box>
					</Link>
					<Link to={"/settings/list"} onClick={closeMenu}>
						<Box height="70px" display="flex">
							<Image src={settings} boxSize={5} mr="6%" alt="settings" />
							Settings
						</Box>
					</Link>
				</>
			);
		}
		if (officestaff && !superadmin && !systemadmin && !doctorRole) {
			return (
				<>
					<Link to={"/dashboard"} onClick={closeMenu}>
						<Box height="70px" display="flex">
							<Image src={dashboardimg} boxSize={5} alt="dashboard" mr="6%" />
							Dashboard
						</Box>
					</Link>
					<Link to={"/patient/list"} onClick={closeMenu}>
						<Box height="70px" display="flex">
							<Image src={patient} boxSize={5} mr="6%" alt="patient" />
							Patient
						</Box>
					</Link>
					<Link to={"/appointment/list"} onClick={closeMenu}>
						<Box height="70px" display="flex">
							<Image src={appointment} boxSize={5} mr="6%" alt="appointment" />
							Appointment
						</Box>
					</Link>
					<Link to={"/prescription/list"} onClick={closeMenu}>
						<Box height="70px" display="flex">
							<Image
								src={prescription}
								boxSize={5}
								mr="6%"
								alt="prescription"
							/>
							Prescription
						</Box>
					</Link>
					<Link to={"/staff/list"} onClick={closeMenu}>
						<Box height="70px" display="flex">
							<Image src={staff} boxSize={5} mr="6%" alt="staff" />
							Staff
						</Box>
					</Link>
					<Link to={"/settings/list"} onClick={closeMenu}>
						<Box height="70px" display="flex">
							<Image src={settings} boxSize={5} mr="6%" alt="settings" />
							Settings
						</Box>
					</Link>
				</>
			);
		}
		if (systemadmin && doctorRole && !officestaff && !superadmin) {
			return (
				<>
					<Link to={"/dashboard"} onClick={closeMenu}>
						<Box height="70px" display="flex">
							<Image src={dashboardimg} boxSize={5} alt="dashboard" mr="6%" />
							Dashboard
						</Box>
					</Link>
					<Link to={"/doctor/list"} onClick={closeMenu}>
						<Box height="70px" display="flex">
							<Image src={doctor} boxSize={5} mr="6%" alt="doctor" />
							Doctor
						</Box>
					</Link>
					<Link to={"/patient/list"} onClick={closeMenu}>
						<Box height="70px" display="flex">
							<Image src={patient} boxSize={5} mr="6%" alt="patient" />
							Patient
						</Box>
					</Link>
					<Link to={"/appointment/list"} onClick={closeMenu}>
						<Box height="70px" display="flex">
							<Image src={appointment} boxSize={5} mr="6%" alt="appointment" />
							Appointment
						</Box>
					</Link>
					<Link to={"/prescription/list"} onClick={closeMenu}>
						<Box height="70px" display="flex">
							<Image
								src={prescription}
								boxSize={5}
								mr="6%"
								alt="prescription"
							/>
							Prescription
						</Box>
					</Link>
					<Link to={"/staff/list"} onClick={closeMenu}>
						<Box height="70px" display="flex">
							<Image src={staff} boxSize={5} mr="6%" alt="staff" />
							Staff
						</Box>
					</Link>
					<Link to={"/settings/list"} onClick={closeMenu}>
						<Box height="70px" display="flex">
							<Image src={settings} boxSize={5} mr="6%" alt="settings" />
							Settings
						</Box>
					</Link>
					<Link to={"/user/list"} onClick={closeMenu}>
						<Box height="70px" display="flex">
							<Image src={user} boxSize={5} mr="6%" alt="users" />
							Users
						</Box>
					</Link>
				</>
			);
		}
		if (systemadmin && officestaff && !doctorRole && !superadmin) {
			return (
				<>
					<Link to={"/dashboard"} onClick={closeMenu}>
						<Box height="70px" display="flex">
							<Image src={dashboardimg} boxSize={5} alt="dashboard" mr="6%" />
							Dashboard
						</Box>
					</Link>
					<Link to={"/doctor/list"} onClick={closeMenu}>
						<Box height="70px" display="flex">
							<Image src={doctor} boxSize={5} mr="6%" alt="doctor" />
							Doctor
						</Box>
					</Link>
					<Link to={"/patient/list"} onClick={closeMenu}>
						<Box height="70px" display="flex">
							<Image src={patient} boxSize={5} mr="6%" alt="patient" />
							Patient
						</Box>
					</Link>
					<Link to={"/appointment/list"} onClick={closeMenu}>
						<Box height="70px" display="flex">
							<Image src={appointment} boxSize={5} mr="6%" alt="appointment" />
							Appointment
						</Box>
					</Link>
					<Link to={"/prescription/list"} onClick={closeMenu}>
						<Box height="70px" display="flex">
							<Image
								src={prescription}
								boxSize={5}
								mr="6%"
								alt="prescription"
							/>
							Prescription
						</Box>
					</Link>
					<Link to={"/staff/list"} onClick={closeMenu}>
						<Box height="70px" display="flex">
							<Image src={staff} boxSize={5} mr="6%" alt="staff" />
							Staff
						</Box>
					</Link>
					<Link to={"/settings/list"} onClick={closeMenu}>
						<Box height="70px" display="flex">
							<Image src={settings} boxSize={5} mr="6%" alt="settings" />
							Settings
						</Box>
					</Link>
					<Link to={"/user/list"} onClick={closeMenu}>
						<Box height="70px" display="flex">
							<Image src={user} boxSize={5} mr="6%" alt="users" />
							Users
						</Box>
					</Link>
				</>
			);
		}
		if (officestaff && doctorRole && !systemadmin && !superadmin) {
			return (
				<>
					<Link to={"/dashboard"} onClick={closeMenu}>
						<Box height="70px" display="flex">
							<Image src={dashboardimg} boxSize={5} alt="dashboard" mr="6%" />
							Dashboard
						</Box>
					</Link>
					<Link to={"/doctor/list"} onClick={closeMenu}>
						<Box height="70px" display="flex">
							<Image src={doctor} boxSize={5} mr="6%" alt="doctor" />
							Doctor
						</Box>
					</Link>
					<Link to={"/patient/list"} onClick={closeMenu}>
						<Box height="70px" display="flex">
							<Image src={patient} boxSize={5} mr="6%" alt="patient" />
							Patient
						</Box>
					</Link>
					<Link to={"/appointment/list"} onClick={closeMenu}>
						<Box height="70px" display="flex">
							<Image src={appointment} boxSize={5} mr="6%" alt="appointment" />
							Appointment
						</Box>
					</Link>
					<Link to={"/prescription/list"} onClick={closeMenu}>
						<Box height="70px" display="flex">
							<Image
								src={prescription}
								boxSize={5}
								mr="6%"
								alt="prescription"
							/>
							Prescription
						</Box>
					</Link>
					<Link to={"/staff/list"} onClick={closeMenu}>
						<Box height="70px" display="flex">
							<Image src={staff} boxSize={5} mr="6%" alt="staff" />
							Staff
						</Box>
					</Link>
					<Link to={"/settings/list"} onClick={closeMenu}>
						<Box height="70px" display="flex">
							<Image src={settings} boxSize={5} mr="6%" alt="settings" />
							Settings
						</Box>
					</Link>
					<Link to={"/user/list"} onClick={closeMenu}>
						<Box height="70px" display="flex">
							<Image src={user} boxSize={5} mr="6%" alt="users" />
							Users
						</Box>
					</Link>
				</>
			);
		}
		if (systemadmin && doctorRole && officestaff && !superadmin) {
			return (
				<>
					<Link to={"/dashboard"} onClick={closeMenu}>
						<Box height="70px" display="flex">
							<Image src={dashboardimg} boxSize={5} alt="dashboard" mr="6%" />
							Dashboard
						</Box>
					</Link>
					<Link to={"/doctor/list"} onClick={closeMenu}>
						<Box height="70px" display="flex">
							<Image src={doctor} boxSize={5} mr="6%" alt="doctor" />
							Doctor
						</Box>
					</Link>
					<Link to={"/patient/list"} onClick={closeMenu}>
						<Box height="70px" display="flex">
							<Image src={patient} boxSize={5} mr="6%" alt="patient" />
							Patient
						</Box>
					</Link>
					<Link to={"/appointment/list"} onClick={closeMenu}>
						<Box height="70px" display="flex">
							<Image src={appointment} boxSize={5} mr="6%" alt="appointment" />
							Appointment
						</Box>
					</Link>
					<Link to={"/prescription/list"} onClick={closeMenu}>
						<Box height="70px" display="flex">
							<Image
								src={prescription}
								boxSize={5}
								mr="6%"
								alt="prescription"
							/>
							Prescription
						</Box>
					</Link>
					<Link to={"/staff/list"} onClick={closeMenu}>
						<Box height="70px" display="flex">
							<Image src={staff} boxSize={5} mr="6%" alt="staff" />
							Staff
						</Box>
					</Link>
					<Link to={"/settings/list"} onClick={closeMenu}>
						<Box height="70px" display="flex">
							<Image src={settings} boxSize={5} mr="6%" alt="settings" />
							Settings
						</Box>
					</Link>
					<Link to={"/user/list"} onClick={closeMenu}>
						<Box height="70px" display="flex">
							<Image src={user} boxSize={5} mr="6%" alt="users" />
							Users
						</Box>
					</Link>
				</>
			);
		}
	};
	const [isOpen, setIsOpen] = useState(false);

	const toggleMenu = () => {
		setIsOpen(!isOpen);
	};
	const closeMenu = () => {
		setIsOpen(false);
	};

	return (
		<Box zIndex={100}>
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

			{add && (
				<Box>
					<Box bg="#FFFFFF">
						<Center>
							<Box width={{ md: "80%", base: "90%", lg: "70%" }}>
								{form && (
									<Flex align="center" justify="space-between">
										<Box
											pt="4"
											pb="4"
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
								{dashboard && (
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
												<Image mr="8%" height="32px" width="82px" src={Logo} />
											</Center>
										</Box>
										<Box>
											<LogoutButton />
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
													<Image src={Logo} alt="logo" />
												</DrawerHeader>
												<DrawerCloseButton />
												<DrawerBody>{renderMenuItems()}</DrawerBody>
											</DrawerContent>
										</Drawer>
									</Box>
								)}
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
											<Image src={Logo} alt="logo" />
										</DrawerHeader>
										<DrawerCloseButton />
										<DrawerBody>{renderMenuItems()}</DrawerBody>
									</DrawerContent>
								</Drawer>
							</Flex>
							<Center>
								<Box
									//   margin="auto"
									height={"80px"}
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
		</Box>
	);
};

export default HeaderWithButton;
