import type {
	AppointmentAdd,
	AppointmentDelete,
	AppointmentUpdate,
	SyncData,
} from "@repo/types/dto";
import { url } from "./api";
import { createJSONRPCReq, appointment, slimJsonrpcTokenPost } from ".";

export const addAppointment = async (p: AppointmentAdd) => {
	return slimJsonrpcTokenPost<SyncData, unknown>(
		`${url}`,
		createJSONRPCReq(1, appointment.Add, [
			{
				...p,
			},
		]),
	);
};

export const deleteAppointment = async (data: AppointmentDelete) => {
	const paramData = {
		id: data.id,
		patient_id: data.patient_id,
		doctor_id: data.doctor_id,
		last_updated_input: data.last_updated_input,
	};
	return slimJsonrpcTokenPost<SyncData, unknown>(
		`${url}`,
		createJSONRPCReq(1, appointment.Delete, [paramData]),
	);
};

export const updateAppointment = async (data: AppointmentUpdate) => {
	return slimJsonrpcTokenPost<SyncData, unknown>(
		`${url}`,
		createJSONRPCReq(1, appointment.Update, [data]),
	);
};
