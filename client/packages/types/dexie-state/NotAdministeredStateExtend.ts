import { parseISO } from "date-fns";
import type { NotAdministeredState } from "../dto";

export type NotAdministeredStateExtend = Omit<
	NotAdministeredState,
	"last_updated"
> & {
	last_updated: Date;
};

export const mapNotAdminister = (
	NotAdministeredStateExtend: NotAdministeredState,
): NotAdministeredStateExtend => {
	return {
		...NotAdministeredStateExtend,
		last_updated: parseISO(NotAdministeredStateExtend.last_updated),
	};
};
