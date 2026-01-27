import { z } from "zod";
import { genderTypeSchema } from ".";

export const socialHistoryAddSchema = z.object({
	patient_id: z.string(),
	birth_gender: genderTypeSchema,
	tobacco: z.array(z.enum(["type1", "type2", "type3", "type4", "type5"])),
	alcohol: z.array(z.enum(["type1", "type2", "type3", "type4", "type5"])),
	cardiovascular: z.array(
		z.enum(["type1", "type2", "type3", "type4", "type5"]),
	),
	sexual_activity: z.array(
		z.enum(["type1", "type2", "type3", "type4", "type5"]),
	),
	drug_abuse: z.array(z.enum(["type1", "type2", "type3", "type4", "type5"])),
	safety: z.array(z.enum(["type1", "type2", "type3", "type4", "type5"])),
	comments: z.string(),
});
