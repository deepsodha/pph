import { z } from "zod";
import { passwordInputSchema } from "./password-input";

export const loginMobileSchema = z.object({
	mobile_number: z
		.string()
		.min(10, { message: "Please enter valid mobile number" })
		.max(10, { message: "Please enter valid mobile number" }),
	password: passwordInputSchema,
});
