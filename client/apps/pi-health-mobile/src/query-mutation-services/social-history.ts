import {
	addSocialHistory,
	updateSocialHistory,
	deleteSocialHistory,
	type SlimJSONRPCRes,
} from "@repo/services/src";
import { MutationReuse } from "./mutation-reuse";
import type {
	SyncData,
	SocialHistoryAdd,
	SocialHistoryUpdate,
	SocialHistoryDelete,
} from "@repo/types/dto";
import type { UseMutationResult } from "@tanstack/react-query";

export function AddSocialHistoryFn(): UseMutationResult<
	SlimJSONRPCRes<SyncData, unknown>,
	Error,
	SocialHistoryAdd,
	unknown
> {
	return MutationReuse(
		["SocialHistoryAdd"],
		addSocialHistory,
		"SocialHistory Add",
		true,
	);
}

export function UpdateSocialHistoryFn(): UseMutationResult<
	SlimJSONRPCRes<SyncData, unknown>,
	Error,
	SocialHistoryUpdate,
	unknown
> {
	return MutationReuse(
		["SocialHistoryUpdate"],
		updateSocialHistory,
		"SocialHistory Update",
		true,
	);
}

export function DeleteSocialHistoryFn(): UseMutationResult<
	SlimJSONRPCRes<SyncData, unknown>,
	Error,
	SocialHistoryDelete,
	unknown
> {
	return MutationReuse(
		["SocialHistoryDelete"],
		deleteSocialHistory,
		"SocialHistory Delete",
	);
}
