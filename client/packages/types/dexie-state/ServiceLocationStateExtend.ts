import { parseISO } from "date-fns";
import type { ServiceLocationState } from "../dto";

export type ServiceLocationStateExtend = Omit<
	ServiceLocationState,
	"last_updated"
> & {
	last_updated: Date;
};

export const mapServiceLocation = (
	ServiceLocationStateExtend: ServiceLocationState,
): ServiceLocationStateExtend => {
	return {
		...ServiceLocationStateExtend,
		last_updated: parseISO(ServiceLocationStateExtend.last_updated),
	};
};
