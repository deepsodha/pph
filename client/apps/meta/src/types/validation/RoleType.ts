import { z } from "zod";

export const roleTypeSchema = z.union([
	z.literal("SuperAdmin"),
	z.literal("SystemAdmin"),
]);
