import { z } from "zod";

export const chooseAppointmentTypeSchema = z.enum([
	"Weekly",
	"Monthly",
	"Quarterly",
	"Yearly",
]);
