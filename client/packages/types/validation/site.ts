import { z } from "zod";

export const siteSchema = z.enum(["Site1", "Site2"]);
