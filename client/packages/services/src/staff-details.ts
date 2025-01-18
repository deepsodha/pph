import type {
	SyncData,
	StaffAdd,
	StaffDelete,
	StaffUpdate,
} from "@repo/types/dto";
import { url } from "./api";
import { slimJsonrpcTokenPost, createJSONRPCReq, staff } from ".";

export const addStaffDetails = async (p: StaffAdd) => {
	return slimJsonrpcTokenPost<SyncData, unknown>(
		`${url}`,
		createJSONRPCReq(1, staff.Add, [
			{
				...p,
			},
		]),
	);
};

export const deleteStaffDetails = async (data: StaffDelete) => {
	return slimJsonrpcTokenPost<SyncData, unknown>(
		`${url}`,
		createJSONRPCReq(1, staff.Delete, [data]),
	);
};

export const updateStaffDetails = async (data: StaffUpdate) => {
	return slimJsonrpcTokenPost<SyncData, unknown>(
		`${url}`,
		createJSONRPCReq(1, staff.Update, [data]),
	);
};
