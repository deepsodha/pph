import { z } from "zod";
import { addressInputSchema } from "./address-input";

export const organizationSchema = z.object({
	id: z.string(),
	name: z.string(),
	details: z.string(),
	phone_number: z.string(),
	email: z.string(),
	address: addressInputSchema,
});
