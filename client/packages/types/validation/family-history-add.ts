import { z } from "zod";
import { healthStautsSchema } from "./healh-status";

export const familyHistoryAddSchema = z.object({
	patient_id: z.string().nonempty().nonempty(),
	family_member: z.string().nonempty(),
	health_status: healthStautsSchema,
});
