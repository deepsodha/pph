import {
	Box,
	Modal,
	ModalCloseButton,
	ModalContent,
	ModalHeader,
	ModalOverlay,
	Image,
} from "@chakra-ui/react";
import TextField from "../../components/text-field";
import { useLiveQuery } from "dexie-react-hooks";
import { db } from "../../db/db";
import ProfileFemale from "../../assets/Girl Icon.svg";
import ProfileMale from "../../assets/Boy Icon.svg";
import { useState } from "react";

type SystemAdminPopupProps = {
	open: boolean;
	close: () => void;
	id: string;
};

const SystenAdminPopup = ({ open, close, id }: SystemAdminPopupProps) => {
	const systemadmin_id = id;
	const [imageError, setImageError] = useState(false);

	const systemadminData = useLiveQuery(() => db.systemadmin.toArray());

	const systemadminDataById = systemadminData?.find(
		(item) => item.id === systemadmin_id,
	);
	const chosenProfileImage =
		systemadminDataById?.user.gender === "Female" ? ProfileFemale : ProfileMale;

	return (
		<Modal isOpen={open} onClose={close} motionPreset={"slideInBottom"}>
			<ModalOverlay />
			<ModalContent mr={5} ml={5}>
				<ModalHeader textAlign="center">System Admin Details</ModalHeader>
				<ModalCloseButton />
				<Box display="flex" justifyContent="Center">
					<Image
						src={
							imageError
								? chosenProfileImage
								: systemadminDataById?.user.photo_url
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
					<TextField
						heading={" Name:"}
						result={systemadminDataById?.user.first_name}
					/>
					<TextField heading={"Dob:"} result={systemadminDataById?.user.dob} />
					<TextField
						heading={"Email:"}
						result={systemadminDataById?.user.email}
					/>
					<TextField
						heading={"Gender:"}
						result={systemadminDataById?.user.gender}
					/>
					<TextField
						heading={"Role:"}
						result={systemadminDataById?.roles?.join(", ")}
					/>

					<TextField
						heading={"Phone Number Type:"}
						result={systemadminDataById?.phone.number_type}
					/>
					<TextField
						heading={"Phone Number:"}
						result={systemadminDataById?.phone.number}
					/>
					<TextField
						heading={"Goverment Id Type:"}
						result={systemadminDataById?.government_info.id_type}
					/>
					<TextField
						heading={"Goverment Id No:"}
						result={systemadminDataById?.government_info.id_no}
					/>
				</Box>
			</ModalContent>
		</Modal>
	);
};

export default SystenAdminPopup;
