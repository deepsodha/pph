import { z } from "zod";
import { statusSchema } from "./status";

export const medicationsAddSchema = z.object({
	status: statusSchema,
	patient_id: z.string().nonempty(),
	drug: z.string(),
	instruction: z.string().nonempty(),
	comments: z.string(),
});
