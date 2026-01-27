import type {
	OBandPregnancyAdd,
	OBandPregnancyDelete,
	OBandPregnancyUpdate,
	SyncData,
} from "@repo/types/dto";
import { slimJsonrpcTokenPost, url, createJSONRPCReq, patients } from ".";

export const addObandPregnancy = (p: OBandPregnancyAdd) => {
	return slimJsonrpcTokenPost<SyncData, unknown>(
		`${url}`,
		createJSONRPCReq(1, patients.History.ObAndPregnancy.Add, [
			{
				...p,
			},
		]),
	);
};

export const updateObandPregnancy = (p: OBandPregnancyUpdate) => {
	return slimJsonrpcTokenPost<SyncData, unknown>(
		`${url}`,
		createJSONRPCReq(1, patients.History.ObAndPregnancy.Update, [
			{
				...p,
			},
		]),
	);
};

export const deleteObandPregnancy = (p: OBandPregnancyDelete) => {
	return slimJsonrpcTokenPost<SyncData, unknown>(
		`${url}`,
		createJSONRPCReq(1, patients.History.ObAndPregnancy.Delete, [
			{
				...p,
			},
		]),
	);
};
