import { z } from "zod";
export const vfsTypes = z.enum(["Mild", "Moderate", "Severe"]);
