import z from "zod";

export const passwordInputSchema = z.string().superRefine((val, ctx) => {
	const capitalLetterRegex = /[A-Z]/;
	const numericRegex = /[0-9]/;
	const specialCharacterRegex = /[^A-Za-z0-9]/;
	if (!val) {
		ctx.addIssue({
			code: z.ZodIssueCode.custom,
			message: "Enter Password",
		});
	} else if (val.length < 6) {
		ctx.addIssue({
			code: z.ZodIssueCode.custom,
			message: "Password length must be at least  6 characters.",
		});
	} else if (!capitalLetterRegex.test(val)) {
		ctx.addIssue({
			code: z.ZodIssueCode.custom,
			message: "Password must contain at least one capital letter. \n",
		});
	} else if (!numericRegex.test(val)) {
		ctx.addIssue({
			code: z.ZodIssueCode.custom,
			message: "Password must contain at least one numeric digit. \n",
		});
	} else if (!specialCharacterRegex.test(val)) {
		ctx.addIssue({
			code: z.ZodIssueCode.custom,
			message: "Password must contain at least one special character \n",
		});
	}
});
