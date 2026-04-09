import { z } from "zod";
import { visitTypeSchema } from "./visit-type";
import { chooseAppointmentTypeSchema } from "./choose-appointment-type";

export const appointmentInputSchema = z.object({
	doctor_name: z.string().nonempty(),
	visit: visitTypeSchema,
	patient_name: z.string().nonempty(),
	date: z.string().nonempty(),
	choose_appointment: chooseAppointmentTypeSchema,
	appointment_duration: z.number().nonnegative(),
	note: z.string().nonempty(),
	room_and_equipment_no: z.string().nonempty(),
	staff_id: z.string().nonempty(),
	staff_name: z.string().nonempty(),
});
