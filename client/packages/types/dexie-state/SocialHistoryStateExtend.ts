import { parseISO } from "date-fns";
import type { SocialHistoryState } from "../dto";

export type SocialHistoryStateExtend = Omit<
	SocialHistoryState,
	"last_updated"
> & {
	last_updated: Date;
};

export const mapSocialHistory = (
	SocialHistoryStateExtend: SocialHistoryState,
): SocialHistoryStateExtend => {
	return {
		...SocialHistoryStateExtend,
		last_updated: parseISO(SocialHistoryStateExtend.last_updated),
	};
};
