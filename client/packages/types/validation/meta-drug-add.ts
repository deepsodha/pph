import { z } from "zod";

export const DrugaddSchema = z.object({
	brand: z.string().nonempty(),
	generic: z.string().nonempty(),
	details: z.string().nonempty(),
	category: z.string().nonempty(),
	side_effects: z.string().nonempty(),
	drugsdose_info: z.string().nonempty(),
	precautions: z.string().nonempty(),
	manufacturer_name: z.string().nonempty(),
	medicines: z.string().nonempty(),
	contra_indications: z.string().nonempty(),
	diseases: z.string().nonempty(),
	interactions: z.string().nonempty(),
	contains: z.string().nonempty(),
});
