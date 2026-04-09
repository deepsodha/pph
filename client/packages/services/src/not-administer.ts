import { url } from "./api";
import { slimJsonrpcTokenPost, createJSONRPCReq, patients } from ".";
import type {
	NotAdministeredAdd,
	NotAdministeredUpdate,
	NotAdministeredDelete,
	SyncData,
} from "@repo/types/dto";

export const addNotAdminister = async (p: NotAdministeredAdd) => {
	return slimJsonrpcTokenPost<SyncData, unknown>(
		`${url}`,
		createJSONRPCReq(1, patients.Notaddminister.Add, [
			{
				...p,
			},
		]),
	);
};

export const deleteNotAdminister = async (data: NotAdministeredDelete) => {
	const paramData = {
		id: data.id,
		patient_id: data.patient_id,
		last_updated_input: data.last_updated_input,
	};
	return slimJsonrpcTokenPost<SyncData, unknown>(
		`${url}`,
		createJSONRPCReq(1, patients.Notaddminister.Delete, [paramData]),
	);
};

export const updateNotAdminister = async (data: NotAdministeredUpdate) => {
	return slimJsonrpcTokenPost<SyncData, unknown>(
		`${url}`,
		createJSONRPCReq(1, patients.Notaddminister.Update, [data]),
	);
};
