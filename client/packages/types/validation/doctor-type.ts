import { z } from "zod";

export const doctorTypeSchema = z.enum([
	"FamilyMedicinePhysician",
	"Pediatrician",
	"Gynecologist",
	"Cardiologist",
	"Pharmacist",
	"Dermatologist",
	"Psychiatrist",
	"Surgeon",
]);
