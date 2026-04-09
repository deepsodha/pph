import { z } from "zod";
import { genderTypeSchema } from "./gender-type";

export const userInputSchema = z.object({
	first_name: z.string().nonempty(),
	middle_name: z.string().nonempty(),
	last_name: z.string().nonempty(),
	dob: z
		.string()
		.nonempty()
		.superRefine((value, ctx) => {
			const inputDate = new Date(value);
			const currentDate = new Date();
			if (inputDate >= currentDate) {
				ctx.addIssue({
					code: z.ZodIssueCode.custom,
					message: "Date of Birth cannot be of Future time.",
				});
			}
		}),
	gender: genderTypeSchema,
	email: z
		.string()
		.nonempty()
		.superRefine((val, ctx) => {
			const emailRegex = /^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$/;
			if (!emailRegex.test(val)) {
				ctx.addIssue({
					code: z.ZodIssueCode.custom,
					message: "Enter valid Email. \n",
				});
			}
		}),
	photo_url: z.string().nonempty(),
});
