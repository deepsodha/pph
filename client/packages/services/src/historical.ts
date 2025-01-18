import { slimJsonrpcTokenPost, createJSONRPCReq, patients } from ".";
import type {
	HistoricalAdd,
	HistoricalDelete,
	HistoricalUpdate,
	SyncData,
} from "@repo/types/dto";
import { url } from "./api";

export const addHistorical = async (v: HistoricalAdd) => {
	return slimJsonrpcTokenPost<SyncData, unknown>(
		`${url}`,
		createJSONRPCReq(1, patients.AddHistorical.Add, [
			{
				...v,
			},
		]),
	);
};

export const deleteHistorical = async (data: HistoricalDelete) => {
	const paramData = {
		id: data.id,
		patient_id: data.patient_id,
		last_updated_input: data.last_updated_input,
	};
	return slimJsonrpcTokenPost<SyncData, unknown>(
		`${url}`,
		createJSONRPCReq(1, patients.AddHistorical.Delete, [paramData]),
	);
};

export const updateHistorical = async (data: HistoricalUpdate) => {
	return slimJsonrpcTokenPost<SyncData, unknown>(
		`${url}`,
		createJSONRPCReq(1, patients.AddHistorical.Update, [data]),
	);
};
