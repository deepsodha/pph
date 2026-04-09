import { z } from "zod";
export const Administered = z.enum(["Mild", "Moderate", "Severe"]);
