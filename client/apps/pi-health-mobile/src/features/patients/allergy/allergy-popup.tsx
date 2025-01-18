import {
	Box,
	Modal,
	ModalCloseButton,
	ModalContent,
	ModalHeader,
	ModalOverlay,
	Text,
} from "@chakra-ui/react";
import TextField from "../../../components/text-field";
import { useLiveQuery } from "dexie-react-hooks";
import { db } from "../../../db/db";
import { useParams } from "@tanstack/react-router";
import { convertUTCtoLocalDate } from "../../../../../../packages/ui/component/utc-date-to-normal-date";

type AllergyPopupProps = {
	open: boolean;
	close: () => void;
	id: string;
};

const AllergyPopup = ({ open, close, id }: AllergyPopupProps) => {
	const allergy_id = id;
	const patientId = useParams({
		from: "/allergy/list/$patientId",
		select: (params) => params.patientId,
	});
	const allergyData = useLiveQuery(() => db.allergy.toArray());
	const headerData = useLiveQuery(() => db.patients.toArray());
	const headerFilterData = headerData?.find((item) => item.id === patientId);
	const allergyDataById = allergyData?.find((item) => item.id === allergy_id);

	return (
		<Modal isOpen={open} onClose={close} motionPreset={"slideInBottom"}>
			<ModalOverlay />
			<ModalContent marginRight={5} ml={5}>
				<ModalHeader textAlign="center">
					{headerFilterData?.user.first_name}’s Allergy Details
				</ModalHeader>
				<ModalCloseButton />
				<Box margin="1%">
					<div>
						<Text
							ml={2}
							mb="2%"
							fontWeight="bold"
							color={allergyDataById?.stauts === "Active" ? "green" : "red"}
						>
							{allergyDataById?.stauts === "Active" ? "Active" : "InActive"}
						</Text>
						<TextField
							heading={"Allergen Name:"}
							result={allergyDataById?.allergen}
						/>
						<TextField
							heading={"Reactions:"}
							result={allergyDataById?.reaction}
						/>
						<TextField
							heading={"Allergy Severities:"}
							result={allergyDataById?.allergy_severities}
						/>
						<TextField
							heading={"Date of Onset:"}
							result={
								allergyDataById?.input_date &&
								convertUTCtoLocalDate(allergyDataById?.input_date)
							}
						/>

						<TextField
							heading={"Comments:"}
							result={allergyDataById?.comments}
						/>
					</div>
				</Box>
			</ModalContent>
		</Modal>
	);
};

export default AllergyPopup;
