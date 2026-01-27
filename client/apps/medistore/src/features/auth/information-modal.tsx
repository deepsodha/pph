import {
	Box,
	Center,
	Modal,
	ModalCloseButton,
	ModalContent,
	ModalHeader,
	ModalOverlay,
} from "@chakra-ui/react";

type InformationModalProps = {
	open: boolean;
	close: () => void;
};
const InformationModal = ({ open, close }: InformationModalProps) => {
	return (
		<Box>
			<Modal isOpen={open} onClose={close} motionPreset={"slideInBottom"}>
				<ModalOverlay />
				<ModalContent mr={5} ml={5}>
					<ModalHeader textAlign="center">
						Privacy Policy & Terms and Conditions
					</ModalHeader>
					<ModalCloseButton />
					<Center>Privacy Policy & Terms and Conditions</Center>
				</ModalContent>
			</Modal>
		</Box>
	);
};

export default InformationModal;
