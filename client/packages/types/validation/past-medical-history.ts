import { z } from "zod";

export const pastMedicalHistoryAddSchema = z.object({
	patient_id: z.string().nonempty(),
	blood_type: z.string().nonempty(),
	head: z.string().nonempty(),
	respiratory: z.string().nonempty(),
	musculoskeletal: z.string().nonempty(),
	endocrine: z.string().nonempty(),
	eyes: z.string().nonempty(),
	gastrointestinal: z.string().nonempty(),
	skin: z.string().nonempty(),
	ears: z.string().nonempty(),
	noses: z.string().nonempty(),
	neurological: z.string().nonempty(),
	heme: z.string().nonempty(),
	mouth: z.string().nonempty(),
	infectious: z.string().nonempty(),
	cardiovascular: z.string().nonempty(),
	genitourinary: z.string().nonempty(),
	psychiatric: z.string().nonempty(),
	comments: z.string().nonempty(),
});
