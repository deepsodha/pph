import { useForm } from "@felte/react";
import { validator } from "@felte/validator-zod";
import reporterDom from "@felte/reporter-dom";
import type { z } from "zod";
import { convertToUTC } from "../component/convert-to-utc";
import {
	FormLabel,
	FormControl,
	Input,
	FormErrorMessage,
	Select,
	Textarea,
	Box,
	Center,
} from "@chakra-ui/react";
import { SubmitButton } from "../component";
import type {
	AppointmentAdd,
	ChooseAppointmentType,
	LastUpdatedInput,
	VisitType,
} from "@repo/types/dto";
import { appointmentAddSchema } from "@repo/types/validation";
import type {
	AppointmentStateExtend,
	DoctorStateExtend,
	PatientStateExtend,
	StaffStateExtend,
} from "@repo/types/dexie-state";
import { convertUTCtoLocal } from "../component/utc-date-to-normal-date";
import type { ChangeEvent } from "react";
import DateComponent from "../component/date-comp";

export type AddAppointmentFormProps = {
	onSubmit: (p: AppointmentAdd) => void;
	docters: DoctorStateExtend[] | undefined;
	patient: PatientStateExtend[] | undefined;
	staff: StaffStateExtend[] | undefined;
	appointmentId?: string;
	edit?: boolean;
	initialValues?: AppointmentStateExtend | undefined;
	lastUpdatedInput: () => Promise<LastUpdatedInput>;
};

export const AppointmentForm = (props: AddAppointmentFormProps) => {
	const staffInitialValue = props.initialValues?.staff_id
		? `${props.initialValues.staff_id}:${props.initialValues.staff_name}`
		: "";
	const doctorInitialValue = props.initialValues?.doctor_id
		? `${props.initialValues.doctor_id}:${props.initialValues.doctor_name}`
		: "";

	const patientInitialValue = props.initialValues?.patient_id
		? `${props.initialValues.patient_id}:${props.initialValues.patient_name}`
		: "";

	if (props.initialValues === undefined && props.edit === true) {
		return <p>Loading...</p>;
	}
	function handleChangeForPatient(e: ChangeEvent<HTMLSelectElement>) {
		const [patientId = "", patient_name = ""] = e.target.value.split(":");
		setData("patient_id", patientId);
		setData("patient_name", patient_name);
	}
	function handleChangeForDoctor(e: ChangeEvent<HTMLSelectElement>) {
		const [doctorId = "", doctorName = ""] = e.target.value.split(":");
		setData("doctor_id", doctorId);
		setData("doctor_name", doctorName);
	}
	function handleChangeForStaff(e: ChangeEvent<HTMLSelectElement>) {
		const [staffId = "", staffName = ""] = e.target.value.split(":");
		setData("staff_id", staffId);
		setData("staff_name", staffName);
	}
	const { form, errors, isDirty, isSubmitting, setData } = useForm<
		z.infer<typeof appointmentAddSchema>
	>({
		onSubmit: async (values) => {
			const utcDate = convertToUTC(values.date);
			const modifiedValues = {
				...values,
				last_updated_input: await props.lastUpdatedInput(),
			};
			modifiedValues.date = utcDate;

			props.onSubmit(modifiedValues);
		},

		initialValues: {
			patient_id: props.initialValues?.patient_id || "",
			doctor_id: props.initialValues?.doctor_id || "",
			doctor_name: props.initialValues?.doctor_name || "",
			patient_name: props.initialValues?.patient_name,
			visit: (props.initialValues?.visit as VisitType) || "",
			date:
				(props.initialValues?.date &&
					convertUTCtoLocal(props.initialValues.date)) ||
				"",
			appointment_duration: props.initialValues?.appointment_duration,
			choose_appointment:
				(props.initialValues?.choose_appointment as ChooseAppointmentType) ||
				"",
			note: props.initialValues?.note || "",
			room_and_equipment_no: props.initialValues?.room_and_equipment_no,
			staff_id: props.initialValues?.staff_id || "",
			staff_name: props.initialValues?.staff_name || "",
		},
		extend: [validator({ schema: appointmentAddSchema }), reporterDom()],
	});
	const today = new Date().toISOString().slice(0, 16);

	return (
		<Center>
			<Box width={{ md: "80%", base: "90%", lg: "70%" }}>
				<form ref={form}>
					<FormLabel mb={2} my={3} color={"#095FBA"}>
						Doctor Name
						<span style={{ color: "red" }}>*</span>
					</FormLabel>
					<FormControl
						mt={2}
						isInvalid={(errors().doctor_name || []).length !== 0}
					>
						<Select
							placeholder="Doctor Name"
							name="doctor_name"
							bgColor="#FFFFFF"
							borderColor="#095FBA"
							onChange={(e) => handleChangeForDoctor(e)}
							value={isDirty() ? undefined : doctorInitialValue}
						>
							{props.docters?.map((items) => (
								<option
									key={items.id}
									value={`${items.id}:${items.user.user.first_name}`}
								>
									{items.user.user.first_name}
								</option>
							))}
						</Select>
						{errors().doctor_name && (
							<FormErrorMessage>{errors().doctor_name}</FormErrorMessage>
						)}
					</FormControl>

					<FormLabel mb={2} my={3} color={"#095FBA"}>
						Patient Name
						<span style={{ color: "red" }}>*</span>
					</FormLabel>

					<FormControl
						mt={2}
						isInvalid={(errors().patient_name || []).length !== 0}
					>
						<Select
							placeholder="Patient Name"
							name="patient_name"
							bgColor="#FFFFFF"
							borderColor="#095FBA"
							value={isDirty() ? undefined : patientInitialValue}
							onChange={(e) => handleChangeForPatient(e)}
						>
							{props.patient?.map((items) => (
								<option
									key={items.id}
									value={`${items.id}:${items.user.first_name}`}
								>
									{items.user.first_name}
								</option>
							))}
						</Select>
						{errors().patient_name && (
							<FormErrorMessage>{errors().patient_name}</FormErrorMessage>
						)}
					</FormControl>

					<FormLabel mb={2} my={3} color={"#095FBA"}>
						Visit
						<span style={{ color: "red" }}>*</span>
					</FormLabel>
					<FormControl mt={2} isInvalid={(errors().visit || []).length !== 0}>
						<Select
							placeholder="Select Visit"
							name="visit"
							bgColor="#FFFFFF"
							borderColor="#095FBA"
						>
							<option value="SickVisit">SickVisit</option>
							<option value="RegularVisit">RegularVisit</option>
						</Select>
						{errors().visit && (
							<FormErrorMessage>{errors().visit}</FormErrorMessage>
						)}
					</FormControl>

					<FormLabel mb={2} my={3} color={"#095FBA"}>
						Date
						<span style={{ color: "red" }}>*</span>
					</FormLabel>
					<FormControl mt={2} isInvalid={(errors().date || []).length !== 0}>
						<DateComponent
							type={"datetime-local"}
							name={"date"}
							placeholder={"date"}
							min={today}
						/>
						{errors().date && (
							<FormErrorMessage>{errors().date}</FormErrorMessage>
						)}
					</FormControl>

					<FormLabel mb={2} my={3} color={"#095FBA"}>
						Appointment Duration (in minutes)
						<span style={{ color: "red" }}>*</span>
					</FormLabel>
					<FormControl
						mt={2}
						isInvalid={(errors().appointment_duration || []).length !== 0}
					>
						<Input
							type="number"
							bgColor="#FFFFFF"
							name="appointment_duration"
							borderColor="#095FBA"
							placeholder="Appointment Duration (in minutes)"
							min={10}
							max={60}
						/>
						{errors().appointment_duration && (
							<FormErrorMessage>
								{errors().appointment_duration}
							</FormErrorMessage>
						)}
					</FormControl>

					<FormLabel mb={2} my={3} color={"#095FBA"}>
						Choose Appointment
						<span style={{ color: "red" }}>*</span>
					</FormLabel>
					<FormControl
						mt={2}
						isInvalid={(errors().choose_appointment || []).length !== 0}
					>
						<Select
							placeholder="Choose Appointment"
							name="choose_appointment"
							bgColor="#FFFFFF"
							borderColor="#095FBA"
						>
							<option value="Weekly">Weekly</option>
							<option value="Monthly">Monthly</option>
							<option value="Quarterly">Quarterly</option>
							<option value="Yearly">Yearly</option>
						</Select>
						{errors().choose_appointment && (
							<FormErrorMessage>{errors().choose_appointment}</FormErrorMessage>
						)}
					</FormControl>

					<FormLabel mb={2} my={3} color={"#095FBA"}>
						Note
						<span style={{ color: "red" }}>*</span>
					</FormLabel>
					<FormControl mt={2} isInvalid={(errors().note || []).length !== 0}>
						<Textarea
							bgColor="#FFFFFF"
							name="note"
							borderColor="#095FBA"
							placeholder="Note"
						/>
						{errors().note && (
							<FormErrorMessage>{errors().note}</FormErrorMessage>
						)}
					</FormControl>

					<FormLabel mb={2} my={3} color={"#095FBA"}>
						Room and Equipment No
						<span style={{ color: "red" }}>*</span>
					</FormLabel>
					<FormControl
						mt={2}
						isInvalid={(errors().room_and_equipment_no || []).length !== 0}
					>
						<Select
							placeholder="Room and Equipment No"
							name="room_and_equipment_no"
							bgColor="#FFFFFF"
							borderColor="#095FBA"
						>
							<option value="Room 1">Room 1</option>
							<option value="Room 2">Room 2</option>
							<option value="Room 3">Room 3</option>
							<option value="Room 4">Room 4</option>
						</Select>
						{errors().room_and_equipment_no && (
							<FormErrorMessage>
								{errors().room_and_equipment_no}
							</FormErrorMessage>
						)}
					</FormControl>

					<FormLabel mb={2} my={3} color={"#095FBA"}>
						Staff
						<span style={{ color: "red" }}>*</span>
					</FormLabel>
					<FormControl
						mt={2}
						isInvalid={(errors().staff_name || []).length !== 0}
					>
						<Select
							placeholder="Select Staff"
							name="staff_name"
							bgColor="#FFFFFF"
							borderColor="#095FBA"
							onChange={(e) => handleChangeForStaff(e)}
							value={isDirty() ? undefined : staffInitialValue}
						>
							{props.staff?.map((items) => (
								<option
									key={items.id}
									value={`${items.id}:${items.user.user.first_name}`}
								>
									{items.user.user.first_name}
								</option>
							))}
						</Select>
						{errors().staff_name && (
							<FormErrorMessage>{errors().staff_name}</FormErrorMessage>
						)}
					</FormControl>

					<SubmitButton loading={isSubmitting()} />
				</form>
			</Box>
		</Center>
	);
};
