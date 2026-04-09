import { z } from "zod";

export const implanatableDevicesAddSchema = z.object({
	patient_id: z.string(),
	status: z.string().nonempty(),
	udi: z.string().nonempty(),
	udi_unknown: z.boolean(),
	comments: z.string(),
});
