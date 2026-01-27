import { z } from "zod";

export const clientaddSchema = z.object({
	name: z.string().nonempty(),
	address: z.string().nonempty(),
	mobile_number: z.string().nonempty(),
	email: z.string().nonempty(),
	gst_no: z.string().nonempty(),
	pan_no: z.string().nonempty(),
});
