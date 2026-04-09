import { z } from "zod";

export const genderTyprSchema = z.enum(["Male", "Female", "Other", "Unknown"]);
