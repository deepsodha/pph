import { type SlimJSONRPCRes, addSoap, updateSoap } from "@repo/services/src";
import type { SoapAdd, SoapUpdate, SyncData } from "@repo/types/dto";
import type { UseMutationResult } from "@tanstack/react-query";
import { MutationReuse } from "./mutation-reuse";

export function AddSoapDataFn(): UseMutationResult<
	SlimJSONRPCRes<SyncData, unknown>,
	Error,
	SoapAdd,
	unknown
> {
	return MutationReuse(["AddSoap"], addSoap, "Soap", true);
}
export function UpdateSoapDataFn(): UseMutationResult<
	SlimJSONRPCRes<SyncData, unknown>,
	Error,
	SoapUpdate,
	unknown
> {
	return MutationReuse(["UpdateSoap"], updateSoap, "Soap", true);
}
