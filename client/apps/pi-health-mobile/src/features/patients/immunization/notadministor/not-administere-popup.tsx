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

type NotAdministeredPopUpProps = {
	open: boolean;
	close: () => void;
	id: string;
	patientId: string;
};

const NotAdministeredPopUp = ({
	open,
	close,
	id,
}: NotAdministeredPopUpProps) => {
	const notAdminister_id = id;

	const notAdministerData = useLiveQuery(() => db.notadminister.toArray());

	const notAdministerDataById = notAdministerData?.find(
		(item) => item.id === notAdminister_id,
	);

	return (
		<Modal isOpen={open} onClose={close} motionPreset={"slideInBottom"}>
			<ModalOverlay />
			<ModalContent mr={5} ml={5}>
				<ModalHeader textAlign="center">Not Administered Details</ModalHeader>
				<ModalCloseButton />
				<Box margin="1%">
					<>
						<TextField
							heading={"Vaccine:"}
							result={notAdministerDataById?.vaccine}
						/>
						<TextField
							heading={"Type:"}
							result={notAdministerDataById?.types}
						/>
						<TextField
							heading={"Recorded Date & Time:"}
							result={notAdministerDataById?.recorded}
						/>
						<TextField
							heading={"Reason:"}
							result={notAdministerDataById?.reason_for_non_administration}
						/>
						<TextField
							heading={"Comments:"}
							result={notAdministerDataById?.comments}
						/>
					</>
				</Box>
			</ModalContent>
		</Modal>
	);
};
export default NotAdministeredPopUp;
