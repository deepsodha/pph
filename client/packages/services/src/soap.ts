import type { SoapAdd, SoapUpdate, SyncData } from "@repo/types/dto";
import { slimJsonrpcTokenPost } from "./jsonrpc-helper";
import { url } from "./api";
import { createJSONRPCReq } from "./jsonrpc-type";
import { patients } from "./rpc-methods";

export const addSoap = async (p: SoapAdd) => {
	return slimJsonrpcTokenPost<SyncData, unknown>(
		`${url}`,
		createJSONRPCReq(1, patients.Notes.Soap.Add, [
			{
				...p,
			},
		]),
	);
};
export const updateSoap = (p: SoapUpdate) => {
	return slimJsonrpcTokenPost<SyncData, unknown>(
		`${url}`,
		createJSONRPCReq(1, patients.Notes.Soap.Update, [
			{
				...p,
			},
		]),
	);
};
