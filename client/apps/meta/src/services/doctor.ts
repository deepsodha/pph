import type { Doctors, GetDoctors } from "@repo/types/dto";
import axios from "axios";

export const add_doctor = async (d: Doctors) => {
	const token = localStorage.getItem("token");
	console.log(token, "tk");

	const result = await axios.post<Doctors>(
		"http://localhost:8080/api/doctors",
		d,
		{
			headers: {
				Authorization: `Bearer ${token} `,
			},
		},
	);
	console.log("added to Doctor", result);
	return result.data;
};
export const get_doctor = async () => {
	const token = localStorage.getItem("token");

	const result = await axios.get<GetDoctors[]>(
		"http://localhost:8080/api/doctors",
		{
			headers: {
				Authorization: `Bearer ${token} `,
			},
		},
	);
	return result.data;
};

export const update_doctor = async (doctorID: string, doctor: Doctors) => {
	const token = localStorage.getItem("token");

	const result = await axios.put<Doctors>(
		`http://localhost:8080/api/doctors/${doctorID}`,
		doctor,
		{
			headers: {
				Authorization: `Bearer ${token} `,
			},
		},
	);
	return result.data;
};

export const delete_doctor = async (doctorID: string) => {
	const token = localStorage.getItem("token");

	const result = await axios.delete(
		`http://localhost:8080/api/doctors/${doctorID}`,
		{
			headers: {
				Authorization: `Bearer ${token} `,
			},
		},
	);
	return result.data;
};
