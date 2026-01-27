import { createJSONRPCReq, patients } from ".";
import type {
	PatientAdd,
	PatientUpdate,
	PatientDelete,
	SyncData,
} from "@repo/types/dto";
import { url } from "./api";
import { slimJsonrpcTokenPost } from "./jsonrpc-helper";

export const addPatient = async (p: PatientAdd) => {
	return slimJsonrpcTokenPost<SyncData, unknown>(
		`${url}`,
		createJSONRPCReq(1, patients.Add, [
			{
				...p,
			},
		]),
	);
};

export const deletePatient = async (data: PatientDelete) => {
	return slimJsonrpcTokenPost<SyncData, unknown>(
		`${url}`,
		createJSONRPCReq(1, patients.Delete, [data]),
	);
};

export const updatePatient = async (data: PatientUpdate) => {
	return slimJsonrpcTokenPost<SyncData, unknown>(
		`${url}`,
		createJSONRPCReq(1, patients.Update, [data]),
	);
};
