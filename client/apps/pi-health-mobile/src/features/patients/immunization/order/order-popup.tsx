import {
	Box,
	Modal,
	ModalCloseButton,
	ModalContent,
	ModalHeader,
	ModalOverlay,
} from "@chakra-ui/react";
import TextField from "../../../../components/text-field";
import { useLiveQuery } from "dexie-react-hooks";
import { db } from "../../../../db/db";

type OrderPopupProps = {
	open: boolean;
	close: () => void;
	id: string;
};

const OrderPopup = ({ open, close, id }: OrderPopupProps) => {
	const order_id = id;

	const orderData = useLiveQuery(() => db.orders.toArray());

	const orderDataById = orderData?.find((item) => item.id === order_id);

	return (
		<Modal isOpen={open} onClose={close} motionPreset={"slideInBottom"}>
			<ModalOverlay />
			<ModalContent mr={5} ml={5}>
				<ModalHeader textAlign="center">New Order Details</ModalHeader>
				<ModalCloseButton />
				<Box margin="1%">
					<>
						<TextField heading={"Vaccine:"} result={orderDataById?.vaccine} />
						<TextField heading={"Type:"} result={orderDataById?.types} />
						<TextField
							heading={"Ordered Date & Time:"}
							result={orderDataById?.ordered}
						/>
						<TextField heading={"Provider:"} result={orderDataById?.provider} />
						<TextField heading={"Comments:"} result={orderDataById?.comments} />
					</>
				</Box>
			</ModalContent>
		</Modal>
	);
};

export default OrderPopup;
