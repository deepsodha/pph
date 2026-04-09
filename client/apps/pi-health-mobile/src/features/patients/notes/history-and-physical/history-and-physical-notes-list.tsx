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
import type { NoteType, HistoryAndPhysical } from "@repo/types/dto";
import type { NoteStateExtend } from "@repo/types/dexie-state";
import NotesListBoldField from "../notes-list-bold-field";

type HistoryAndPhysicalNotesListProps = {
	note: NoteStateExtend;
};

const HistoryAndPhysicalNotesList = ({
	note,
}: HistoryAndPhysicalNotesListProps) => {
	const MotionBox = motion(Box);

	const isHistoryAndPhysical = (
		note: NoteType,
	): note is { HistoryAndPhysical: HistoryAndPhysical } =>
		"HistoryAndPhysical" in note;
	return (
		<>
			{isHistoryAndPhysical(note.note) && (
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
							mt={5}
							borderRadius="md"
							borderX="1px solid green"
							borderY={"1px solid green"}
						>
							<AccordionItem>
								<h2>
									<AccordionButton>
										<Box as="span" flex="1" textAlign="left">
											<Text fontSize={"18px"} color="green" fontWeight={500}>
												History And Physical Note{" "}
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
										value={note.note.HistoryAndPhysical.chief_complaint}
									/>
									<NotesListBoldField
										label={"Health Concerns:"}
										value={note.note.HistoryAndPhysical.health_concerns}
									/>
									<NotesListBoldField
										label={"Goals:"}
										value={note.note.HistoryAndPhysical.goals}
									/>
									<NotesListBoldField
										label={"Assessment:"}
										value={note.note.HistoryAndPhysical.exam}
									/>
									<NotesListBoldField
										label={"Exam:"}
										value={note.note.HistoryAndPhysical.assessment}
									/>
									<NotesListBoldField
										label={"Plan:"}
										value={note.note.HistoryAndPhysical.plan}
									/>
									<NotesListBoldField
										label={"Family History:"}
										value={note.note.HistoryAndPhysical.family_history}
									/>
									<NotesListBoldField
										label={"History of Present Illness:"}
										value={
											note.note.HistoryAndPhysical.history_of_present_illness
										}
									/>
									<NotesListBoldField
										label={"Mental or functional:"}
										value={note.note.HistoryAndPhysical.mental_or_functional}
									/>
									<NotesListBoldField
										label={"Hospitalizations:"}
										value={note.note.HistoryAndPhysical.hospitalizations}
									/>
									<Box mt="7px" mb="7px">
										<Divider border="1px solid" />
									</Box>
									<Text fontSize={"20px"}>
										<Text fontWeight={"bold"}>Medication:</Text>
										{note.note.HistoryAndPhysical.medications.map((note) => (
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
										{note.note.HistoryAndPhysical.allergies.map((note) => (
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
										<NotesListVitalsComp
											vitals={note.note.HistoryAndPhysical.vitals}
										/>
									</Text>
									<Box mt="7px" mb="7px">
										<Divider border="1px solid" />
									</Box>
									<NotesListBoldField
										label={"Hospitalizations:"}
										value={note.note.HistoryAndPhysical.hospitalizations}
									/>
									<NotesListBoldField
										label={"Implantable Devices:"}
										value={note.note.HistoryAndPhysical.implantable_devices}
									/>
									<NotesListBoldField
										label={"Past Medical History:"}
										value={note.note.HistoryAndPhysical.past_medical_history}
									/>
									<NotesListBoldField
										label={"Past Surgical:"}
										value={note.note.HistoryAndPhysical.past_surgical_history}
									/>
									<NotesListBoldField
										label={"Social History:"}
										value={note.note.HistoryAndPhysical.social_history}
									/>
									<NotesListBoldField
										label={"Obstetric and Pregnancy History:"}
										value={
											note.note.HistoryAndPhysical
												.obstetric_and_pregnancy_history
										}
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

export default HistoryAndPhysicalNotesList;
