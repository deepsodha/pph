import {
	Modal,
	ModalOverlay,
	ModalContent,
	ModalHeader,
	ModalCloseButton,
	Box,
} from "@chakra-ui/react";

import TextField from "../../components/text-field";
import { useLiveQuery } from "dexie-react-hooks";
import { db } from "../../db/db";

type ServiceLocationDetailsPopupProps = {
	open: boolean;
	close: () => void;
	id: string;
};

const ServiceLocationDetailsPopUp = ({
	open,
	close,
	id,
}: ServiceLocationDetailsPopupProps) => {
	const servicelocation_id = id;

	const servicelocationData = useLiveQuery(() => db.servicelocation.toArray());

	const servicelocationDataById = servicelocationData?.find(
		(item) => item.id === servicelocation_id,
	);

	return (
		<Modal isOpen={open} onClose={close} motionPreset={"slideInBottom"}>
			<ModalOverlay />
			<ModalContent mr={5} ml={5}>
				<ModalHeader textAlign="center">Service Location Details</ModalHeader>
				<ModalCloseButton />
				<Box margin="1%">
					<>
						{/* <TextField
								heading={"Name:"}
								// result={data?.result?.patient_details?.first_name}
								result={"test"}
							/> */}
						{/* <TextField
									heading={"Address 2 (optional): "}
									// result={data?.result?.patient_details?.email}
									result={"test"}
								/> */}
						{/* <TextField
									heading={"Zip: "}
									// result={data?.result?.patient_details?.gender}
									result={"test"}
								/> */}
						{/* <TextField
								heading={"Place of service code: "}
								// result={data?.result?.patient_details?.address?.pin_code}
								result={"test"}
							/>
							<TextField
								heading={"Phone No: "}
								// result={data?.result?.patient_details?.address?.city}
								result={"test"}
							/>
							<TextField
								heading={"Fax (optional):"}
								// result={data?.result?.patient_details?.address?.district}
								result={"test"}
							/> */}
						<div>
							<TextField
								heading={"Name:"}
								result={servicelocationDataById?.service_location_name}
								// result={"test"}
							/>
							<TextField
								heading={"Pin Code:"}
								result={servicelocationDataById?.address.pin_code}
							/>
							<TextField
								heading={"Country: "}
								result={servicelocationDataById?.address.country}
							/>
							<TextField
								heading={"State:"}
								result={servicelocationDataById?.address.state}
							/>
							<TextField
								heading={"City:"}
								result={servicelocationDataById?.address.city}
							/>
							<TextField
								heading={"Address Line"}
								result={servicelocationDataById?.address.address_line}
							/>
							<TextField
								heading={"Start Time:"}
								result={servicelocationDataById?.start_time}
							/>
							<TextField
								heading={"End Time:"}
								result={servicelocationDataById?.end_time}
							/>
						</div>
					</>
				</Box>
			</ModalContent>
		</Modal>
	);
};

export default ServiceLocationDetailsPopUp;
