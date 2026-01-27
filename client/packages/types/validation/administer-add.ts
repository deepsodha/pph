import { z } from "zod";
import { typesSchema } from "./types";
import { brandSchema } from "./brand";
import { doctorTypeSchema } from "./doctor-type";

export const administerAddSchema = z.object({
	patient_id: z.string(),
	vaccine: z.string().nonempty(),
	types: typesSchema,
	generic: z.string().nonempty(),
	ordered: z.string().nonempty(),
	recorded: z.string().nonempty(),
	dose: z.string().nonempty(),
	site: z.string().nonempty(),
	brand: brandSchema,
	number_of_serial: z.number(),
	lot: z.number(),
	expiration: z.string().nonempty(),
	consent_obtain: z.string().nonempty(),
	administrated_by: z.string().nonempty(),
	provider: doctorTypeSchema,
	vis_date: z.string().nonempty(),
	clinic_location: z.string().nonempty(),
	vfs_financial_class: z.string().nonempty(),
	comments: z.string().nonempty(),
});
