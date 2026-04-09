import type {
	PastSurgicalHistoryAdd,
	PastSurgicalHistoryDelete,
	PastSurgicalHistoryUpdate,
	SyncData,
} from "@repo/types/dto";
import { slimJsonrpcTokenPost, url, createJSONRPCReq, patients } from ".";

export const addPastSurgicalHistory = (p: PastSurgicalHistoryAdd) => {
	return slimJsonrpcTokenPost<SyncData, unknown>(
		`${url}`,
		createJSONRPCReq(1, patients.History.PastSurgicalHistory.Add, [
			{
				...p,
			},
		]),
	);
};

export const updatePastSurgicalHistory = (p: PastSurgicalHistoryUpdate) => {
	return slimJsonrpcTokenPost<SyncData, unknown>(
		`${url}`,
		createJSONRPCReq(1, patients.History.PastSurgicalHistory.Update, [
			{
				...p,
			},
		]),
	);
};

export const deletePastSurgicalHistory = (p: PastSurgicalHistoryDelete) => {
	return slimJsonrpcTokenPost<SyncData, unknown>(
		`${url}`,
		createJSONRPCReq(1, patients.History.PastSurgicalHistory.Delete, [
			{
				...p,
			},
		]),
	);
};
