import { z } from "zod";
import { doctorTypeSchema } from "./doctor-type";

export const historicalAddSchema = z.object({
	patient_id: z.string().nonempty(),
	vaccine: z.string().nonempty(),
	types: z.string().nonempty(),
	date: z.string().nonempty(),
	number_in_series: z.string().nonempty(),
	provider: doctorTypeSchema,
	source_of_information: z.string().nonempty(),
	comments: z.string(),
});
