import type {
	HospitalizationAdd,
	HospitalizationDelete,
	HospitalizationUpdate,
	SyncData,
} from "@repo/types/dto";
import { slimJsonrpcTokenPost, url, createJSONRPCReq, patients } from ".";

export const addHospitalizationHistory = (p: HospitalizationAdd) => {
	return slimJsonrpcTokenPost<SyncData, unknown>(
		`${url}`,
		createJSONRPCReq(1, patients.History.Hospitalization.Add, [
			{
				...p,
			},
		]),
	);
};

export const updateHospitalizationHistory = (p: HospitalizationUpdate) => {
	return slimJsonrpcTokenPost<SyncData, unknown>(
		`${url}`,
		createJSONRPCReq(1, patients.History.Hospitalization.Update, [
			{
				...p,
			},
		]),
	);
};

export const deleteHospitalizationHistory = (p: HospitalizationDelete) => {
	return slimJsonrpcTokenPost<SyncData, unknown>(
		`${url}`,
		createJSONRPCReq(1, patients.History.Hospitalization.Delete, [
			{
				...p,
			},
		]),
	);
};
