import axios from "axios";
import type { Clients, GetClients } from "@repo/types/dto";

export const add_client = async (u: Clients) => {
	const token = localStorage.getItem("token");
	console.log(token, "tk");

	const result = await axios.post<Clients>(
		"http://localhost:8080/api/clients",
		u,
		{
			headers: {
				Authorization: `Bearer ${token} `,
			},
		},
	);
	console.log("added to home", result);
	return result.data;
};

export const get_client = async () => {
	const token = localStorage.getItem("token");

	const result = await axios.get<GetClients[]>(
		"http://localhost:8080/api/clients",
		{
			headers: {
				Authorization: `Bearer ${token} `,
			},
		},
	);
	return result.data;
};

export const update_client = async (clientId: string, u: Clients) => {
	const token = localStorage.getItem("token");

	// const result = await axios.put("http://localhost:8080/api/users/4e06fedb1b5c4ce9a155050acd288ada", u, {
	const result = await axios.put<Clients>(
		`http://localhost:8080/api/clients/${clientId}`,
		u,
		{
			headers: {
				Authorization: `Bearer ${token} `,
			},
		},
	);
	return result.data;
};
export const delete_client = async (clientId: string) => {
	const token = localStorage.getItem("token");

	// const result = await axios.put("http://localhost:8080/api/users/4e06fedb1b5c4ce9a155050acd288ada", u, {
	const result = await axios.delete(
		`http://localhost:8080/api/clients/${clientId}`,
		{
			headers: {
				Authorization: `Bearer ${token} `,
			},
		},
	);
	return result.data;
};
