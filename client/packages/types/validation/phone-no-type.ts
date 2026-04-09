import { z } from "zod";

export const phoneNoTypeSchema = z.enum(["Mobile", "Home", "Office"]);
