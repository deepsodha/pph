import { z } from "zod";

export const pastSurgicalHistoryAddScheama = z.object({
	patient_id: z.string(),
	common_surgeries: z.array(z.string()),
	comments: z.string(),
});
