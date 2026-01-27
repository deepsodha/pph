import { slimJsonrpcTokenPost, createJSONRPCReq, organizations } from ".";
import type {
	OrganizationAdd,
	OrganizationUpdate,
	OrganizationDelete,
	SelectOrganization,
	AuthenticatedUser,
	Getorganization,
	LastUpdatedInput,
	SyncData,
	LocationRecord,
} from "@repo/types/dto";

import { url } from "./api";

export const addOrganization = async (p: OrganizationAdd) => {
	return slimJsonrpcTokenPost<OrganizationAdd, unknown>(
		`${url}`,
		createJSONRPCReq(1, organizations.Add, [
			{
				...p,
			},
		]),
	);
};

export const updateOrganization = async (data: OrganizationUpdate) => {
	const paramData = {
		id: data.id,
		name: data.name,
		details: data.details,
		phone_number: data.phone_number,
		email: data.email,
		address: {
			pin_code: data.address.pin_code,
			city: data.address.city,
			state: data.address.state,
			address_line: data.address.address_line,
			country: data.address.country,
		},
	};
	return slimJsonrpcTokenPost<OrganizationUpdate, unknown>(
		`${url}`,
		createJSONRPCReq(1, organizations.Update, [paramData]),
	);
};

export const deleteOrganization = async (data: OrganizationDelete) => {
	return slimJsonrpcTokenPost<SyncData, unknown>(
		`${url}`,
		createJSONRPCReq(1, organizations.Delete, [data]),
	);
};

export const organization_select = async (o: SelectOrganization) => {
	return slimJsonrpcTokenPost<AuthenticatedUser, unknown>(
		`${url}`,
		createJSONRPCReq(1, organizations.Select, [
			{
				...o,
			},
		]),
	);
};
export const getOrganizationById = async (id: string) => {
	const ParamData = {
		org_id: id,
	};
	return slimJsonrpcTokenPost<Getorganization[], unknown>(
		`${url}`,
		createJSONRPCReq(1, organizations.Get, [ParamData]),
	);
};
export const getOrganizationLocationPinCode = async (pin_code: string) => {
	const ParamData = {
		pin_code: pin_code,
	};
	return slimJsonrpcTokenPost<LocationRecord[], unknown>(
		`${url}`,
		createJSONRPCReq(1, organizations.Location, [ParamData]),
	);
};

export const getOrganizationCountry = async () => {
	const ParamData = {};
	return slimJsonrpcTokenPost<string[], unknown>(
		`${url}`,
		createJSONRPCReq(1, organizations.GetAllCountries, [ParamData]),
	);
};

export const getOrganizationState = async (country: string | null) => {
	const ParamData = { country_name: country };
	return slimJsonrpcTokenPost<string[], unknown>(
		`${url}`,
		createJSONRPCReq(1, organizations.GetAllState, [ParamData]),
	);
};

export const getOrganizationPinCode = async () => {
	const ParamData = {};
	return slimJsonrpcTokenPost<string[], unknown>(
		`${url}`,
		createJSONRPCReq(1, organizations.PinCodes, [ParamData]),
	);
};

export const getOrganization = async () => {
	const ParamData = {};
	return slimJsonrpcTokenPost<Getorganization[], unknown>(
		`${url}`,
		createJSONRPCReq(1, organizations.GetAll, [ParamData]),
	);
};

export const getOrganizationCity = async (
	state_name: string,
	country_name: string,
) => {
	const ParamData = { state_name: state_name, country_name: country_name };
	return slimJsonrpcTokenPost<string[], unknown>(
		`${url}`,
		createJSONRPCReq(1, organizations.GetAllCity, [ParamData]),
	);
};

export const getVaccines = async () => {
	const ParamData = {};
	return slimJsonrpcTokenPost<string[], unknown>(
		`${url}`,
		createJSONRPCReq(1, organizations.Vaccines, [ParamData]),
	);
};

export const OrganizationSync = async (p: LastUpdatedInput) => {
	return slimJsonrpcTokenPost<SyncData, unknown>(
		`${url}`,
		createJSONRPCReq(1, organizations.Sync, [
			{
				...p,
			},
		]),
	);
};

export const getDrugs = async () => {
	const ParamData = {};

	return slimJsonrpcTokenPost<string[], unknown>(
		`${url}`,
		createJSONRPCReq(1, organizations.Drugs, [ParamData]),
	);
};
