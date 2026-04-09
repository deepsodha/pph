import type {
	DoctorDelete,
	DoctorAdd,
	DoctorUpdate,
	SyncData,
} from "@repo/types/dto";
import { url } from "./api";
import { slimJsonrpcTokenPost, createJSONRPCReq, doctors } from ".";

export const addDoctorDetails = async (p: DoctorAdd) => {
	return slimJsonrpcTokenPost<SyncData, unknown>(
		`${url}`,
		createJSONRPCReq(1, doctors.Add, [
			{
				...p,
			},
		]),
	);
};

export const deleteDoctorDetails = async (data: DoctorDelete) => {
	return slimJsonrpcTokenPost<SyncData, unknown>(
		`${url}`,
		createJSONRPCReq(1, doctors.Delete, [data]),
	);
};

export const updateDoctorDetails = async (data: DoctorUpdate) => {
	return slimJsonrpcTokenPost<SyncData, unknown>(
		`${url}`,
		createJSONRPCReq(1, doctors.Update, [data]),
	);
};
