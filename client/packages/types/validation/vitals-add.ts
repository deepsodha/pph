import { z } from "zod";

export const vitalsAddSchema = z.object({
	patient_id: z.string(),
	date_time: z.string().nonempty(),
	blood_pressure: z.number(),
	heart_rate: z.number(),
	height: z.number(),
	weight: z.number(),
	temperature: z.number(),
	bmi: z.number(),
	comments: z.string(),
});
