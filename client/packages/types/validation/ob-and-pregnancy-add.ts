import { z } from "zod";

export const obAndPregnancyAddSchemaa = z.object({
	patient_id: z.string(),
	age_onset_of_menses: z.number(),
	age_at_menopause: z.number(),
	comments_ob: z.string(),
	total_pregnancy: z.number(),
	full_term: z.number(),
	pre_term: z.number(),
	miscarriages: z.number(),
	living: z.number(),
	comments_pregnancy: z.string(),
});
