import { z } from "zod";

export const authUserSchema = z.object({
	first_name: z.string(),
	middle_name: z.string().nullable(),
	last_name: z.string(),
	email: z.string(),
	photo_url: z.string(),
	gender: z.string(),
	date_of_birth: z.string(),
	password_hash: z.string(),
});
