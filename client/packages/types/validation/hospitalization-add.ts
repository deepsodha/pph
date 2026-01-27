import { z } from "zod";

export const hospitalizationAddSchema = z.object({
	patient_id: z.string().nonempty(),
	admission_date: z.string().nonempty(),
	related_to: z.string().nonempty(),
	status: z.string().nonempty(),
	length_of_stay: z.number().nullable(),
	procedure: z.string().nonempty(),
	comments: z.string(),
});
