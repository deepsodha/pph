import {
	Box,
	Image,
	Modal,
	ModalCloseButton,
	ModalContent,
	ModalHeader,
	ModalOverlay,
} from "@chakra-ui/react";
import TextField from "../../components/text-field";
import { useLiveQuery } from "dexie-react-hooks";
import { db } from "../../db/db";
import { useState } from "react";
import ProfileFemale from "../../assets/Girl Icon.svg";
import ProfileMale from "../../assets/Boy Icon.svg";
import { convertUTCtoLocalDate } from "../../../../../packages/ui/component/utc-date-to-normal-date";

type DocterDetailsPopUpProps = {
	open: boolean;
	close: () => void;
	id: string;
};
const DocterDetailsPopUp = ({ open, close, id }: DocterDetailsPopUpProps) => {
	const doctorDetail_id = id;
	const [imageError, setImageError] = useState(false);

	const doctorDetailData = useLiveQuery(() => db.doctors.toArray());

	const doctorDetailDataById = doctorDetailData?.find(
		(item) => item.id === doctorDetail_id,
	);
	const chosenProfileImage =
		doctorDetailDataById?.user.user.gender === "Female"
			? ProfileFemale
			: ProfileMale;

	return (
		<Modal isOpen={open} onClose={close} motionPreset={"slideInBottom"}>
			<ModalOverlay />
			<ModalContent mr={5} ml={5}>
				<ModalHeader textAlign="center">Doctor Details</ModalHeader>
				<ModalCloseButton />
				<Box maxHeight="80vh" overflowY="auto">
					<Box display="flex" justifyContent="center" alignContent={"center"}>
						<Image
							src={
								imageError
									? chosenProfileImage
									: doctorDetailDataById?.user.user.photo_url
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
						<div>
							<TextField
								heading={"Name:"}
								result={doctorDetailDataById?.user.user.first_name}
							/>
							<TextField
								heading={"Dob:"}
								result={
									doctorDetailDataById?.user.user.dob &&
									convertUTCtoLocalDate(doctorDetailDataById?.user.user.dob)
								}
							/>
							<TextField
								heading={"Email:"}
								result={doctorDetailDataById?.user.user.email}
							/>
							<TextField
								heading={"Gender:"}
								result={doctorDetailDataById?.user.user.gender}
							/>
							<TextField
								heading={"Address:"}
								result={doctorDetailDataById?.user.address.address_line}
							/>
							<TextField
								heading={"Pin Code:"}
								result={doctorDetailDataById?.user.address?.pin_code}
							/>
							<TextField
								heading={"City:"}
								result={doctorDetailDataById?.user.address?.city}
							/>

							<TextField
								heading={"Country:"}
								result={doctorDetailDataById?.user.address?.country}
							/>
							<TextField
								heading={"Doctor Role:"}
								result={doctorDetailDataById?.doctor_role.join(", ")}
							/>
							<TextField
								heading={"Doctor Registration No:"}
								result={doctorDetailDataById?.user.phone.number}
							/>
							<TextField
								heading={"Doctor Department:"}
								result={doctorDetailDataById?.doctor_department}
							/>
							<TextField
								heading={"Doctor Speciality:"}
								result={doctorDetailDataById?.doctor_speciality}
							/>
							<TextField
								heading={"Phone Number Type:"}
								result={doctorDetailDataById?.user.phone?.number_type}
							/>
							<TextField
								heading={"Phone Number:"}
								result={doctorDetailDataById?.user.phone?.number}
							/>
							<TextField
								heading={"Government ID Type:"}
								result={doctorDetailDataById?.user.government_info?.id_type}
							/>
							<TextField
								heading={"Government ID No:"}
								result={doctorDetailDataById?.user.government_info?.id_no}
							/>
							<TextField
								heading={"Emergency:"}
								result={
									doctorDetailDataById?.emergency
										? "Available"
										: "Not Available"
								}
							/>
						</div>
					</Box>
				</Box>
			</ModalContent>
		</Modal>
	);
};

export default DocterDetailsPopUp;
