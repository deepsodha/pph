import type {
	FamilyHistoryAdd,
	FamilyHistoryDelete,
	FamilyHistoryUpdate,
	SyncData,
} from "@repo/types/dto";
import { slimJsonrpcTokenPost, url, createJSONRPCReq, patients } from ".";

export const addFamilyHistory = (p: FamilyHistoryAdd) => {
	return slimJsonrpcTokenPost<SyncData, unknown>(
		`${url}`,
		createJSONRPCReq(1, patients.History.FamilyHistory.Add, [
			{
				...p,
			},
		]),
	);
};

export const updateFamilyHistory = (p: FamilyHistoryUpdate) => {
	return slimJsonrpcTokenPost<SyncData, unknown>(
		`${url}`,
		createJSONRPCReq(1, patients.History.FamilyHistory.Update, [
			{
				...p,
			},
		]),
	);
};

export const deleteFamilyHistory = (p: FamilyHistoryDelete) => {
	return slimJsonrpcTokenPost<SyncData, unknown>(
		`${url}`,
		createJSONRPCReq(1, patients.History.FamilyHistory.Delete, [
			{
				...p,
			},
		]),
	);
};
