import {
	type SlimJSONRPCRes,
	addProblem,
	deleteProblem,
	updateProblem,
} from "@repo/services/src";
import { MutationReuse } from "./mutation-reuse";
import type {
	ProblemsAdd,
	ProblemsDelete,
	ProblemsUpdate,
	SyncData,
} from "@repo/types/dto";
import type { UseMutationResult } from "@tanstack/react-query";

export function AddProblemDataFn(): UseMutationResult<
	SlimJSONRPCRes<SyncData, unknown>,
	Error,
	ProblemsAdd,
	unknown
> {
	return MutationReuse(["AddProblem"], addProblem, "Problem", true);
}
export function UpdateProblemDataFn(): UseMutationResult<
	SlimJSONRPCRes<SyncData, unknown>,
	Error,
	ProblemsUpdate,
	unknown
> {
	return MutationReuse(
		["UpdateProblem"],
		updateProblem,
		"Problem Update",
		true,
	);
}

export function deleteProblemFn(): UseMutationResult<
	SlimJSONRPCRes<SyncData, unknown>,
	Error,
	ProblemsDelete,
	unknown
> {
	return MutationReuse(["DeleteProblem"], deleteProblem, "Delete Problem");
}
