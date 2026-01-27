import { z } from "zod";

export const AllergySeveritiesType = z.enum(["Mild", "Moderate", "Severe"]);
