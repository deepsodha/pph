import { z } from "zod";

export const staffTypeSchema = z.enum([
	"Consultation",
	"Examination",
	"Diagnosis",
	"Treatment",
	"Procedure",
	"Surgery",
	"Therapy",
	"Counseling",
	"MedicationManagement",
	"Imaging",
	"LaboratoryTest",
	"Rehabilitation",
	"HomeCare",
	"Telemedicine",
	"WellnessProgram",
]);
