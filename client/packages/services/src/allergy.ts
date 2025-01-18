import { createJSONRPCReq, patients } from ".";
import type {
	AllergyAdd,
	AllergyDelete,
	AllergyUpdate,
	SyncData,
} from "@repo/types/dto";
import { url } from "./api";
import { slimJsonrpcTokenPost } from "./jsonrpc-helper";

export const addAllergy = async (p: AllergyAdd) => {
	return slimJsonrpcTokenPost<SyncData, unknown>(
		`${url}`,
		createJSONRPCReq(1, patients.Allergies.Add, [
			{
				...p,
			},
		]),
	);
};

export const deleteAllergy = async (data: AllergyDelete) => {
	return slimJsonrpcTokenPost<SyncData, unknown>(
		`${url}`,
		createJSONRPCReq(1, patients.Allergies.Delete, [data]),
	);
};

export const getAllergyName = async () => {
	const ParamData = {};
	return slimJsonrpcTokenPost<string[], unknown>(
		`${url}`,
		createJSONRPCReq(1, patients.Allergies.Name, [ParamData]),
	);
};
export const updateAllergy = async (data: AllergyUpdate) => {
	return slimJsonrpcTokenPost<SyncData, unknown>(
		`${url}`,
		createJSONRPCReq(1, patients.Allergies.Update, [data]),
	);
};
