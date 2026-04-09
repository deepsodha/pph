import { createJSONRPCReq, patients } from ".";
import type {
	AdministerAdd,
	AdministerDelete,
	AdministerUpdate,
	SyncData,
} from "@repo/types/dto";
import { url } from "./api";
import { slimJsonrpcTokenPost } from "./jsonrpc-helper";

export const addAdminister = (p: AdministerAdd) => {
	return slimJsonrpcTokenPost<SyncData, unknown>(
		`${url}`,
		createJSONRPCReq(1, patients.Addminister.Add, [
			{
				...p,
			},
		]),
	);
};

export const deleteAdminister = (data: AdministerDelete) => {
	return slimJsonrpcTokenPost<SyncData, unknown>(
		`${url}`,
		createJSONRPCReq(1, patients.Addminister.Delete, [
			{
				...data,
			},
		]),
	);
};

export const updateAdminister = (data: AdministerUpdate) => {
	return slimJsonrpcTokenPost<SyncData, unknown>(
		`${url}`,
		createJSONRPCReq(1, patients.Addminister.Update, [data]),
	);
};
