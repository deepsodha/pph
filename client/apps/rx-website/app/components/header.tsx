"use client";
import React, { useState } from "react";
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
import { HamburgerIcon, CloseIcon, ChevronDownIcon } from "@chakra-ui/icons";
const Header = () => {
	const [isMenuOpen, setIsMenuOpen] = useState(false);
	const toggleMenu = () => {
		setIsMenuOpen(!isMenuOpen);
	};

	return (
		<>
			<Box
				marginRight={"15%"}
				marginTop={"0.2%"}
				marginBottom="0.2%"
				display={{ base: "none", md: "none", lg: "flex" }}
				justifyContent={{ md: "flex-end", base: "flex-end" }}
			>
				<Box marginRight={"42%"}>
					<Text fontSize={"14px"}>Your Ultimate Medicine Partner</Text>
				</Box>
				<Box display={"flex"}>
					<Image
						src="/Gmail.svg"
						alt=""
						marginRight={"2%"}
						ml={{ base: "5%" }}
					/>
					<Text fontSize={{ md: "15px", base: "10px" }}>
						support@Rx.pihealth.com
					</Text>
				</Box>
				<Box display={"flex"}>
					<Image src="/Icon.svg" alt="" marginLeft={"14%"} marginRight={"3%"} />
					<Text fontSize={{ md: "15px", base: "10px" }}>+912717453330</Text>
				</Box>
			</Box>
			<Box
				display={"flex"}
				alignItems={"center"}
				justifyContent={"space-around"}
				px={3}
				backgroundColor={"#EBF4F9"}
				height="10%"
			>
				<Box>
					<Link href="/">
						<Image
							width={"100%"}
							height="50%"
							src="/Logo - 2 3.svg"
							alt=""
							ml={{ lg: "100%", md: "20%" }}
						/>
					</Link>
				</Box>
				<Box marginLeft="8%" display={{ base: "none", md: "flex" }}>
					<Menu>
						<MenuButton
							as={Link}
							fontSize={{ md: "16px", base: "8px" }}
							marginTop="4%"
							marginX="5"
							marginY="3"
							color={"black"}
						>
							Drugs & Supplements
							<ChevronDownIcon
								height={"14px"}
								marginLeft={"6px"}
								width={"18px"}
							/>
						</MenuButton>
						<MenuList
							width="2%"
							display="flex"
							flexWrap="wrap"
							p="2"
							backgroundColor="#2A2B2D"
							color="black"
							borderRadius="md"
							padding="4%"
						>
							<Link href="/drug-search">
								<MenuItem backgroundColor="#2A2B2D" color="white">
									Find A-Z Drugs
								</MenuItem>
							</Link>
							<Link href="/ndc">
								<MenuItem backgroundColor="#2A2B2D" color="white">
									National Drug Code
								</MenuItem>
							</Link>
							<Link href="find-pharmacy">
								<MenuItem backgroundColor="#2A2B2D" color="white">
									Find Pharmacy
								</MenuItem>
							</Link>
						</MenuList>
					</Menu>
					<Link
						fontSize={{ md: "16px", base: "8px" }}
						marginTop="4%"
						color={"black"}
						marginY="3"
					>
						Find Your Doctor
					</Link>
					<Link
						fontSize={{ md: "16px", base: "8px" }}
						marginTop="4%"
						color={"black"}
						marginY="3"
						marginX="4"
						href="/contact-us"
					>
						Contact us
					</Link>
				</Box>
				<Box marginRight={"5%"} display={{ base: "none", md: "flex" }}>
					<Link
						href="/sign-in"
						fontSize={"16px"}
						marginX="3"
						marginY="4"
						color={"black"}
						fontWeight={400}
					>
						Login
					</Link>
					<Link href="/sign-up">
						<Button
							backgroundColor={"#095FBA"}
							color={"white"}
							borderRadius="20px"
							margin={"7%"}
							_hover={{
								backgroundColor: "white",
								color: "#095FBA",
								borderColor: "#095FBA",
								border: "1px",
							}}
						>
							Sign Up
						</Button>
					</Link>
				</Box>
				<Box display={{ base: "flex", lg: "none", md: "none" }}>
					<Button variant="ghost" onClick={toggleMenu}>
						{isMenuOpen ? (
							<CloseIcon color={"black"} />
						) : (
							<HamburgerIcon color={"black"} />
						)}
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
						as={Link}
						fontSize={"20px"}
						marginTop="4%"
						marginX="3"
						marginY="3"
						color={"black"}
					>
						Drugs & Supplements
						<ChevronDownIcon
							height={"14px"}
							marginLeft={"6px"}
							width={"18px"}
						/>
					</MenuButton>
					<MenuList
						width="2%"
						display="flex"
						flexWrap="wrap"
						p="2"
						backgroundColor="#2A2B2D"
						color="black"
						borderRadius="md"
						padding="4%"
					>
						<Link href="/drug-search">
							<MenuItem backgroundColor="#2A2B2D" color="white">
								Find A-Z Drugs
							</MenuItem>
						</Link>
						<Link href="/ndc">
							<MenuItem backgroundColor="#2A2B2D" color="white">
								National Drug Code
							</MenuItem>
						</Link>
						<Link href="find-pharmacy">
							<MenuItem backgroundColor="#2A2B2D" color="white">
								Find Pharmacy
							</MenuItem>
						</Link>
					</MenuList>
				</Menu>
				<Link fontSize={"20px"} margin={"12px"} color={"black"}>
					Find your doctor
				</Link>
				<Link
					fontSize={"20px"}
					margin={"12px"}
					color={"black"}
					href="/contact-us"
				>
					Contact Us
				</Link>
				<Link href="/sign-in" color="black" fontSize={"20px"} margin={"12px"}>
					Login
				</Link>
				<Link href="/sign-up">
					<Button
						margin={"9px"}
						backgroundColor={"#095FBA"}
						border={"1px"}
						color="white"
						borderRadius="20px"
					>
						Sign up
					</Button>
				</Link>
			</Box>
		</>
	);
};
export default Header;
