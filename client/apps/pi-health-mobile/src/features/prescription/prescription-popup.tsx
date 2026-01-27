import {
	Box,
	Modal,
	ModalCloseButton,
	ModalContent,
	ModalHeader,
	ModalOverlay,
} from "@chakra-ui/react";
import TextField from "../../components/text-field";
import { useLiveQuery } from "dexie-react-hooks";
import { db } from "../../db/db";
import { convertUTCtoLocalDate } from "../../../../../packages/ui/component/utc-date-to-normal-date";

type PrescriptionPopupProps = {
	open: boolean;
	close: () => void;
	id: string;
};

const PrescriptionPopup = ({ open, close, id }: PrescriptionPopupProps) => {
	const prescription_id = id;

	const PrescriptionData = useLiveQuery(() => db.prescription.toArray());

	const PrescriptionDataById = PrescriptionData?.find(
		(item) => item.id === prescription_id,
	);

	return (
		<Modal isOpen={open} onClose={close} motionPreset={"slideInBottom"}>
			<ModalOverlay />
			<ModalContent>
				<ModalHeader textAlign="center">Prescription Details</ModalHeader>
				<ModalCloseButton />
				<Box margin="1%">
					<TextField
						heading={"Patient Name:"}
						result={PrescriptionDataById?.patient_name}
					/>
					<TextField
						heading={"Prescription Name:"}
						result={PrescriptionDataById?.presecription_name}
					/>
					<TextField
						heading={"Instruction:"}
						result={PrescriptionDataById?.instruction}
					/>
					<TextField
						heading={"Date:"}
						result={
							PrescriptionDataById?.date &&
							convertUTCtoLocalDate(PrescriptionDataById?.date)
						}
					/>
					<TextField
						heading={"Drug Name:"}
						result={PrescriptionDataById?.drug_name}
					/>
				</Box>
			</ModalContent>
		</Modal>
	);
};

export default PrescriptionPopup;
