import { z } from "zod";

export const metaroleTypeSchema = z.union([
	z.literal("SuperAdmin"),
	z.literal("SystemAdmin"),
]);
