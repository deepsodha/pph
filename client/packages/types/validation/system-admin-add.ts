import { z } from "zod";
import { userInputSchema } from "./user-input";
import { roleTypeSchema } from "./role-type";
import { govInfoInputSchema } from "./gov-info-input";
import { contactInputSchema } from "./contact-input";
import { passwordInputSchema } from "./password-input";
import { organizationAddSchema } from ".";
export const systemAdminAddSchema = z
	.object({
		org_details: organizationAddSchema,
		user: userInputSchema,
		roles: z.array(roleTypeSchema),
		phone: contactInputSchema,
		government_info: govInfoInputSchema,
		password: passwordInputSchema,
		confirm_password: z.string().min(6),
	})
	.refine((data) => data.password === data.confirm_password, {
		message: "Passwords don't match",
		path: ["confirm_password"],
	});
