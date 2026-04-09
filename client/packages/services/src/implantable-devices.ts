import type {
	ImplantableDevicesAdd,
	ImplantableDevicesDelete,
	ImplantableDevicesUpdate,
	SyncData,
} from "@repo/types/dto";
import { slimJsonrpcTokenPost, url, createJSONRPCReq, patients } from ".";

export const addImplantableDevices = (p: ImplantableDevicesAdd) => {
	return slimJsonrpcTokenPost<SyncData, unknown>(
		`${url}`,
		createJSONRPCReq(1, patients.History.ImplantableDevices.Add, [
			{
				...p,
			},
		]),
	);
};

export const updateImplantableDevices = (p: ImplantableDevicesUpdate) => {
	return slimJsonrpcTokenPost<SyncData, unknown>(
		`${url}`,
		createJSONRPCReq(1, patients.History.ImplantableDevices.Update, [
			{
				...p,
			},
		]),
	);
};

export const deleteImplantableDevices = (p: ImplantableDevicesDelete) => {
	return slimJsonrpcTokenPost<SyncData, unknown>(
		`${url}`,
		createJSONRPCReq(1, patients.History.ImplantableDevices.Delete, [
			{
				...p,
			},
		]),
	);
};
