import { z } from "zod";

export const visitTypeSchema = z.enum(["SickVisit", "RegularVisit"]);
