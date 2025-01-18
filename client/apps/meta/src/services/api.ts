// import axios from "axios";

// export const TokenPost = async <T>(url: string, data: T): Promise<T> => {
// 	const token = () => localStorage.getItem("token");

// 	const headers = token()
// 		? { headers: { Authorization: `Bearer ${token()}` } }
// 		: {};
// 	console.log(headers, "render data.......");
// 	const res = await axios.post(url, data, headers);
// 	console.log(res.data, "data for api....");

// 	return res.data;
// };

// export const LoginPost = async <T>(url: string, data: T): Promise<T> => {
// 	const res = await axios.post(url, data);
// 	console.log(res.data, "data for api....");

// 	return res.data;
// };

export const base_url = import.meta.env.VITE_BASE_URL;

export const login_url = `${base_url}/login`;
export const user_url = `${base_url}/api/users`;
