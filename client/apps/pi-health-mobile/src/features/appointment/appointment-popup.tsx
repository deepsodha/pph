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

type AppointmentPopupProps = {
	open: boolean;
	close: () => void;
	id: string;
};

const AppointmentPopup = ({ open, close, id }: AppointmentPopupProps) => {
	const appointment_id = id;

	const appointmentData = useLiveQuery(() => db.appointments.toArray());
	const appointmentDataById = appointmentData?.find(
		(item) => item.id === appointment_id,
	);
	const firstName = appointmentDataById?.staff_id.split(":")[1];
	return (
		<Modal isOpen={open} onClose={close} motionPreset={"slideInBottom"}>
			<ModalOverlay />
			<ModalContent>
				<ModalHeader textAlign="center">Appointment Details</ModalHeader>
				<ModalCloseButton />
				<Box margin="1%">
					<TextField
						heading={"Doctor Name:"}
						result={appointmentDataById?.doctor_name}
					/>
					<TextField
						heading={"Patient Name:"}
						result={appointmentDataById?.patient_name}
					/>
					<TextField
						heading={"Select Visit:"}
						result={appointmentDataById?.visit}
					/>
					<TextField
						heading={"Date:"}
						result={
							appointmentDataById?.date &&
							convertUTCtoLocalDate(appointmentDataById?.date)
						}
					/>
					<TextField
						heading={"Appointment Duration:"}
						result={`${appointmentDataById?.appointment_duration} minutes`}
					/>
					<TextField
						heading={"Choose Appointment:"}
						result={appointmentDataById?.choose_appointment}
					/>
					<TextField heading={"Note:"} result={appointmentDataById?.note} />
					<TextField
						heading={"Room & Equipment No:"}
						result={appointmentDataById?.room_and_equipment_no}
					/>
					<TextField heading={"Staff Id:"} result={firstName} />
				</Box>
			</ModalContent>
		</Modal>
	);
};

export default AppointmentPopup;
