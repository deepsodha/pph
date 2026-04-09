import {
	type SlimJSONRPCRes,
	addServiceLoctionDetails,
	deleteServiceLocation,
	updateServiceLocationDetails,
	serviceLocation_select,
} from "@repo/services/src";
import type {
	AuthenticatedUser,
	SelectServiceLocation,
	ServiceLocationAdd,
	ServiceLocationDelete,
	ServiceLocationUpdate,
	SyncData,
} from "@repo/types/dto";
import { MutationReuse } from "./mutation-reuse";
import { type UseMutationResult, useMutation } from "@tanstack/react-query";
import { useToast } from "@chakra-ui/react";
import { useAuthStore } from "../store/store";
import { useAtom } from "jotai";
import { servicelocationid } from "../atoms/header";

export function addServiceLocationDetailsDataFn(): UseMutationResult<
	SlimJSONRPCRes<SyncData, unknown>,
	Error,
	ServiceLocationAdd,
	unknown
> {
	return MutationReuse(
		["AddSerivceLocation"],
		addServiceLoctionDetails,
		"ServiceLocation",
		true,
	);
}
export function UpdateServiceLocationDetailsDataFn(): UseMutationResult<
	SlimJSONRPCRes<SyncData, unknown>,
	Error,
	ServiceLocationUpdate,
	unknown
> {
	return MutationReuse(
		["UpdateServiceLocationDetail"],
		updateServiceLocationDetails,
		"Service Location Update",
		true,
	);
}
export function DeleteServiceLocationDataFn(): UseMutationResult<
	SlimJSONRPCRes<SyncData, unknown>,
	Error,
	ServiceLocationDelete,
	unknown
> {
	return MutationReuse(
		["DeleteServiceLocation"],
		deleteServiceLocation,
		"Delete Service Location",
	);
}
export function ServiceLocationSelectDataFn(): UseMutationResult<
	SlimJSONRPCRes<AuthenticatedUser, unknown>,
	Error,
	SelectServiceLocation,
	unknown
> {
	const toast = useToast();
	const { updateToken } = useAuthStore();
	const [, setservicelocationId] = useAtom(servicelocationid);
	return useMutation({
		mutationFn: serviceLocation_select,
		onSuccess: (data) => {
			if (data.result) {
				updateToken(data.result?.token);
				setservicelocationId(data.result.service_location_id);
				toast({
					title: "Success",
					description: "Hospital Selected Successfully",
					status: "success",
					duration: 3000,
					isClosable: true,
				});
			}
		},
		onError: (error) => {
			toast({
				title: "Error",
				description: error as unknown as string,
				status: "error",
				duration: 3000,
				isClosable: true,
			});
		},
	});
}

// export function GetOrganizationDataFn(): UseQueryResult<
// 	SlimJSONRPCRes<GetServiceLocation[], unknown>,
// 	Error
// > {
// 	return useQuery({
// 		queryKey: ["GetOrganization"],
// 		queryFn: () => getOrganization(),
// 	});
// }
