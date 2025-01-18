import { parseISO } from "date-fns";
import type { UserState } from "../dto";

export type UserStateExtend = Omit<UserState, "last_updated"> & {
	last_updated: Date;
};

export const mapUser = (UserStateExtend: UserState): UserStateExtend => {
	return {
		...UserStateExtend,
		last_updated: parseISO(UserStateExtend.last_updated),
	};
};
