import {
	Box,
	Modal,
	ModalCloseButton,
	ModalContent,
	ModalHeader,
	ModalOverlay,
} from "@chakra-ui/react";

import TextField from "../../../components/text-field";
import { useLiveQuery } from "dexie-react-hooks";
import { db } from "../../../db/db";

type MedicationPopupProps = {
	open: boolean;
	close: () => void;
	id: string;
	patientId: string;
};

const MedicationPopup = ({ open, close, id }: MedicationPopupProps) => {
	const medication_id = id;

	const medicationData = useLiveQuery(() => db.medications.toArray());

	const medicationDataById = medicationData?.find(
		(item) => item.id === medication_id,
	);

	return (
		<Modal isOpen={open} onClose={close} motionPreset={"slideInBottom"}>
			<ModalOverlay />
			<ModalContent mr={5} ml={5}>
				<ModalHeader textAlign="center">Medication Details</ModalHeader>
				<ModalCloseButton />
				<Box margin="1%">
					<>
						<TextField
							heading={"Status:"}
							result={medicationDataById?.status}
						/>
						<TextField heading={"Drug:"} result={medicationDataById?.drug} />
						<TextField
							heading={"Instructions:"}
							result={medicationDataById?.instruction}
						/>
						<TextField
							heading={"Comments:"}
							result={medicationDataById?.comments}
						/>
					</>
				</Box>
			</ModalContent>
		</Modal>
	);
};

export default MedicationPopup;
