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

type HistoricalPopupProps = {
	open: boolean;
	close: () => void;
	id: string;
	patientId: string;
};

const HistoricalPopup = ({ open, close, id }: HistoricalPopupProps) => {
	const historical_id = id;

	const historicalData = useLiveQuery(() => db.historical.toArray());

	const historicalDataById = historicalData?.find(
		(item) => item.id === historical_id,
	);

	return (
		<Modal isOpen={open} onClose={close} motionPreset={"slideInBottom"}>
			<ModalOverlay />
			<ModalContent ml={5} mr={5}>
				<ModalHeader textAlign="center">Historical Details</ModalHeader>
				<ModalCloseButton />
				<Box margin="1%">
					<>
						<TextField
							heading={"Vaccine:"}
							result={historicalDataById?.vaccine}
						/>
						<TextField heading={"Type:"} result={historicalDataById?.types} />
						<TextField heading={"Date:"} result={historicalDataById?.date} />
						<TextField
							heading={"Number in Series:"}
							result={historicalDataById?.number_in_series}
						/>
						<TextField
							heading={"Provider:"}
							result={historicalDataById?.provider}
						/>
						<TextField
							heading={"Source of Information:"}
							result={historicalDataById?.source_of_information}
						/>
						<TextField
							heading={"Comments:"}
							result={historicalDataById?.comments}
						/>
					</>
				</Box>
			</ModalContent>
		</Modal>
	);
};

export default HistoricalPopup;
