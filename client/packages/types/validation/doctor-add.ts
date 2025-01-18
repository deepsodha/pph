import { z } from "zod";
import { doctorTypeSchema } from "./doctor-type";

export const doctorInputSchema = z.object({
	user_id: z.string().nonempty(),
	doctor_role: z
		.array(doctorTypeSchema)
		.refine((data) => data.includes("Cardiologist"), {
			message: "Cardiologist role is required",
			path: ["doctor_role"],
		}),
	docter_register_number: z.string().nonempty(),
	doctor_department: z.string().nonempty(),
	doctor_speciality: z.string().nonempty(),
	emergency: z.boolean(),
});
