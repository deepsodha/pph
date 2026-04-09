"use client";
import type React from "react";
import { useState } from "react";
import {
	Box,
	Button,
	Image,
	Link,
	Menu,
	MenuButton,
	MenuItem,
	MenuList,
	Text,
} from "@chakra-ui/react";
import { CloseIcon, HamburgerIcon } from "@chakra-ui/icons";
const Header: React.FC = () => {
	const [isMenuOpen, setIsMenuOpen] = useState(false);
	const toggleMenu = () => {
		setIsMenuOpen(!isMenuOpen);
	};
	return (
		<>
			<Box
				display={{ base: "none", md: "none", lg: "flex" }}
				justifyContent={{ md: "flex-end", base: "flex-end" }}
				m="0.6% 1.5% 0.6% 0"
			>
				<Box display={"flex"}>
					<Image
						src="Gmail Icon.svg"
						alt=""
						boxSize={{ md: "auto", base: "16px" }}
						marginRight={"2%"}
						ml={{ base: "5%" }}
					/>
					<Text fontSize={{ md: "15px", base: "10px" }}>
						support@pihealth.com
					</Text>
				</Box>
				<Box display={"flex"} marginLeft={"2%"}>
					<Image
						src="Phone Icon.svg"
						alt=""
						boxSize={{ md: "auto", base: "16px" }}
						marginLeft={"8%"}
						marginRight={"3%"}
					/>
					<Text fontSize={{ md: "15px", base: "10px" }} mr="50px">
						+912717453330
					</Text>
				</Box>
			</Box>

			<Box
				display={"flex"}
				alignItems={"center"}
				justifyContent={"space-between"}
				px={"0.5%"}
				backgroundColor={"#DDEDFA"}
				height="10%"
			>
				<Box>
					<Link href="/">
						<Image
							width={"100%"}
							height="100%"
							src="Pi-health logo (2).svg"
							alt=""
							mt={{ md: "4%", base: "4%" }}
							ml={{ lg: "20%", md: "0%" }}
						/>
					</Link>
				</Box>
				<Box display={{ base: "none", md: "flex" }}>
					<Menu>
						<MenuButton
							fontSize={{ md: "19px", base: "8px" }}
							marginTop="4%"
							color={"black"}
							marginX="3"
							as={Link}
						>
							All Products
						</MenuButton>
						<MenuList>
							<Link>
								<MenuItem>Rx</MenuItem>
							</Link>
							<Link>
								<MenuItem>Pi-Health</MenuItem>
							</Link>
							<Link>
								<MenuItem>Find My Doc</MenuItem>
							</Link>
						</MenuList>
					</Menu>
					<Link
						fontSize={{ md: "19px", base: "8px" }}
						marginTop="4%"
						color={"black"}
						marginX="3"
						href="/price"
					>
						Pricing
					</Link>
					<Link
						fontSize={{ md: "19px", base: "8px" }}
						marginTop="4%"
						color={"black"}
						marginX="5"
						href="/contact-us"
					>
						Contact Us
					</Link>
					<Link href="/sign-in">
						<Button
							mt={{ md: "22px", base: "2px", lg: "20px" }}
							backgroundColor={"#DDEDFA"}
							border={"1px"}
							borderColor="#3A7AFE"
							color={"black"}
							mr={1}
							_hover={{ bg: "#095FBA", color: "white" }}
						>
							Log In
						</Button>
					</Link>
					<Link href="/free-trial">
						<Button
							margin={{ md: "22px", base: "2px", lg: "20px" }}
							backgroundColor={"#095FBA"}
							color={"white"}
						>
							Free Trial
						</Button>
					</Link>
					``
				</Box>
				<Box display={{ base: "flex", md: "none" }} ml="auto">
					<Button onClick={toggleMenu} variant={"ghost"} color={"black"}>
						{isMenuOpen ? <CloseIcon /> : <HamburgerIcon />}
					</Button>
				</Box>
			</Box>

			<Box
				display={{ base: isMenuOpen ? "flex" : "none", lg: "none" }}
				flexDirection="column"
				p={2}
				mt={0}
				bg="#F6FBFD"
				alignItems="flex-start"
			>
				<Menu>
					<MenuButton
						color={"black"}
						fontSize={{ md: "20px", base: "20px" }}
						marginX="3"
					>
						All Products
					</MenuButton>
					<MenuList>
						<Link>
							<MenuItem>Rx</MenuItem>
						</Link>
						<Link>
							<MenuItem>Pi-Health</MenuItem>
						</Link>
						<Link>
							<MenuItem>Find My Doc</MenuItem>
						</Link>
					</MenuList>
				</Menu>
				<Link href="/price" fontSize={"20px"} color={"black"} margin={"11px"}>
					Pricing
				</Link>
				<Link href="/contact-us">
					<Button margin={"11px"} backgroundColor={"#095FBA"} color={"white"}>
						Contact Us
					</Button>
				</Link>
				<Link href="/sign-in">
					<Button
						margin={"11px"}
						backgroundColor={"white"}
						border={"1px"}
						borderColor="#3A7AFE"
						color={"black"}
					>
						Login
					</Button>
				</Link>
			</Box>
		</>
	);
};
export default Header;
