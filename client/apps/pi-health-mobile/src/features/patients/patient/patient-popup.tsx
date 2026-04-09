import {
	Box,
	Image,
	Modal,
	ModalCloseButton,
	ModalContent,
	ModalHeader,
	ModalOverlay,
} from "@chakra-ui/react";
import TextField from "../../../components/text-field";
import { useLiveQuery } from "dexie-react-hooks";
import { db } from "../../../db/db";
import { useState } from "react";
import ProfileFemale from "../../../assets/Girl Icon.svg";
import ProfileMale from "../../../assets/Boy Icon.svg";
import { convertUTCtoLocalDate } from "../../../../../../packages/ui/component/utc-date-to-normal-date";

type PatientPopupProps = {
	open: boolean;
	close: () => void;
	patientId: string;
};

const PatientPopup = ({ open, close, patientId }: PatientPopupProps) => {
	const patient_id = patientId;
	const [imageError, setImageError] = useState(false);

	const patientData = useLiveQuery(() => db.patients.toArray());

	const patientDataById = patientData?.find((item) => item.id === patient_id);
	const chosenProfileImage =
		patientDataById?.user.gender === "Female" ? ProfileFemale : ProfileMale;
	return (
		<Modal isOpen={open} onClose={close} motionPreset={"slideInBottom"}>
			<ModalOverlay />
			<Box>
				<ModalContent marginRight={5} ml={5}>
					<ModalHeader textAlign="center">Patient Details</ModalHeader>
					<ModalCloseButton />
					<Box display="flex" justifyContent="Center">
						<Image
							src={
								imageError
									? chosenProfileImage
									: patientDataById?.user.photo_url
							}
							marginBottom="20px"
							width="80px"
							height="80px"
							borderRadius="full"
							mt={7}
							mb={7}
							onError={() => setImageError(true)}
						/>
					</Box>
					<Box margin="1%">
						<>
							<TextField
								heading={"Name:"}
								result={patientDataById?.user.first_name}
							/>
							<TextField
								heading={"DOB:"}
								result={
									patientDataById?.user.dob &&
									convertUTCtoLocalDate(patientDataById?.user.dob)
								}
							/>
							<TextField
								heading={"Email:"}
								result={patientDataById?.user.email}
							/>
							<TextField
								heading={"Gender:"}
								result={patientDataById?.user.gender}
							/>
							<TextField
								heading={"Aadharcard Number:"}
								result={patientDataById?.government_info.id_no}
							/>
							<TextField
								heading={"Phone Number:"}
								result={patientDataById?.phone.number}
							/>
							<TextField
								heading={"Address:"}
								result={patientDataById?.address.address_line}
							/>
							<TextField
								heading={"Pin code:"}
								result={patientDataById?.address.pin_code}
							/>
							<TextField
								heading={"City:"}
								result={patientDataById?.address.city}
							/>

							<TextField
								heading={"State:"}
								result={patientDataById?.address.state}
							/>
							<TextField
								heading={"Country:"}
								result={patientDataById?.address.country}
							/>
						</>
					</Box>
				</ModalContent>
			</Box>
		</Modal>
	);
};

export default PatientPopup;
