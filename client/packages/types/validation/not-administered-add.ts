import { z } from "zod";
import { typesSchema } from "./types";
export const notAdministeredAddSchema = z.object({
	patient_id: z.string().nonempty(),
	vaccine: z.string().nonempty(),
	types: typesSchema,
	recorded: z.string().nonempty(),
	reason_for_non_administration: z.string().nonempty(),
	comments: z.string(),
});
