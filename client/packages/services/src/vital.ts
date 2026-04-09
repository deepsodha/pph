import { slimJsonrpcTokenPost, createJSONRPCReq, patients } from ".";
import type {
	VitalsAdd,
	VitalsUpdate,
	VitalsDelete,
	SyncData,
} from "@repo/types/dto";
import { url } from "./api";

export const addVital = async (v: VitalsAdd) => {
	return slimJsonrpcTokenPost<SyncData, unknown>(
		`${url}`,
		createJSONRPCReq(1, patients.Vitals.Add, [
			{
				...v,
			},
		]),
	);
};

export const deleteVital = async (data: VitalsDelete) => {
	return slimJsonrpcTokenPost<SyncData, unknown>(
		`${url}`,
		createJSONRPCReq(1, patients.Vitals.Delete, [data]),
	);
};

export const updateVital = async (data: VitalsUpdate) => {
	return slimJsonrpcTokenPost<SyncData, unknown>(
		`${url}`,
		createJSONRPCReq(1, patients.Vitals.Update, [data]),
	);
};
