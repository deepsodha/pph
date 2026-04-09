export type JSONRPCReqId = string | number | null;
export type JSONRPCResId = string | number;

export type JSONRPCReq = {
	jsonrpc: string;
	method: string;
	params?: Array<unknown>;
	id: JSONRPCReqId; //Cross check on server // Number shold whole value
};
export const createJSONRPCReq = (
	id: JSONRPCReqId,
	method: string,
	params: Array<unknown> | undefined,
): JSONRPCReq => {
	return {
		jsonrpc: "2.0",
		id,
		method,
		params,
	};
};

type ErrorCodes = -1 /*Client Side or Network Error*/ | -32601 | -32701;

export type JSONRPCError<T> = {
	code: ErrorCodes;
	message: string;
	data?: T;
};

export type JSONRPCRes<T, E> = {
	jsonrpc: string;
	error?: JSONRPCError<E>;
	result?: T;
	id: JSONRPCResId;
};

export type SlimJSONRPCRes<T, E> = {
	error?: JSONRPCError<E>;
	result?: T;
};

export const toSlipJSONRPCRes = <T, E>(
	x: JSONRPCRes<T, E>,
): SlimJSONRPCRes<T, E> => {
	return {
		result: x.result,
		error: x.error,
	};
};
