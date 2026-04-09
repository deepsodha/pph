// Create toast message based on JSONRPC Error

import type { JSONRPCError } from "@repo/services/src";

export const toastError = <T,>(error: JSONRPCError<T>) => {
	return <div>{error.code}</div>;
};
