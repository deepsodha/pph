import { parseISO } from "date-fns";
import type { ImplantableDevicesState } from "../dto";

export type ImplantableDevicesStateExtend = Omit<
	ImplantableDevicesState,
	"last_updated"
> & {
	last_updated: Date;
};

export const mapImplantableDevices = (
	ImplantableDevicesStateExtend: ImplantableDevicesState,
): ImplantableDevicesStateExtend => {
	return {
		...ImplantableDevicesStateExtend,
		last_updated: parseISO(ImplantableDevicesStateExtend.last_updated),
	};
};
