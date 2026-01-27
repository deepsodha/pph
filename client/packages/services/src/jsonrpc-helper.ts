import axios from "axios";
import {
	type JSONRPCReq,
	type SlimJSONRPCRes,
	toSlipJSONRPCRes,
} from "./jsonrpc-type";
import type { JSONRPCRes } from "./jsonrpc-type";
import { invoke } from "@tauri-apps/api/core";

const JSONRPCVERSION = "2.0";

export const jsonrpcPost = async <T, E>(
	url: string,
	data: JSONRPCReq,
): Promise<JSONRPCRes<T, E>> => {
	try {
		const res = await axios.post(url, data);
		return res.data as JSONRPCRes<T, E>;
	} catch (ex) {
		return {
			jsonrpc: JSONRPCVERSION,
			error: {
				code: -1,
				data: ex,
				message:
					"There is some unknown network error. Please try again. If error persist contact admin",
			},
			id: data.id,
		} as JSONRPCRes<T, E>;
	}
};

export const jsonrpcWithTokenPost = async <T, E>(
	url: string,
	data: JSONRPCReq,
): Promise<JSONRPCRes<T, E>> => {
	try {
		const token = () => localStorage.getItem("token");

		const isDesktopOnly = false; // Assuming this value will be dynamically set
		let response: JSONRPCRes<T, E>;

		if (isDesktopOnly) {
			const body = JSON.stringify(data);
			response = await invoke("rpc_handler", { body, token: token() });
		} else {
			const headers = token()
				? { headers: { Authorization: `Bearer ${token()}` } }
				: {};
			const res = await axios.post(url, data, headers);
			response = res.data as JSONRPCRes<T, E>;
		}

		return response;
	} catch (ex) {
		return {
			jsonrpc: JSONRPCVERSION,
			error: {
				code: -1,
				data: ex,
				message:
					"There is some unknown network error. Please try again. If error persists, contact admin.",
			},
			id: data.id,
		} as JSONRPCRes<T, E>;
	}
};

export const slimJsonrpcPost = async <T, E>(
	url: string,
	data: JSONRPCReq,
): Promise<SlimJSONRPCRes<T, E>> => {
	return toSlipJSONRPCRes(await jsonrpcPost(url, data));
};

export const slimJsonrpcTokenPost = async <T, E>(
	url: string,
	data: JSONRPCReq,
): Promise<SlimJSONRPCRes<T, E>> => {
	return toSlipJSONRPCRes(await jsonrpcWithTokenPost(url, data));
};
