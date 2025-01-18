import { z } from "zod";
import { contactInputSchema } from "./contact-input";
import { addressInputSchema } from "./address-input";
import { govInfoInputSchema } from "./gov-info-input";
import { userInputSchema } from "./user-input";

export const UserUpdateSchema = z.object({
	user: userInputSchema,
	phone: contactInputSchema,
	address: addressInputSchema,
	goverment_info: govInfoInputSchema,
});
