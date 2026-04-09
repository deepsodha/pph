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
type VitalsPopupProps = {
	open: boolean;
	close: () => void;
	id: string;
};
const VitalsPopup = ({ open, close, id }: VitalsPopupProps) => {
	const vitals_id = id;

	const vitalsData = useLiveQuery(() => db.vitals.toArray());

	const vitalsDataById = vitalsData?.find((item) => item.id === vitals_id);

	return (
		<div>
			<Modal isOpen={open} onClose={close} motionPreset={"slideInBottom"}>
				<ModalOverlay />
				<ModalContent marginRight={5} ml={5}>
					<ModalHeader textAlign="center">Vitals Details</ModalHeader>
					<ModalCloseButton />
					<Box margin="1%">
						<TextField
							heading={"Date & Time:"}
							result={vitalsDataById?.date_time}
						/>
						<TextField
							heading={"Blood Pressure:"}
							result={vitalsDataById?.blood_pressure}
						/>
						<TextField
							heading={"Heart Rate:"}
							result={vitalsDataById?.heart_rate}
						/>
						<TextField heading={"Height:"} result={vitalsDataById?.height} />
						<TextField heading={"Weight:"} result={vitalsDataById?.weight} />
						<TextField
							heading={"Temprature:"}
							result={vitalsDataById?.temperature}
						/>
						<TextField heading={" BMI:"} result={vitalsDataById?.bmi} />
						<TextField
							heading={"Comments:"}
							result={vitalsDataById?.comments}
						/>
					</Box>
				</ModalContent>
			</Modal>
		</div>
	);
};

export default VitalsPopup;
