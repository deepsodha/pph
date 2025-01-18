import { z, type ZodCustomIssue } from "zod";
import { govIdTypeSchema } from "./gov-id-type";
import type { GovIdType } from "../dto";

export const govInfoInputSchema = z
	.object({
		id_no: z.string().nonempty(),
		id_type: govIdTypeSchema,
	})
	.refine(
		(data): data is { id_no: string; id_type: GovIdType } => {
			const { id_no, id_type } = data;
			let expectedLength: number;

			switch (id_type) {
				case "AadhaarCard":
					expectedLength = 12;
					break;
				case "Passport":
					expectedLength = 8;
					break;
				case "DrivingLicense":
					expectedLength = 6;
					break;
				default:
					return true;
			}

			return id_no.length === expectedLength;
		},
		(data): ZodCustomIssue => {
			const { id_type } = data;
			let errorMessage: string;

			switch (id_type) {
				case "AadhaarCard":
					errorMessage = "AadhaarCard ID number must be 12 characters long";
					break;
				case "Passport":
					errorMessage = "Passport ID number must be 8 characters long";
					break;
				case "DrivingLicense":
					errorMessage = "DrivingLicense ID number must be 6 characters long";
					break;
				default:
					return {
						code: "custom",
						message: "Invalid ID number length based on government ID type",
						path: ["id_no"],
					};
			}

			return {
				code: "custom",
				message: errorMessage,
				path: ["id_no"],
			};
		},
	);
