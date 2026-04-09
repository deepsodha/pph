import ProfileFemale from "../../assets/Girl Icon.svg";
import ProfileMale from "../../assets/Boy Icon.svg";
import { useLiveQuery } from "dexie-react-hooks";
import { useState } from "react";
import { db } from "../../db/db";
import {
	Modal,
	ModalOverlay,
	ModalContent,
	ModalHeader,
	Box,
	Image,
	Text,
	ModalCloseButton,
} from "@chakra-ui/react";
import { convertUTCtoLocalDate } from "../../../../../packages/ui/component/utc-date-to-normal-date";
import TextField from "../../components/text-field";
type UserPopupProps = {
	open: boolean;
	close: () => void;
	id: string;
};
const userPopup = (props: UserPopupProps) => {
	const user_id = props.id;
	const [imageError, setImageError] = useState(false);

	const userData = useLiveQuery(() => db.user.toArray());

	const userDataById = userData?.find((item) => item.id === user_id);
	const chosenProfileImage =
		userDataById?.user.gender === "Female" ? ProfileFemale : ProfileMale;

	return (
		<div>
			<Modal
				isOpen={props.open}
				onClose={props.close}
				motionPreset={"slideInBottom"}
			>
				<ModalOverlay />
				<ModalContent mr={5} ml={5}>
					<ModalHeader textAlign="center">User</ModalHeader>
					<ModalCloseButton />
					<Box maxHeight="80vh" overflowY="auto">
						<Box display="flex" justifyContent="center" alignContent={"center"}>
							<Image
								src={
									imageError ? chosenProfileImage : userDataById?.user.photo_url
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
							<Text fontSize="2xl" my={1}>
								<TextField
									heading={"Name:"}
									result={userDataById?.user.first_name}
								/>
								<TextField
									heading={"Dob:"}
									result={
										userDataById?.user.dob &&
										convertUTCtoLocalDate(userDataById?.user.dob)
									}
								/>
								<TextField
									heading={"Email:"}
									result={userDataById?.user.email}
								/>
								<TextField
									heading={"Gender:"}
									result={userDataById?.user.gender}
								/>
								<TextField
									heading={"Address:"}
									result={userDataById?.address.address_line}
								/>
								<TextField
									heading={"Pin Code:"}
									result={userDataById?.address?.pin_code}
								/>
								<TextField
									heading={"City:"}
									result={userDataById?.address?.city}
								/>

								<TextField
									heading={"Country:"}
									result={userDataById?.address?.country}
								/>
								<TextField
									heading={"Phone Number Type:"}
									result={userDataById?.phone?.number_type}
								/>
								<TextField
									heading={"Phone Number:"}
									result={userDataById?.phone?.number}
								/>
								<TextField
									heading={"Government ID Type:"}
									result={userDataById?.government_info?.id_type}
								/>
								<TextField
									heading={"Government ID No:"}
									result={userDataById?.government_info?.id_no}
								/>
							</Text>
						</Box>
					</Box>
				</ModalContent>
			</Modal>
		</div>
	);
};

export default userPopup;
