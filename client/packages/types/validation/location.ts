import { z } from "zod";

export const locationSchema = z.enum(["Office", "Home"]);
