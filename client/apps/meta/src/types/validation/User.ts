import { z } from "zod";
import { roleTypeSchema } from "./RoleType";

export const usersSchema = z.object({
	mobile_number: z.string(),
	password: z.string(),
	role: z.array(roleTypeSchema),
});
