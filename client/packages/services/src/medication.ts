import { url } from "./api";
import { patients } from "./rpc-methods";
import { createJSONRPCReq } from "./jsonrpc-type";
import { slimJsonrpcTokenPost } from ".";
import type {
	MedicationsAdd,
	MedicationUpdate,
	MedicationDelete,
	SyncData,
} from "@repo/types/dto";

export const addMedication = async (p: MedicationsAdd) => {
	return slimJsonrpcTokenPost<SyncData, unknown>(
		`${url}`,
		createJSONRPCReq(1, patients.Medications.Add, [
			{
				...p,
			},
		]),
	);
};

export const deleteMedication = async (data: MedicationDelete) => {
	const paramData = {
		id: data.id,
		patient_id: data.patient_id,
		last_updated_input: data.last_updated_input,
	};
	return slimJsonrpcTokenPost<SyncData, unknown>(
		`${url}`,
		createJSONRPCReq(1, patients.Medications.Delete, [paramData]),
	);
};

export const updateMedication = async (data: MedicationUpdate) => {
	return slimJsonrpcTokenPost<SyncData, unknown>(
		`${url}`,
		createJSONRPCReq(1, patients.Medications.Update, [data]),
	);
};
