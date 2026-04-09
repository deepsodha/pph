import {
	Box,
	Text,
	Modal,
	ModalCloseButton,
	ModalContent,
	ModalHeader,
	ModalOverlay,
} from "@chakra-ui/react";
import TextField from "../../../components/text-field";
import { useLiveQuery } from "dexie-react-hooks";
import { db } from "../../../db/db";
import { useParams } from "@tanstack/react-router";
import { convertUTCtoLocalDate } from "../../../../../../packages/ui/component/utc-date-to-normal-date";

type ProblempopupProp = {
	open: boolean;
	close: () => void;
	id: string;
};
const Problem_popup = ({ open, close, id }: ProblempopupProp) => {
	const problem_id = id;
	const patientId = useParams({
		from: "/problem/list/$patientId",
		select: (params) => params.patientId,
	});

	const problemData = useLiveQuery(() => db.problems.toArray());
	const headerData = useLiveQuery(() => db.patients.toArray());
	const headerFilterData = headerData?.find((item) => item.id === patientId);
	const problemDataById = problemData?.find((item) => item.id === problem_id);

	return (
		<div>
			<Modal isOpen={open} onClose={close} motionPreset={"slideInBottom"}>
				<ModalOverlay />
				<ModalContent mr={5} ml={5}>
					<ModalHeader textAlign="center">
						{headerFilterData?.user.first_name}’s Problem Details
					</ModalHeader>
					<ModalCloseButton />

					<Box display="flex" justifyContent="Center" />
					<Box margin="1%">
						<>
							<Text
								ml={2}
								mb="2%"
								fontWeight="bold"
								color={problemDataById?.status === "Active" ? "green" : "red"}
							>
								{problemDataById?.status === "Active" ? "Active" : "InActive"}
							</Text>
							<TextField
								heading={"Problem Name:"}
								result={problemDataById?.issue}
							/>
							{/* <TextField
								heading={"ICD-9:"}
								result={problemDataById?.icd_9_problem}
							/> */}
							<TextField
								heading={"ICD-10:"}
								result={problemDataById?.icd_10_problem}
							/>
							<TextField
								heading={"Type:"}
								result={problemDataById?.issue_type}
							/>
							<TextField
								heading={"Start Date:"}
								result={
									problemDataById?.start_date &&
									convertUTCtoLocalDate(problemDataById?.start_date)
								}
							/>
							<TextField
								heading={"End Date:"}
								result={problemDataById?.end_date}
							/>
							<TextField
								heading={"Comments:"}
								result={problemDataById?.comment}
							/>
						</>
					</Box>
				</ModalContent>
			</Modal>
		</div>
	);
};

export default Problem_popup;
