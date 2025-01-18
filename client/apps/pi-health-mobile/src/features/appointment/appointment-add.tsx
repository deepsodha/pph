import { Box, Button, Center, Image } from "@chakra-ui/react";
import Add from "../../assets/Plus Icon.svg";
import { Link } from "@tanstack/react-router";
import { AddAppointmentDataFn } from "../../query-mutation-services/appointment-data-fn";
import {
	addValue,
	dashboardValue,
	displayMenu,
	formValue,
	headerText,
} from "../../atoms/header";
import { useAtom } from "jotai";
import { AppointmentForm } from "@repo/ui/forms";
import type { AppointmentAdd } from "@repo/types/dto";
import { db } from "../../db/db";
import { useLiveQuery } from "dexie-react-hooks";
import { useMountEffect } from "@react-hookz/web";
import { motion, AnimatePresence } from "framer-motion";

const MotionBox = motion(Box);

const AppointmentAddForm = () => {
	const [, setHeaderText] = useAtom(headerText);
	const [, setAddValue] = useAtom(addValue);
	const [, setFormValue] = useAtom(formValue);
	const [, setmenu] = useAtom(displayMenu);
	const [, setDashboardValue] = useAtom(dashboardValue);
	const docters = useLiveQuery(() => db.doctors.toArray());
	const staff = useLiveQuery(() => db.staff.toArray());

	const patient = useLiveQuery(() => db.patients.toArray());

	const addMutation = AddAppointmentDataFn();
	useMountEffect(() => {
		setHeaderText("Add New Appointment");
		setAddValue(true);
		setFormValue(true);
		setDashboardValue(false);
		setmenu(false);
	});

	return (
		<div>
			<AnimatePresence>
				<Box bgColor={"#F7F7F9"}>
					<Center>
						<MotionBox
							initial={{ opacity: 0, x: -50 }}
							animate={{ opacity: 1, x: 0 }}
							exit={{ opacity: 0, x: -50 }}
							transition={{ duration: 0.65 }}
							width={"100%"}
						>
							<Link to={"/patient/add"}>
								<Center>
									<Button
										size="md"
										height="48px"
										color={"#095FBA"}
										bgColor={"#F7F7F9"}
										border="2px"
										my={3}
										width={{ md: "80%", base: "90%", lg: "70%" }}
									>
										<Image src={Add} mr={1} /> Add Patient
									</Button>
								</Center>
							</Link>
							<AppointmentForm
								onSubmit={(p) => {
									addMutation.mutateAsync(p as AppointmentAdd);
								}}
								docters={docters}
								patient={patient}
								staff={staff}
								lastUpdatedInput={db.getLastUpdated}
								edit={false}
							/>
						</MotionBox>
					</Center>
				</Box>
			</AnimatePresence>
		</div>
	);
};

export default AppointmentAddForm;
