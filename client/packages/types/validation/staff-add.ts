import { z } from "zod";
import { staffTypeSchema } from "./staff-type";

export const staffInputSchema = z.object({
	user_id: z.string().nonempty(),
	staff_department: z.array(staffTypeSchema),
	emergency: z.boolean(),
});
