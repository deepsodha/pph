import { auth_url } from "./api";
import type { AuthenticatedUser, LoginMobile } from "@repo/types/dto";

import { slimJsonrpcPost, auth, createJSONRPCReq } from ".";

export const login = async (l: LoginMobile) => {
	return slimJsonrpcPost<AuthenticatedUser, unknown>(
		`${auth_url}`,
		createJSONRPCReq(1, auth.LoginMobile, [
			{
				...l,
			},
		]),
	);
};
