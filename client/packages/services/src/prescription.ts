import { slimJsonrpcTokenPost, createJSONRPCReq, prescription } from ".";
import type {
	PrescriptionAdd,
	PrescriptionUpdate,
	PrescriptionDelete,
	SyncData,
} from "@repo/types/dto";
import { url } from "./api";

export const addPrescription = async (p: PrescriptionAdd) => {
	return slimJsonrpcTokenPost<SyncData, unknown>(
		`${url}`,
		createJSONRPCReq(1, prescription.Add, [
			{
				...p,
			},
		]),
	);
};

export const deletePrescription = async (data: PrescriptionDelete) => {
	const paramData = {
		id: data.id,
		patient_id: data.patient_id,
		last_updated_input: data.last_updated_input,
	};
	return slimJsonrpcTokenPost<SyncData, unknown>(
		`${url}`,
		createJSONRPCReq(1, prescription.Delete, [paramData]),
	);
};

export const updatePrescription = async (data: PrescriptionUpdate) => {
	return slimJsonrpcTokenPost<SyncData, unknown>(
		`${url}`,
		createJSONRPCReq(1, prescription.Update, [data]),
	);
};
