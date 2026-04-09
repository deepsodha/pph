import { z } from "zod";
import { visitTypeSchema } from "./visit-type";
import { chooseAppointmentTypeSchema } from "./choose-appointment-type";
export const appointmentAddSchema = z.object({
	patient_id: z.string(),
	doctor_id: z.string(),
	doctor_name: z.string().nullable(),
	patient_name: z.string(),
	visit: visitTypeSchema,
	date: z.string(),
	appointment_duration: z
		.number()
		.min(10, { message: "Please enter a duration of minimum 10 minutes" })
		.max(60, { message: "Please enter a duration of maximum 60 minutes" }),
	choose_appointment: chooseAppointmentTypeSchema,
	note: z.string(),
	room_and_equipment_no: z.string(),
	staff_id: z.string(),
	staff_name: z.string(),
});
