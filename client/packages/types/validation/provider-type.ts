import { z } from "zod";
export const provider = z.enum(["Mild", "Moderate", "Severe"]);
