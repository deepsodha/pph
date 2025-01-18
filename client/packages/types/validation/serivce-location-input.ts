import z from "zod";
import { addressInputSchema } from "./address-input";
const isTimeGreaterThan = (startTime: string, endTime: string): boolean => {
	const [startHour, startMinute] = startTime.split(":").map(Number);
	const [endHour, endMinute] = endTime.split(":").map(Number);

	const startTimestamp = new Date(0, 0, 0, startHour, startMinute, 0, 0);
	const endTimestamp = new Date(0, 0, 0, endHour, endMinute, 0, 0);

	// // Adjust hours for PM times
	// if (startMeridiem === "PM" && startHour !== "12") startTimestamp.setHours(startTimestamp.getHours() + 12);
	// if (endMeridiem === "PM" && endHour !== "12") endTimestamp.setHours(endTimestamp.getHours() + 12);

	return startTimestamp.getTime() < endTimestamp.getTime();
};
export const serviceLocationInputSchema = z
	.object({
		service_location_name: z.string().nonempty(),
		address: addressInputSchema,
		start_time: z.string().superRefine((val, ctx) => {
			if (!val) {
				ctx.addIssue({
					code: z.ZodIssueCode.custom,
					message: "Enter Start time",
				});
			}
		}),
		end_time: z.string().superRefine((val, ctx) => {
			if (!val) {
				ctx.addIssue({
					code: z.ZodIssueCode.custom,
					message: "Enter end time",
				});
			}
		}),
	})
	.superRefine((data, ctx) => {
		if (!isTimeGreaterThan(data.start_time, data.end_time)) {
			ctx.addIssue({
				path: ["end_time"] || undefined,
				code: z.ZodIssueCode.custom,
				message: "end time must be greater",
			});
		}
	});
