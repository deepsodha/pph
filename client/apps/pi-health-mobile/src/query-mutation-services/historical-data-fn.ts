import {
	type SlimJSONRPCRes,
	addHistorical,
	deleteHistorical,
	updateHistorical,
} from "@repo/services/src";
import { MutationReuse } from "./mutation-reuse";
import type {
	HistoricalAdd,
	HistoricalDelete,
	HistoricalUpdate,
	SyncData,
} from "@repo/types/dto";
import type { UseMutationResult } from "@tanstack/react-query";

export function AddHistoricalDataFn(): UseMutationResult<
	SlimJSONRPCRes<SyncData, unknown>,
	Error,
	HistoricalAdd,
	unknown
> {
	return MutationReuse(["AddHistorical"], addHistorical, "Historical", true);
}

export function UpdateHistoricalDataFn(): UseMutationResult<
	SlimJSONRPCRes<SyncData, unknown>,
	Error,
	HistoricalUpdate,
	unknown
> {
	return MutationReuse(
		["UpdateHistorical"],
		updateHistorical,
		"Historical Update",
		true,
	);
}

export function DeleteHistoricalDataFn(): UseMutationResult<
	SlimJSONRPCRes<SyncData, unknown>,
	Error,
	HistoricalDelete,
	unknown
> {
	return MutationReuse(
		["DeleteHistorical"],
		deleteHistorical,
		"Delete Historical",
	);
}
