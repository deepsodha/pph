import { z } from "zod";
import { metaroleTypeSchema } from "./metaroleTypeSchema";

export const UserSchema = z.object({
	mobile_number: z.string().nonempty(),
	password: z.string().nonempty(),
	role: z.array(metaroleTypeSchema),
});
