import { z } from "zod";
import { userInputSchema } from "./user-input";
import { addressInputSchema } from "./address-input";
import { contactInputSchema } from "./contact-input";
import { govInfoInputSchema } from "./gov-info-input";

export const patientInputSchema = z.object({
	user: userInputSchema,
	address: addressInputSchema,
	phone: contactInputSchema,
	government_info: govInfoInputSchema,
});
