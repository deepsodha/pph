import { z } from "zod";
export const consentType = z.enum(["Verbal", "Written"]);
