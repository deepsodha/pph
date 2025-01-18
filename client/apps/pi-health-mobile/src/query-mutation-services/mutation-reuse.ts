import type { SyncData } from "@repo/types/dto";
import { useToast } from "@chakra-ui/react";
import {
	type MutationFunction,
	type MutationKey,
	type UseMutationResult,
	useMutation,
} from "@tanstack/react-query";
import type { SlimJSONRPCRes } from "../../../../packages/services/src/jsonrpc-type";
import { useNavigate, useRouter } from "@tanstack/react-router";
import { db } from "../db/db";

type MutationFn<T> = MutationFunction<SlimJSONRPCRes<SyncData, unknown>, T>;

export function MutationReuse<T>(
	key: MutationKey,
	mutationFn: MutationFn<T>,
	role: string,
	route?: boolean,
	path?: string,
): UseMutationResult<SlimJSONRPCRes<SyncData, unknown>, Error, T, unknown> {
	const navigate = useNavigate();
	const router = useRouter();
	const toast = useToast();

	return useMutation({
		mutationKey: key,
		mutationFn: mutationFn,
		onSuccess: async (data) => {
			if (data.result) {
				route && navigate({ to: path ? path : router.history.back() });
				toast({
					description: `${role} Success`,
					status: "success",
					duration: 2000,
				});
				db.bulkPut(data.result);
			} else if (data.error) {
				toast({
					description: data.error.message,
					status: "error",
					duration: 2000,
				});
			}
		},
		onError: () => {},
	});
}
