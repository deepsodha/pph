import { z } from "zod";
import { typesSchema } from "./types";

export const orderAddSchema = z.object({
	patient_id: z.string().nonempty(),
	vaccine: z.string().nonempty(),
	types: typesSchema,
	ordered: z.string().nonempty(),
	provider: z.string().nonempty(),
	comments: z.string(),
});
