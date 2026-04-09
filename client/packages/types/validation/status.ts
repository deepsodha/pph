import { z } from "zod";

export const statusSchema = z.enum(["Active", "InActive", "MarkAsError"]);
