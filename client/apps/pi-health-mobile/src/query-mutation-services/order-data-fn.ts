import {
	type SlimJSONRPCRes,
	addOrder,
	deleteOrder,
	updateOrder,
} from "@repo/services/src";
import { MutationReuse } from "./mutation-reuse";
import type {
	OrderAdd,
	OrderDelete,
	OrderUpdate,
	SyncData,
} from "@repo/types/dto";
import type { UseMutationResult } from "@tanstack/react-query";

export function AddOrderDataFn(): UseMutationResult<
	SlimJSONRPCRes<SyncData, unknown>,
	Error,
	OrderAdd,
	unknown
> {
	return MutationReuse(["AddOrder"], addOrder, "Order Add", true);
}

export function UpdateOrderDataFn(): UseMutationResult<
	SlimJSONRPCRes<SyncData, unknown>,
	Error,
	OrderUpdate,
	unknown
> {
	return MutationReuse(["UpdateOrder"], updateOrder, "Order Update", true);
}

export function DeleteOrderDataFn(): UseMutationResult<
	SlimJSONRPCRes<SyncData, unknown>,
	Error,
	OrderDelete,
	unknown
> {
	return MutationReuse(["DeleteOrder"], deleteOrder, "Delete Order");
}
