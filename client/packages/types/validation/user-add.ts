import { z } from "zod";
import { contactInputSchema } from "./contact-input";
import { addressInputSchema } from "./address-input";
import { govInfoInputSchema } from "./gov-info-input";
import { userInputSchema } from "./user-input";
import { passwordInputSchema } from "./password-input";

export const UserInputSchema = z
	.object({
		user: userInputSchema,
		phone: contactInputSchema,
		address: addressInputSchema,
		goverment_info: govInfoInputSchema,
		password: passwordInputSchema,
		confirm_password: z.string(),
	})
	.refine((data) => data.password === data.confirm_password, {
		message: "Passwords don't match",
		path: ["confirm_password"],
	});
