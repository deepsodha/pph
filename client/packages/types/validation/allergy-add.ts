import { z } from "zod";
import { AllergySeveritiesType } from "./allergy-severities-type";
import { statusSchema } from "./status";
export const allergyAddSchema = z.object({
	patient_id: z.string(),
	allergen: z.string(),
	reaction: z.string().min(1, { message: "Reactions are required" }),
	allergy_severities: AllergySeveritiesType,
	input_date: z.string().nonempty(),
	status: statusSchema,
	comments: z.string(),
});
