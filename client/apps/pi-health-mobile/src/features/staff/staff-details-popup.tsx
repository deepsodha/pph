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

type StaffDetailsPopUpProps = {
	open: boolean;
	close: () => void;
	id: string;
};
const StaffDetailsPopUp = ({ open, close, id }: StaffDetailsPopUpProps) => {
	const staff_id = id;
	const [imageError, setImageError] = useState(false);

	const staffDetailData = useLiveQuery(() => db.staff.toArray());

	const staffDetailDataById = staffDetailData?.find(
		(item) => item.id === staff_id,
	);
	const chosenProfileImage =
		staffDetailDataById?.user.user.gender === "Female"
			? ProfileFemale
			: ProfileMale;

	return (
		<Modal isOpen={open} onClose={close} motionPreset={"slideInBottom"}>
			<ModalOverlay />
			<ModalContent mr={5} ml={5}>
				<ModalHeader textAlign="center">Staff Details</ModalHeader>
				<ModalCloseButton />
				<Box maxHeight="80vh" overflowY="auto">
					<Box display="flex" justifyContent="center" alignContent={"center"}>
						<Image
							src={
								imageError
									? chosenProfileImage
									: staffDetailDataById?.user.user.photo_url
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
								result={staffDetailDataById?.user.user.first_name}
							/>
							<TextField
								heading={"Dob:"}
								result={
									staffDetailDataById?.user.user.dob &&
									convertUTCtoLocalDate(staffDetailDataById?.user.user.dob)
								}
							/>
							<TextField
								heading={"Email:"}
								result={staffDetailDataById?.user.user.email}
							/>
							<TextField
								heading={"Gender:"}
								result={staffDetailDataById?.user.user.gender}
							/>
							<TextField
								heading={"Address:"}
								result={staffDetailDataById?.user.address.address_line}
							/>
							<TextField
								heading={"Pin Code:"}
								result={staffDetailDataById?.user.address?.pin_code}
							/>
							<TextField
								heading={"City:"}
								result={staffDetailDataById?.user.address?.city}
							/>

							<TextField
								heading={"Country:"}
								result={staffDetailDataById?.user.address?.country}
							/>
							<TextField
								heading={"Staff Department:"}
								result={staffDetailDataById?.staff_department.join(", ")}
							/>
							<TextField
								heading={"Phone Number Type:"}
								result={staffDetailDataById?.user.phone.number_type}
							/>
							<TextField
								heading={"Phone Number:"}
								result={staffDetailDataById?.user.phone.number}
							/>

							<TextField
								heading={"Government ID Type:"}
								result={staffDetailDataById?.user.government_info?.id_type}
							/>
							<TextField
								heading={"Government ID No:"}
								result={staffDetailDataById?.user.government_info?.id_no}
							/>
							<TextField
								heading={"Emergency:"}
								result={
									staffDetailDataById?.emergency ? "Available" : "Not Available"
								}
							/>
						</div>
					</Box>
				</Box>
			</ModalContent>
		</Modal>
	);
};

export default StaffDetailsPopUp;
