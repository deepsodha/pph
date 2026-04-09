import { Box, Center, Text, Image, useDisclosure } from "@chakra-ui/react";
import InformationModal from "./information-modal";
import { LoginForm } from "./loginForm";
import LoginBackground from "../../assets/LoginBg.svg";
import PiHealth from "../../assets/PiHealthLogo.svg";
import Pharmacy from "../../assets/pharmacy.svg";
import PiPharmacy from "../../assets/PiPharmacyLogo.svg";

const login = () => {
	const {
		isOpen: isModalOpen,
		onOpen: openModalBase,
		onClose: closeModal,
	} = useDisclosure();
	const openModal = () => {
		openModalBase();
	};
	return (
		<Box width={"100%"} display={"flex"}>
			<Box width={"50%"} bgColor={"#095FBA"} height={"100vh"}>
				<LoginForm />

				<Center>
					<Box _hover={{ cursor: "pointer" }} mt={100}>
						<Text as="u" color={"#FFFFFF"} onClick={() => openModal()}>
							Privacy Policy & Terms and Conditions
						</Text>
						<InformationModal open={isModalOpen} close={closeModal} />
					</Box>
				</Center>
			</Box>
			<Box width={"50%"} height={"100vh"}>
				<Box
					height={"100%"}
					bgImage={LoginBackground} // Replace '/background-image.jpg' with your actual background image path
					bgSize="cover"
				>
					<Box height={"100%"} position="absolute">
						<Box mt={"35%"} ml={"15%"}>
							<Image src={Pharmacy} height={"160px"} width={"160px"} />
						</Box>
						<Box ml={"41%"}>
							<Image src={PiPharmacy} />
							<Center>
								<Text>Your trusted medicine partner</Text>
							</Center>
						</Box>

						<Box mt={125} ml={"45%"}>
							<Center>
								<Text>Powered by,</Text>
							</Center>
							<Center>
								<Image src={PiHealth} height={"81px"} width={"200px"} />
							</Center>
						</Box>
					</Box>
				</Box>
			</Box>
		</Box>
	);
};

export default login;
