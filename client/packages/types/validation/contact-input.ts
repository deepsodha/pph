import { z } from "zod";
import { phoneNoTypeSchema } from "./phone-no-type";

export const contactInputSchema = z.object({
	number: z
		.string()
		.min(10, { message: "Please enter valid mobile number" })
		.max(10, { message: "Please enter valid mobile number" }),
	number_type: phoneNoTypeSchema,
});
