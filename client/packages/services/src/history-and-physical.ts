import type {
	HistoryAndPhysicalAdd,
	HistoryAndPhysicalUpdate,
	SyncData,
} from "@repo/types/dto";
import { slimJsonrpcTokenPost } from "./jsonrpc-helper";
import { url } from "./api";
import { createJSONRPCReq } from "./jsonrpc-type";
import { patients } from "./rpc-methods";

export const addHistoryAndPhysical = async (p: HistoryAndPhysicalAdd) => {
	return slimJsonrpcTokenPost<SyncData, unknown>(
		`${url}`,
		createJSONRPCReq(1, patients.Notes.HistoryAndPhysical.Add, [
			{
				...p,
			},
		]),
	);
};

export const updateHistoryAndPhysical = (p: HistoryAndPhysicalUpdate) => {
	return slimJsonrpcTokenPost<SyncData, unknown>(
		`${url}`,
		createJSONRPCReq(1, patients.Notes.HistoryAndPhysical.Update, [
			{
				...p,
			},
		]),
	);
};
