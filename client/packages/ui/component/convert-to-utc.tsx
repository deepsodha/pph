import { utcToZonedTime } from "date-fns-tz";
import { subHours } from "date-fns";
export const convertToUTC = (localDateTime: string): string => {
	const timeZone = "UTC";
	const zonedDate = utcToZonedTime(localDateTime, timeZone);
	const modifiedDate = subHours(zonedDate, 1);
	return modifiedDate.toISOString();
};
