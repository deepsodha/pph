import {
	type SlimJSONRPCRes,
	addHistoryAndPhysical,
	updateHistoryAndPhysical,
} from "@repo/services/src";
import type {
	HistoryAndPhysicalAdd,
	HistoryAndPhysicalUpdate,
	SyncData,
} from "@repo/types/dto";
import type { UseMutationResult } from "@tanstack/react-query";
import { MutationReuse } from "./mutation-reuse";

export function AddHistoryAndPhysicalDataFn(): UseMutationResult<
	SlimJSONRPCRes<SyncData, unknown>,
	Error,
	HistoryAndPhysicalAdd,
	unknown
> {
	return MutationReuse(
		["AddHistoryAndPhysical"],
		addHistoryAndPhysical,
		"HistoryAndPhysical",
		true,
	);
}

export function UpdateHistoryAndPhysicalDataFn(): UseMutationResult<
	SlimJSONRPCRes<SyncData, unknown>,
	Error,
	HistoryAndPhysicalUpdate,
	unknown
> {
	return MutationReuse(
		["UpdateHospitalization"],
		updateHistoryAndPhysical,
		"Hospitalization Update",
		true,
	);
}
