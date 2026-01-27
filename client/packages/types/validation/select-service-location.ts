import { z } from "zod";

export const SelectServiceLocationSchema = z.object({
	id: z.string(),
	name: z.string(),
});
