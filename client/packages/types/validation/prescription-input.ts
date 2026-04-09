import { z } from "zod";

export const prescriptionInputSchema = z.object({
	patient_id: z.string().nonempty(),
	patient_name: z.string().nonempty(),
	presecription_name: z.string().nonempty(),
	instruction: z.string().nonempty(),
	date: z.string().nonempty(),
	drug_name: z.array(z.string()),
});
