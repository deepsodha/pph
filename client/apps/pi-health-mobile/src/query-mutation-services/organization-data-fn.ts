import {
	type UseMutationResult,
	type UseQueryResult,
	useMutation,
	useQuery,
} from "@tanstack/react-query";
import {
	type SlimJSONRPCRes,
	addOrganization,
	deleteOrganization,
	getDrugs,
	getOrganization,
	getOrganizationById,
	getOrganizationCity,
	getOrganizationCountry,
	getOrganizationState,
	getVaccines,
	organization_select,
	updateOrganization,
	getOrganizationPinCode,
	getOrganizationLocationPinCode,
} from "@repo/services/src";
import { useToast } from "@chakra-ui/react";
import { useNavigate, useRouter } from "@tanstack/react-router";
import type {
	AuthenticatedUser,
	Getorganization,
	OrganizationAdd,
	OrganizationDelete,
	OrganizationUpdate,
	SelectOrganization,
	SyncData,
} from "@repo/types/dto";
import { useAuthStore } from "../store/store";

export function GetOrganizationDataFn(): UseQueryResult<
	SlimJSONRPCRes<Getorganization[], unknown>,
	Error
> {
	return useQuery({
		queryKey: ["GetOrganization"],
		queryFn: () => getOrganization(),
	});
}

export function GetOrganizationByIdDataFn(
	id: string,
): UseQueryResult<SlimJSONRPCRes<Getorganization[], unknown>, Error> {
	return useQuery({
		queryKey: ["GetOrganizationById", id],
		queryFn: () => getOrganizationById(id),
	});
}

export function OrganizationSelectDataFn(): UseMutationResult<
	SlimJSONRPCRes<AuthenticatedUser, unknown>,
	Error,
	SelectOrganization,
	unknown
> {
	const toast = useToast();
	const { updateToken } = useAuthStore();
	return useMutation({
		mutationFn: organization_select,
		onSuccess: (data) => {
			if (data.result) {
				updateToken(data.result?.token);

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

export function GetOrganizationCountryDataFn() {
	return useQuery({
		queryKey: ["GetOrganizationCountry"],
		queryFn: () => getOrganizationCountry(),
	});
}

export function GetOrganizationPinCodeDataFn() {
	return useQuery({
		queryKey: ["GetOrganizationPinCode"],
		queryFn: () => getOrganizationPinCode(),
	});
}

export function GetOrganizationLocationPinCodeDataFn(pin_code: string) {
	return useQuery({
		queryKey: ["GetOrganizationLocationPinCode"],
		queryFn: () => getOrganizationLocationPinCode(pin_code),
	});
}

export function GetOrganizationStateDataFn(country: string | null) {
	return useQuery({
		queryKey: ["GetOrganizationState", country],
		queryFn: () => getOrganizationState(country),
	});
}

export function GetOrganizationCityDataFn(
	state_name: string,
	country_name: string,
) {
	return useQuery({
		queryKey: ["GetOrganizationCity", state_name, country_name],
		queryFn: () => getOrganizationCity(state_name, country_name),
	});
}
export function GetVaccineDataFn() {
	return useQuery({
		queryKey: ["GetVaccine"],
		queryFn: () => getVaccines(),
	});
}

export function AddOrganizationDataFn(): UseMutationResult<
	SlimJSONRPCRes<OrganizationAdd, unknown>,
	Error,
	OrganizationAdd,
	unknown
> {
	const toast = useToast();
	const navigate = useNavigate();
	const router = useRouter();
	return useMutation({
		mutationKey: ["AddOrganization"],
		mutationFn: addOrganization,
		onSuccess: async (data) => {
			if (data.result) {
				navigate({ to: router.history.back() });
				toast({
					description: "Success",
					status: "success",
					duration: 3000,
				});
			} else if (data.error) {
				toast({
					description: data.error.message,
					status: "error",
					duration: 3000,
				});
			}
		},
		onError: () => {},
	});
}

export function UpdateOrganizationDataFn(): UseMutationResult<
	SlimJSONRPCRes<OrganizationUpdate, unknown>,
	Error,
	OrganizationUpdate,
	unknown
> {
	const toast = useToast();
	const navigate = useNavigate();
	const router = useRouter();
	return useMutation({
		mutationKey: ["UpdateOrganization"],
		mutationFn: updateOrganization,
		onSuccess: async (data) => {
			if (data.result) {
				navigate({ to: router.history.back() });
				toast({
					description: "Success",
					status: "success",
					duration: 3000,
				});
			} else if (data.error) {
				toast({
					description: data.error.message,
					status: "error",
					duration: 3000,
				});
			}
		},
		onError: () => {},
	});
}

export function DeleteOrganizationDataFn(): UseMutationResult<
	SlimJSONRPCRes<SyncData, unknown>,
	Error,
	OrganizationDelete,
	unknown
> {
	const toast = useToast();
	return useMutation({
		mutationKey: ["DeleteOrganization"],
		mutationFn: deleteOrganization,
		onSuccess: (data) => {
			if (data.error) {
				toast({
					title: "Delete Failed",
					description: data?.error.message || "Unknown error occurred.",
					status: "error",
					duration: 3000,
					isClosable: true,
				});
			} else if (data.result) {
				toast({
					title: "Delete Success",
					description: "Delete Done Successfully",
					status: "success",
					duration: 3000,
					isClosable: true,
				});
			}
		},
		onError: () => {},
	});
}
export function GetDrugsDataFn() {
	return useQuery({
		queryKey: ["GetDrugs"],
		queryFn: () => getDrugs(),
	});
}
