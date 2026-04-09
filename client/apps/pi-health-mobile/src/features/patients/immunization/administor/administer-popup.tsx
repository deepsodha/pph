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

type AdministerPopupProps = {
	open: boolean;
	close: () => void;
	id: string;
	patientId: string;
};

const AdministerPopup = ({ open, close, id }: AdministerPopupProps) => {
	const administer_id = id;

	const administerData = useLiveQuery(() => db.administer.toArray());

	const administerDataById = administerData?.find(
		(item) => item.id === administer_id,
	);

	return (
		<Box>
			<Modal isOpen={open} onClose={close} motionPreset={"slideInBottom"}>
				<ModalOverlay />
				<ModalContent ml={5} mr={5}>
					<ModalHeader textAlign="center">New Administer Details</ModalHeader>
					<ModalCloseButton />
					<Box margin="1%">
						<Box>
							<TextField
								heading={"Vaccine:"}
								result={administerDataById?.vaccine}
							/>
							<TextField heading={"Type:"} result={administerDataById?.types} />
							<TextField
								heading={"Brand:"}
								result={administerDataById?.brand}
							/>
							<TextField
								heading={"Generic:"}
								result={administerDataById?.generic}
							/>
							<TextField
								heading={"Ordered:"}
								result={administerDataById?.ordered}
							/>
							<TextField
								heading={"Recorded:"}
								result={administerDataById?.recorded}
							/>
							<TextField heading={"Dose:"} result={administerDataById?.dose} />
							<TextField heading={"Site:"} result={administerDataById?.site} />
							<TextField
								heading={"Number in Series:"}
								result={administerDataById?.number_of_series}
							/>
							<TextField heading={"Lot:"} result={administerDataById?.lot} />
							<TextField
								heading={"Expiration:"}
								result={administerDataById?.expiration}
							/>
							<TextField
								heading={"Consent Obtained:"}
								result={administerDataById?.consent_obtain}
							/>
							<TextField
								heading={"Administered By:"}
								result={administerDataById?.administrated_by}
							/>
							<TextField
								heading={"Service Location:"}
								result={administerDataById?.clinic_location}
							/>
							<TextField
								heading={"Provider:"}
								result={administerDataById?.provider}
							/>
							<TextField
								heading={"VIS Date:"}
								result={administerDataById?.vis_date}
							/>
							<TextField
								heading={"VFC Financial Class:"}
								result={administerDataById?.vfs_financial_class}
							/>
							<TextField
								heading={"Comments:"}
								result={administerDataById?.comments}
							/>
						</Box>
					</Box>
				</ModalContent>
			</Modal>
		</Box>
	);
};
export default AdministerPopup;
