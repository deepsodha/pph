import { z } from "zod";

export const healthStautsSchema = z.enum(["Alive", "Deceased", "Unknown"]);
