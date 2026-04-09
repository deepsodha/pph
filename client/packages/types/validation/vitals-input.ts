import { z } from "zod";

export const vitalsInputSchema = z.object({
	doctor_name: z.string().nullable(),
	date_time: z.string(),
	blood_pressure: z.number().nullable(),
	heart_rate: z.string().nullable(),
	height: z.number().nullable(),
	weight: z.string().nullable(),
	temperature: z.string().nullable(),
	bmi: z.string().nullable(),
	comments: z.string().nullable(),
});
