import type {
	SocialHistoryAdd,
	SocialHistoryDelete,
	SocialHistoryUpdate,
	SyncData,
} from "@repo/types/dto";
import { slimJsonrpcTokenPost, url, createJSONRPCReq, patients } from ".";

export const addSocialHistory = (p: SocialHistoryAdd) => {
	return slimJsonrpcTokenPost<SyncData, unknown>(
		`${url}`,
		createJSONRPCReq(1, patients.History.SocialHistory.Add, [
			{
				...p,
			},
		]),
	);
};

export const updateSocialHistory = (p: SocialHistoryUpdate) => {
	return slimJsonrpcTokenPost<SyncData, unknown>(
		`${url}`,
		createJSONRPCReq(1, patients.History.SocialHistory.Update, [
			{
				...p,
			},
		]),
	);
};

export const deleteSocialHistory = (p: SocialHistoryDelete) => {
	return slimJsonrpcTokenPost<SyncData, unknown>(
		`${url}`,
		createJSONRPCReq(1, patients.History.SocialHistory.Delete, [
			{
				...p,
			},
		]),
	);
};
