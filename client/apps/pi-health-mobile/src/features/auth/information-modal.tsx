import {
	Modal,
	ModalOverlay,
	ModalContent,
	ModalCloseButton,
	ModalBody,
	Button,
	Image,
	Box,
	useDisclosure,
	Center,
	Text,
	Link,
} from "@chakra-ui/react";
import Info from "../../assets/Info.svg";
import mail from "../../assets/mail.svg";
import call from "../../assets/call.svg";
import location from "../../assets/location.svg";

const InformationModal = () => {
	const phoneNumber = "+912717453330";
	const { isOpen, onOpen, onClose } = useDisclosure();
	const {
		isOpen: isAboutUsOpen,
		onOpen: onAboutUsOpen,
		onClose: onAboutUsClose,
	} = useDisclosure();
	const {
		isOpen: isContactUsOpen,
		onOpen: onContactUsOpen,
		onClose: onContactUsClose,
	} = useDisclosure();
	return (
		<div>
			<Box justifyContent={"end"} display={"flex"} mr={6} onClick={onOpen}>
				<Image
					src={Info}
					height={10}
					width={10}
					position={"fixed"}
					bottom={"5%"}
				/>
				<Modal
					isCentered
					onClose={onClose}
					isOpen={isOpen}
					motionPreset="slideInBottom"
				>
					<ModalOverlay />
					<ModalContent
						style={{
							width: "70%",
							position: "absolute",
							bottom: "0", // Align to the bottom
							right: "0", // Align to the right
							transform: "translate(0, 0)", // Adjust as needed
						}}
					>
						<ModalCloseButton />
						<ModalBody mt={2}>
							<Box>
								<Button
									width={"80%"}
									bg={"transparent"}
									onClick={onAboutUsOpen}
								>
									About Us
								</Button>
								<Button
									width={"80%"}
									bg={"transparent"}
									onClick={onContactUsOpen}
								>
									Contact Us
								</Button>
							</Box>
						</ModalBody>
					</ModalContent>
				</Modal>
				<Modal
					isCentered
					onClose={onAboutUsClose}
					isOpen={isAboutUsOpen}
					motionPreset="slideInBottom"
				>
					<ModalOverlay />
					<ModalContent>
						<ModalCloseButton />
						<ModalBody mt={2}>
							<Box>
								<Center>
									<Text fontWeight={"bold"}>About Us</Text>
								</Center>
								<br />
								<p>
									Your all-in-one solution for efficient and hassle-free
									community living. Streamline communication, automate
									announcements, manage amenities, and enhance security
									seamlessly.
									<br />
									<br /> Our comprehensive platform is designed to simplify
									every aspect of managing and living in a society. Security is
									paramount;our application offers enhanced access control and
									visitor tracking for peace of mind.
									<br />
									<br /> With real-time updates and data analytics, you can make
									informed decisions and create a harmonious living environment.
								</p>
							</Box>
						</ModalBody>
					</ModalContent>
				</Modal>

				<Modal
					isCentered
					onClose={onContactUsClose}
					isOpen={isContactUsOpen}
					motionPreset="slideInBottom"
				>
					<ModalOverlay />
					<ModalContent>
						<ModalCloseButton />
						<ModalBody mt={2}>
							<Box>
								<Center>
									<Text fontWeight={"bold"}>Contact Us</Text>
								</Center>
								<Box display={"flex"} pt={5}>
									<Image src={mail} />

									<Link ml={3} href="mailto:contact@fuzzycloud.in" isExternal>
										<Text ml={30}>contact@fuzzycloud.in</Text>
									</Link>
								</Box>
								<Box display={"flex"} pt={5}>
									<Image src={call} />
									<a
										href={`tel:${phoneNumber}`}
										style={{ textDecoration: "none" }}
									>
										<Text ml={30}>{phoneNumber}</Text>
									</a>
								</Box>
								<Box display={"flex"} pt={5}>
									<Image src={location} />
									<Link href="https://www.google.com/maps/place/Satyamev+Elite,+608+-+615,+Sardar+Patel+Ring+Rd,+South+Bopal,+Bopal,+Ahmedabad,+Gujarat+380058/@23.0250297,72.4762499,17z/data=!3m1!4b1!4m6!3m5!1s0x395e9b0bb2915c0b:0x13f04f86b21ee36f!8m2!3d23.0250297!4d72.4762499!16s%2Fg%2F11s9v338p1?entry=ttu">
										<Text ml={30}>
											1001, Fuzzy Cloud, Satyamev Elite, Ahmedabad, India
										</Text>
									</Link>
								</Box>
							</Box>
						</ModalBody>
					</ModalContent>
				</Modal>
			</Box>
		</div>
	);
};

export default InformationModal;
