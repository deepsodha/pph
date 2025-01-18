import type {
	AuthenticatedUser,
	ServiceLocationAdd,
	ServiceLocationDelete,
	ServiceLocationUpdate,
	SyncData,
	SelectServiceLocation,
} from "@repo/types/dto";
import { url } from "./api";
import { slimJsonrpcTokenPost, createJSONRPCReq, serviceLocation } from ".";

export const addServiceLoctionDetails = async (s: ServiceLocationAdd) => {
	return slimJsonrpcTokenPost<SyncData, unknown>(
		`${url}`,
		createJSONRPCReq(1, serviceLocation.Add, [
			{
				...s,
			},
		]),
	);
};

export const deleteServiceLocation = async (data: ServiceLocationDelete) => {
	return slimJsonrpcTokenPost<SyncData, unknown>(
		`${url}`,
		createJSONRPCReq(1, serviceLocation.Delete, [data]),
	);
};

export const updateServiceLocationDetails = async (
	data: ServiceLocationUpdate,
) => {
	return slimJsonrpcTokenPost<SyncData, unknown>(
		`${url}`,
		createJSONRPCReq(1, serviceLocation.Update, [data]),
	);
};
export const serviceLocation_select = async (o: SelectServiceLocation) => {
	return slimJsonrpcTokenPost<AuthenticatedUser, unknown>(
		`${url}`,
		createJSONRPCReq(1, serviceLocation.Select, [
			{
				...o,
			},
		]),
	);
};
// export const getServiceLocation = async () => {
// 	const ParamData = {};
// 	return slimJsonrpcTokenPost<GetServiceLocation[], unknown>(
// 		`${url}`,
// 		createJSONRPCReq(1,serviceLocation.GetAll, [ParamData]),
// 	);
// };
