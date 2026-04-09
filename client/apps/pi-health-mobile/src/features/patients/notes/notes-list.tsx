import { Box, Center, Fade, Heading } from "@chakra-ui/react";
import SearchBar from "../../../components/search-bar";
import { db } from "../../../db/db";
import AddPatient from "../../../assets/Add Patient Icon.svg";
import { useMountEffect } from "@react-hookz/web";
import { useAtom } from "jotai";
import { useLiveQuery } from "dexie-react-hooks";
import {
	headerText,
	addImage,
	addPath,
	addValue,
	dashboardValue,
	formValue,
} from "../../../atoms/header";
import { useParams } from "@tanstack/react-router";
import SoapNotesList from "./soap/soap-notes-list";
import HistoryAndPhysicalNotesList from "./history-and-physical/history-and-physical-notes-list";

const NotesList = () => {
	const [, setHeaderText] = useAtom(headerText);
	const [, setaddImage] = useAtom(addImage);
	const [, setaddPath] = useAtom(addPath);
	const [, setAddValue] = useAtom(addValue);
	const [, setFormValue] = useAtom(formValue);
	const [, setDashboardValue] = useAtom(dashboardValue);

	useMountEffect(() => {
		setHeaderText(" Add Notes");
		setaddImage(AddPatient);
		setaddPath(`/notes/add/${patientId}`);
		setAddValue(false);
		setFormValue(false);
		setDashboardValue(false);
	});
	const patientId = useParams({
		from: "/notes/list/$patientId",
		select: (params) => params.patientId,
	});
	const newData = useLiveQuery(() => db.note.toArray())?.filter(
		(item) => item.patient_id === patientId,
	);
	const NotesData = newData?.filter((note) => note);

	console.log(NotesData, "this is a SOAPNote");

	console.log(newData, "this isnew data");
	return (
		<div>
			<Fade in={true}>
				<Box minHeight={"100vh"} bgColor={"#F7F7F9"}>
					<Box
						position="sticky"
						zIndex={10}
						bgColor={"#F7F7F9"}
						top={0}
						left={0}
						right={0}
					>
						<Center>
							<Box width={{ md: "80%", base: "90%", lg: "70%" }}>
								<Heading
									color="#095FBA"
									pt="1"
									fontSize="20px"
									mt="16px"
									mb="10px"
								>
									Notes List
								</Heading>
								<SearchBar placeholder={"Search Notes by Id"} />
							</Box>
						</Center>
					</Box>
					<Box zIndex={15} bgColor={"#F7F7F9"} pb="10%">
						{NotesData?.map((item) => (
							<div key={item.id}>
								<Center>
									<SoapNotesList note={item} />
									<HistoryAndPhysicalNotesList note={item} />
								</Center>
							</div>
						))}
					</Box>
				</Box>
			</Fade>
		</div>
	);
};

export default NotesList;
