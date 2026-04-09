import {
	Modal,
	ModalOverlay,
	ModalContent,
	ModalCloseButton,
	Box,
	HStack,
	Button,
	Image,
	Text,
	ModalHeader,
	Center,
} from "@chakra-ui/react";
import { useState } from "react";
import { motion } from "framer-motion";

import Delete from "../assets/left-icon.svg";

type DeleteModalProps = {
	open: boolean;
	close: () => void;
	handle_delete?: () => void;
};

const DeleteModal = ({ open, close, handle_delete }: DeleteModalProps) => {
	const [, setIsAnimating] = useState(false);

	const handleDeleteWithAnimation = () => {
		setIsAnimating(true);
		setTimeout(() => {
			handle_delete?.();
			close();
		}, 500);
	};

	return (
		<div>
			<Modal isOpen={open} onClose={close}>
				<ModalOverlay />
				<Box>
					<ModalContent>
						<ModalHeader>Delete Confirmation</ModalHeader>
						<ModalCloseButton />
						<Box alignItems="center">
							<Text mb={5} ml={6}>
								Are you sure you want to delete?
							</Text>
						</Box>
						<Center>
							<HStack mb={5}>
								<motion.div whileHover={{ scale: 1.1 }}>
									{/* Wrap the delete button with motion.div for hover animation */}
									<Button
										size="md"
										bgColor={"#FF5050"}
										color={"white"}
										width="140px"
										_hover={{
											bgColor: "Grey",
											color: "red",
											border: "1px solid #F58634",
										}}
										onClick={handleDeleteWithAnimation}
										alignItems={"center"}
										justifyItems={"center"}
										display={"flex"}
										alignContent={"center"}
									>
										<Center gap={2}>
											<Image height="40%" width="40%" src={Delete} />
											<span>Delete</span>
										</Center>
									</Button>
								</motion.div>
								<motion.div whileHover={{ scale: 1.1 }}>
									{/* Wrap the cancel button with motion.div for hover animation */}
									<Button
										size="md"
										bgColor={"#EDF2F7"}
										color={"#717B9E"}
										mx={2}
										_hover={{
											bgColor: "white",
											color: "#F58634",
											border: "1px solid #F58634",
										}}
										width="140px"
										onClick={close}
									>
										Cancel
									</Button>
								</motion.div>
							</HStack>
						</Center>
					</ModalContent>
				</Box>
			</Modal>
		</div>
	);
};

export default DeleteModal;
