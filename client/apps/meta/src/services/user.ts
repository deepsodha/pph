import axios from "axios";
import type { GetUsers, Users } from "../types/dto";

export const add_user = async (u: Users) => {
	const token = localStorage.getItem("token");
	console.log(token, "tk");

	const result = await axios.post<Users>("http://localhost:8080/api/users", u, {
		headers: {
			Authorization: `Bearer ${token} `,
		},
	});
	console.log("added to home", result);
	return result.data;
};
export const get_user = async () => {
	const token = localStorage.getItem("token");

	const result = await axios.get<GetUsers[]>(
		"http://localhost:8080/api/users",
		{
			headers: {
				Authorization: `Bearer ${token} `,
			},
		},
	);
	return result.data;
};
export const update_user = async (userId: string, u: Users) => {
	const token = localStorage.getItem("token");

	// const result = await axios.put("http://localhost:8080/api/users/4e06fedb1b5c4ce9a155050acd288ada", u, {
	const result = await axios.put<Users>(
		`http://localhost:8080/api/users/${userId}`,
		u,
		{
			headers: {
				Authorization: `Bearer ${token} `,
			},
		},
	);
	return result.data;
};
export const delete_user = async (userId: string) => {
	const token = localStorage.getItem("token");

	// const result = await axios.put("http://localhost:8080/api/users/4e06fedb1b5c4ce9a155050acd288ada", u, {
	const result = await axios.delete(
		`http://localhost:8080/api/users/${userId}`,
		{
			headers: {
				Authorization: `Bearer ${token} `,
			},
		},
	);
	return result.data;
};
