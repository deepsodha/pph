import { parseISO } from "date-fns";
import type { ProblemState } from "../dto";

export type ProblemStateExtend = Omit<ProblemState, "last_updated"> & {
	last_updated: Date;
};

export const mapProblem = (
	ProblemStateExtend: ProblemState,
): ProblemStateExtend => {
	return {
		...ProblemStateExtend,
		last_updated: parseISO(ProblemStateExtend.last_updated),
	};
};
