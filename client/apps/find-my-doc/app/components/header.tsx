"use client";
import React, { useState } from "react";
import { Box, Button, IconButton, Image, Link } from "@chakra-ui/react";
import { HamburgerIcon, CloseIcon } from "@chakra-ui/icons";
const Header = () => {
	const [isMenuOpen, setIsMenuOpen] = useState(false);
	const toggleMenu = () => {
		setIsMenuOpen(!isMenuOpen);
	};
	return (
		<div>
			<Box
				display={"flex"}
				alignItems={"center"}
				justifyContent={"space-between"}
				px={3}
				backgroundColor={"#F6FBFD"}
				height="10%"
			>
				<Box>
					<Link href="/">
						<Image
							width={"100%"}
							height="100%"
							src="findmydoc logo.svg"
							alt=""
							mt={{ md: "10%", base: "10%" }}
							ml={{ lg: "18%", md: "0%" }}
						/>
					</Link>
				</Box>
				<Box ml="30% " display={{ base: "none", md: "flex" }} color={"black"}>
					<Link
						href="/"
						fontSize={{ md: "19px", base: "8px" }}
						marginX={5}
						marginTop="8%"
					>
						Find
					</Link>

					<Link
						href="/contact"
						fontSize={{ md: "19px", base: "8px" }}
						marginTop="8%"
						marginX="4"
					>
						Contact
					</Link>

					<Link href="/sign-in">
						<Button
							mt="25%"
							marginX="3"
							// margin={{ md: "16px", base: "2px" }}
							backgroundColor={"#FED337"}
							color={"white"}
							_hover={{ bg: "#FED337", color: "white" }}
						>
							Log In
						</Button>
					</Link>
					<Link href="/sign-up">
						<Button
							mt="23%"
							marginX="3"
							backgroundColor={"#F6FBFD"}
							border={"1px"}
							borderColor="#FED337"
							color={"black"}
							_hover={{ bg: "#F6FBFD" }}
						>
							Sign Up
						</Button>
					</Link>
				</Box>
				<Box display={{ base: "flex", md: "none" }} ml="50%" mt="5%">
					<IconButton
						icon={isMenuOpen ? <CloseIcon /> : <HamburgerIcon />}
						onClick={toggleMenu}
						variant={"ghost"}
						aria-label={"Hamburger"}
						color={"black"}
					/>
				</Box>
			</Box>

			<Box
				display={{ base: isMenuOpen ? "flex" : "none", lg: "none" }}
				flexDirection="column"
				p={2}
				mt={0}
				bg="#F6FBFD"
				alignItems="flex-end"
			>
				<Link href="" fontSize={"20px"} color={"black"} margin={"11px"}>
					Find
				</Link>
				<Link fontSize={"20px"} color={"black"} margin={"11px"}>
					Contact
				</Link>
				<Link href="/price" fontSize={"20px"} color={"black"} margin={"11px"}>
					Pricing
				</Link>
				<Link href="/sign-in">
					<Button
						margin={"11px"}
						backgroundColor={"#FED337"}
						color={"white"}
						_hover={{ bg: "#FED337", color: "white" }}
					>
						Log In
					</Button>
				</Link>
				<Link href="/sign-up">
					<Button
						margin={"11px"}
						backgroundColor={"white"}
						border={"1px"}
						borderColor="#FED337"
						color={"black"}
					>
						Sign Up
					</Button>
				</Link>
			</Box>
		</div>
	);
};
export default Header;
