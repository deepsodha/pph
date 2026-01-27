import { z } from "zod";

export const soapAddSchema = z.object({
	patient_id: z.string(),
	chief_complaint: z.string(),
	subjective: z.string(),
	medications_id: z.string(),
	allergies_id: z.string(),
	mental_or_functional: z.string(),
	vitals_id: z.string(),
	objective: z.string(),
	assessment: z.string(),
	plan: z.string(),
});
