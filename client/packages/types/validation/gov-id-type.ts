import { z } from "zod";

export const govIdTypeSchema = z.enum([
	"AadhaarCard",
	"DrivingLicense",
	"Passport",
]);
