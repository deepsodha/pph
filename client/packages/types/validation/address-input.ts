import { z } from "zod";

export const addressInputSchema = z.object({
	pin_code: z.string().min(6, { message: "Please enter valid pincode" }).max(6),
	city: z.string().superRefine((value, ctx) => {
		if (!value) {
			ctx.addIssue({
				code: z.ZodIssueCode.custom,
				message: "Area is required",
			});
		}
	}),
	state: z.string().superRefine((value, ctx) => {
		if (!value) {
			ctx.addIssue({
				code: z.ZodIssueCode.custom,
				message: "State is required",
			});
		}
	}),
	address_line: z.string().nonempty(),
	country: z.string().superRefine((value, ctx) => {
		if (!value) {
			ctx.addIssue({
				code: z.ZodIssueCode.custom,
				message: "Country is required",
			});
		}
	}),
});
