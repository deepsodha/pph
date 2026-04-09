import { z } from "zod";

export const DoctorSchema = z.object({
	doctor_name: z.string().nonempty(),
	speciality: z.string().nonempty(),
	experience: z.string().nonempty(),
	hospital_address: z.string().nonempty(),
	city: z.string().nonempty(),
	pincode: z.string().nonempty(),
});
