import { z } from "zod";

export const roleTypeSchema = z.enum([
	"Patient",
	"SystemAdmin",
	"Doctor",
	"ClinicalAssistant",
	"OfficeStaff",
	"Biller",
	"BusinessManager",
]);
