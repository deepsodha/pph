import { z } from "zod";
import { statusSchema } from "./status";

export const problemsAddSchema = z.object({
	patient_id: z.string(),
	status: statusSchema,
	issue: z
		.string()
		.min(1, { message: "Problem/Issue Name is required" })
		.max(255, {
			message: "Problem/Issue Name should be at most 255 characters long",
		}),
	//icd_9_problem: z.string(),
	icd_10_problem: z.string(),
	issue_type: z.enum(["Acute", "Chronic"]),
	start_date: z.string().min(2, { message: "Enter start date" }),
	end_date: z.string().min(2, { message: "Enter end date" }),
	comment: z.string(),
});
