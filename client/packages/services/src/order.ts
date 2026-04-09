import { slimJsonrpcTokenPost, createJSONRPCReq, patients } from ".";
import type {
	OrderAdd,
	OrderUpdate,
	OrderDelete,
	SyncData,
} from "@repo/types/dto";
import { url } from "./api";

export const addOrder = async (p: OrderAdd) => {
	return slimJsonrpcTokenPost<SyncData, unknown>(
		`${url}`,
		createJSONRPCReq(1, patients.Order.Add, [
			{
				...p,
			},
		]),
	);
};

export const updateOrder = async (data: OrderUpdate) => {
	return slimJsonrpcTokenPost<SyncData, unknown>(
		`${url}`,
		createJSONRPCReq(1, patients.Order.Update, [data]),
	);
};

export const deleteOrder = async (data: OrderDelete) => {
	const paramData = {
		id: data.id,
		patient_id: data.patient_id,
		last_updated_input: data.last_updated_input,
	};
	return slimJsonrpcTokenPost<SyncData, unknown>(
		`${url}`,
		createJSONRPCReq(1, patients.Order.Delete, [paramData]),
	);
};
