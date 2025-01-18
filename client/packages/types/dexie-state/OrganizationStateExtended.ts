import { parseISO } from "date-fns";
import type { OrganizationState } from "../dto";

export type OrganizationStateExtend = Omit<
	OrganizationState,
	"last_updated"
> & {
	last_updated: Date;
};

export const mapOrganization = (
	OrganizationStateExtend: OrganizationState,
): OrganizationStateExtend => {
	return {
		...OrganizationStateExtend,
		last_updated: parseISO(OrganizationStateExtend.last_updated),
	};
};
