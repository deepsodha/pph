import type {
	SyncData,
	UserAdd,
	UserDelete,
	UserUpdate,
} from "@repo/types/dto";
import { url } from "./api";
import { slimJsonrpcTokenPost, createJSONRPCReq, user } from ".";

export const addUser = async (p: UserAdd) => {
	return slimJsonrpcTokenPost<SyncData, unknown>(
		`${url}`,
		createJSONRPCReq(1, user.Add, [
			{
				...p,
			},
		]),
	);
};

export const deleteUser = async (data: UserDelete) => {
	return slimJsonrpcTokenPost<SyncData, unknown>(
		`${url}`,
		createJSONRPCReq(1, user.Delete, [data]),
	);
};

export const updateUser = async (data: UserUpdate) => {
	return slimJsonrpcTokenPost<SyncData, unknown>(
		`${url}`,
		createJSONRPCReq(1, user.Update, [data]),
	);
};
