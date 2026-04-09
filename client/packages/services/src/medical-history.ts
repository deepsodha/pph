import type {
	PastMedicalHistoryAdd,
	PastMedicalHistoryDelete,
	PastMedicalHistoryUpdate,
	SyncData,
} from "@repo/types/dto";
import { slimJsonrpcTokenPost, url, createJSONRPCReq, patients } from ".";

export const addPastMedicalHistory = (p: PastMedicalHistoryAdd) => {
	return slimJsonrpcTokenPost<SyncData, unknown>(
		`${url}`,
		createJSONRPCReq(1, patients.History.PastMedicalHistory.Add, [
			{
				...p,
			},
		]),
	);
};

export const updatePastMedicalHistory = (p: PastMedicalHistoryUpdate) => {
	return slimJsonrpcTokenPost<SyncData, unknown>(
		`${url}`,
		createJSONRPCReq(1, patients.History.PastMedicalHistory.Update, [
			{
				...p,
			},
		]),
	);
};

export const deletePastMedicalHistory = (p: PastMedicalHistoryDelete) => {
	return slimJsonrpcTokenPost<SyncData, unknown>(
		`${url}`,
		createJSONRPCReq(1, patients.History.PastMedicalHistory.Delete, [
			{
				...p,
			},
		]),
	);
};
