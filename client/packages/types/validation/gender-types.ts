import { z } from "zod";

export const genderTypeSchema = z.enum(["Male", "Female", "Other", "Unknown"]);
