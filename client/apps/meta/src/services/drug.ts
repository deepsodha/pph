import axios from "axios";
import type { Drugs, GetDrugs } from "@repo/types/dto";

export const add_drug = async (u: Drugs) => {
	const token = localStorage.getItem("token");
	console.log(token, "tk");

	const result = await axios.post<Drugs>("http://localhost:8080/api/drugs", u, {
		headers: {
			Authorization: `Bearer ${token} `,
		},
	});
	console.log("added to home", result);
	return result.data;
};
export const get_drug = async () => {
	const token = localStorage.getItem("token");

	const result = await axios.get<GetDrugs[]>(
		"http://localhost:8080/api/drugs",
		{
			headers: {
				Authorization: `Bearer ${token} `,
			},
		},
	);
	return result.data;
};
export const update_drug = async (drugId: string, u: Drugs) => {
	const token = localStorage.getItem("token");

	const result = await axios.put<Drugs>(
		`http://localhost:8080/api/drugs/${drugId}`,
		u,
		{
			headers: {
				Authorization: `Bearer ${token} `,
			},
		},
	);
	return result.data;
};
export const delete_drug = async (drugId: string) => {
	const token = localStorage.getItem("token");

	const result = await axios.delete(
		`http://localhost:8080/api/drugs/${drugId}`,
		{
			headers: {
				Authorization: `Bearer ${token} `,
			},
		},
	);
	return result.data;
};
