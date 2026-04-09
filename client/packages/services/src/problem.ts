import { url } from "./api";
import { slimJsonrpcTokenPost, createJSONRPCReq, patients } from ".";
import type {
	ProblemsAdd,
	ProblemsUpdate,
	ProblemsDelete,
	SyncData,
} from "@repo/types/dto";

export const addProblem = async (p: ProblemsAdd) => {
	return slimJsonrpcTokenPost<SyncData, unknown>(
		`${url}`,
		createJSONRPCReq(1, patients.Problems.Add, [
			{
				...p,
			},
		]),
	);
};

export const deleteProblem = async (data: ProblemsDelete) => {
	const paramData = {
		id: data.id,
		patient_id: data.patient_id,
		last_updated_input: data.last_updated_input,
	};
	return slimJsonrpcTokenPost<SyncData, unknown>(
		`${url}`,
		createJSONRPCReq(1, patients.Problems.Delete, [paramData]),
	);
};

export const updateProblem = async (data: ProblemsUpdate) => {
	return slimJsonrpcTokenPost<SyncData, unknown>(
		`${url}`,
		createJSONRPCReq(1, patients.Problems.Update, [data]),
	);
};
