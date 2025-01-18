import {
	Accordion,
	AccordionItem,
	AccordionButton,
	AccordionPanel,
	Divider,
	Text,
	Box,
} from "@chakra-ui/react";
import { AnimatePresence, motion } from "framer-motion";
import NotesListAllergyComp from "../notes-list-allergy-comp";
import NotesListMedicationComp from "../notes-list-medication-comp";
import NotesListVitalsComp from "../notes-list-vitals-comp";
import type { NoteStateExtend } from "@repo/types/dexie-state";
import type { NoteType, SOAPNote } from "@repo/types/dto";
import NotesListBoldField from "../notes-list-bold-field";

type SoapNotesListProps = {
	note: NoteStateExtend;
};

const SoapNotesList = ({ note }: SoapNotesListProps) => {
	const MotionBox = motion(Box);
	const isSOAPNote = (note: NoteType): note is { SOAPNote: SOAPNote } =>
		"SOAPNote" in note;

	return (
		<>
			{isSOAPNote(note.note) && (
				<AnimatePresence key={note.id}>
					<MotionBox
						key={note.created_at}
						initial={{ opacity: 0, y: -50 }}
						animate={{ opacity: 1, y: 0 }}
						exit={{ opacity: 0, y: -50 }}
						transition={{ duration: 0.65 }}
						width={{ md: "80%", base: "90%", lg: "70%" }}
					>
						<Accordion
							allowToggle
							bgColor={"white"}
							borderRadius="md"
							borderX="1px solid #095FBA"
							borderY={"1px solid #095FBA"}
							mt="25"
						>
							<AccordionItem>
								<h2>
									<AccordionButton>
										<Box flex="1" textAlign="left">
											<Text fontSize={"18px"} color="#095FBA" fontWeight={500}>
												Soap Note{" "}
												<Text>
													(Date:{" "}
													{note.last_updated.toLocaleDateString(undefined, {
														weekday: "short",
														year: "numeric",
														month: "short",
														day: "numeric",
														hour: "numeric",
														minute: "2-digit",
														second: "numeric",
													})}
													)
												</Text>
											</Text>
										</Box>
									</AccordionButton>
								</h2>
								<AccordionPanel pb={4}>
									<NotesListBoldField
										label={"Chief Complaint:"}
										value={note.note.SOAPNote.chief_complaint}
									/>
									<NotesListBoldField
										label={"Assessment:"}
										value={note.note.SOAPNote.assessment}
									/>
									<NotesListBoldField
										label={"Mental or functional:"}
										value={note.note.SOAPNote.mental_or_functional}
									/>
									<Text fontSize={"20px"}>
										<Text fontWeight={"bold"}>Medication:</Text>
										{note.note.SOAPNote.medications.map((note) => (
											<div key={note.id}>
												<NotesListMedicationComp medication={note} />
											</div>
										))}
									</Text>
									<Box mt="7px" mb="7px">
										<Divider border="1px solid" />
									</Box>
									<Text fontSize={"20px"}>
										<Text fontWeight={"bold"}>Allergies:</Text>
										{note.note.SOAPNote.allergies.map((note) => (
											<div key={note.id}>
												<NotesListAllergyComp allergy={note} />
											</div>
										))}
									</Text>
									<Box mt="7px" mb="7px">
										<Divider border="1px solid" />
									</Box>
									<Text fontSize={"20px"}>
										<Text fontWeight={"bold"}>Vitals:</Text>
										<NotesListVitalsComp vitals={note.note.SOAPNote.vitals} />
									</Text>
									<Box mt="7px" mb="7px">
										<Divider border="1px solid" />
									</Box>
									<NotesListBoldField
										label={"Plan:"}
										value={note.note.SOAPNote.plan}
									/>
									<NotesListBoldField
										label={"Objective:"}
										value={note.note.SOAPNote.objective}
									/>
									<NotesListBoldField
										label={"Subjective:"}
										value={note.note.SOAPNote.subjective}
									/>
								</AccordionPanel>
							</AccordionItem>
						</Accordion>
					</MotionBox>
				</AnimatePresence>
			)}
		</>
	);
};

export default SoapNotesList;
