import { Box, Button, Center, Image } from "@chakra-ui/react";
import Add from "../../assets/Plus Icon.svg";
import { Link, useParams } from "@tanstack/react-router";
import { UpdateAppointmentFn } from "../../query-mutation-services/appointment-data-fn";
import {
	addValue,
	dashboardValue,
	displayMenu,
	formValue,
	headerText,
} from "../../atoms/header";
import { useAtom } from "jotai";
import { AppointmentForm } from "@repo/ui/forms";
import type { AppointmentAdd, AppointmentUpdate } from "@repo/types/dto";
import { db } from "../../db/db";
import { useLiveQuery } from "dexie-react-hooks";
import { useMountEffect } from "@react-hookz/web";
import { motion, AnimatePresence } from "framer-motion";

const MotionBox = motion(Box);

const AppointmentEditForm = () => {
	const [, setHeaderText] = useAtom(headerText);
	const [, setAddValue] = useAtom(addValue);
	const [, setFormValue] = useAtom(formValue);
	const [, setmenu] = useAtom(displayMenu);
	const [, setDashboardValue] = useAtom(dashboardValue);
	const docters = useLiveQuery(() => db.doctors.toArray());
	const staff = useLiveQuery(() => db.staff.toArray());
	const patient = useLiveQuery(() => db.patients.toArray());
	const appointment = useLiveQuery(() => db.appointments.toArray());
	const updateMutation = UpdateAppointmentFn();
	const appointmentId = useParams({
		from: "/appointment/edit/$appointmentId",
		select: (params) => params.appointmentId,
	});
	const filteredData = (appointment || []).filter(
		(item) => appointmentId && item.id.includes(appointmentId),
	)[0];
	useMountEffect(() => {
		setHeaderText("Edit Appointment");
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
							initial={{ opacity: 0, x: 50 }}
							animate={{ opacity: 1, x: 0 }}
							exit={{ opacity: 0, x: 50 }}
							transition={{ duration: 0.65 }}
							width={"100%"}
						>
							<Link to={"/patient/add"}>
								<Center>
									<Button
										size="md"
										height="48px"
										width={{ md: "80%", base: "90%", lg: "70%" }}
										color={"#095FBA"}
										bgColor={"#F7F7F9"}
										border="2px"
										my={3}
									>
										<Image src={Add} mr={1} /> Add Patient
									</Button>
								</Center>
							</Link>
							<AppointmentForm
								onSubmit={(p) => {
									const editVal: AppointmentUpdate = {
										id: appointmentId,
										appointment: { ...(p as AppointmentAdd) },
									};
									updateMutation.mutateAsync(editVal);
								}}
								appointmentId={appointmentId}
								docters={docters}
								patient={patient}
								staff={staff}
								initialValues={filteredData}
								lastUpdatedInput={db.getLastUpdated}
								edit={true}
							/>
						</MotionBox>
					</Center>
				</Box>
			</AnimatePresence>
		</div>
	);
};

export default AppointmentEditForm;
