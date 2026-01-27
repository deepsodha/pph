import { z } from "zod";

export const selectOrganizationSchema = z.object({
	id: z.string(),
	name: z.string(),
});
