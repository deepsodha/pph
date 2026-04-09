import { format, utcToZonedTime } from "date-fns-tz";
import { add } from "date-fns";

export function convertUTCtoLocal(utcDateString: string): string {
	const utcDate = new Date(utcDateString);
	const localDate = utcToZonedTime(
		add(utcDate, { hours: 6, minutes: 30 }),
		Intl.DateTimeFormat().resolvedOptions().timeZone,
	);
	return format(localDate, "yyyy-MM-dd HH:mm:ss");
	//return format(localDate, "dd-MM-yyyy");
}

export function convertUTCtoLocalDate(utcDateString: string): string {
	const utcDate = new Date(utcDateString);
	const localDate = utcToZonedTime(
		add(utcDate, { hours: 6, minutes: 30 }),
		Intl.DateTimeFormat().resolvedOptions().timeZone,
	);
	return format(localDate, "dd-MM-yyyy");
}

export function dobDate(utcDateString: string): string {
	const utcDate = new Date(utcDateString);
	const localDate = utcToZonedTime(
		add(utcDate, { hours: 6, minutes: 30 }),
		Intl.DateTimeFormat().resolvedOptions().timeZone,
	);
	return format(localDate, "yyyy-MM-dd");
	//return format(localDate, "dd-MM-yyyy");
}
export function formatedTime(utcDateString: string): string {
	const utcDate = new Date(utcDateString);
	const localDate = utcToZonedTime(
		add(utcDate, { hours: 6, minutes: 30 }),
		Intl.DateTimeFormat().resolvedOptions().timeZone,
	);
	return format(localDate, "HH:mm:ss");
	//return format(localDate, "dd-MM-yyyy");
}
