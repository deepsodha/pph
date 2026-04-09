import {
	type SlimJSONRPCRes,
	addObandPregnancy,
	deleteObandPregnancy,
	updateObandPregnancy,
} from "@repo/services/src";
import type {
	OBandPregnancyAdd,
	OBandPregnancyDelete,
	OBandPregnancyUpdate,
	SyncData,
} from "@repo/types/dto";
import type { UseMutationResult } from "@tanstack/react-query";
import { MutationReuse } from "./mutation-reuse";

export function AddObAndPregnancyFn(): UseMutationResult<
	SlimJSONRPCRes<SyncData, unknown>,
	Error,
	OBandPregnancyAdd,
	unknown
> {
	return MutationReuse(
		["ObAndPregnancyAdd"],
		addObandPregnancy,
		"ObAndPregnancy Add",
		true,
	);
}

export function UpdateObAndPregnancyFn(): UseMutationResult<
	SlimJSONRPCRes<SyncData, unknown>,
	Error,
	OBandPregnancyUpdate,
	unknown
> {
	return MutationReuse(
		["ObAndPregnancyUpdate"],
		updateObandPregnancy,
		"ObAndPregnancy Update",
		true,
	);
}

export function DeleteObAndPregnancyFn(): UseMutationResult<
	SlimJSONRPCRes<SyncData, unknown>,
	Error,
	OBandPregnancyDelete,
	unknown
> {
	return MutationReuse(
		["ObAndPregnancyDelete"],
		deleteObandPregnancy,
		"ObAndPregnancy Delete",
	);
}
