import { z } from "zod";
import { addressInputSchema } from "./address-input";

export const organizationAddSchema = z.object({
	name: z.string().nonempty(),
	details: z.string().nonempty(),
	phone_number: z
		.string()
		.min(10, { message: "Please enter valid mobile number" })
		.max(10, { message: "Please enter valid mobile number" }),
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
	address: addressInputSchema,
});
