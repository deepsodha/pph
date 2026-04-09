import { slimJsonrpcTokenPost, createJSONRPCReq, systemAdmin } from ".";
import type {
	SystemAdminAdd,
	SystemAdminUpdate,
	SystemAdminDelete,
	SyncData,
	GetSystemadmin,
} from "@repo/types/dto";
import { url } from "./api";

export const addSystemAdmin = async (p: SystemAdminAdd) => {
	return slimJsonrpcTokenPost<SyncData, unknown>(
		`${url}`,
		createJSONRPCReq(1, systemAdmin.Add, [
			{
				...p,
			},
		]),
	);
};

export const getSystemAdmin = async () => {
	const paramData = {};
	return slimJsonrpcTokenPost<GetSystemadmin[], unknown>(
		`${url}`,
		createJSONRPCReq(1, systemAdmin.GetAll, [paramData]),
	);
};

export const deleteSystemAdmin = async (data: SystemAdminDelete) => {
	return slimJsonrpcTokenPost<SyncData, unknown>(
		`${url}`,
		createJSONRPCReq(1, systemAdmin.Delete, [data]),
	);
};

export const updateSystemAdmin = async (data: SystemAdminUpdate) => {
	return slimJsonrpcTokenPost<SyncData, unknown>(
		`${url}`,
		createJSONRPCReq(1, systemAdmin.Update, [data]),
	);
};
