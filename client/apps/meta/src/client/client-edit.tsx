import { useMutation } from "@tanstack/react-query";
import { useToast } from "@chakra-ui/react";
import { useAtom } from "jotai";
import { MetaClientForm } from "@repo/ui/forms";
import {
	addValue,
	dashboardValue,
	formValue,
	headerText,
} from "../atoms/header";
import type { Clients } from "@repo/types/dto";
import { useNavigate, useParams, useRouter } from "@tanstack/react-router";

import { useMountEffect } from "@react-hookz/web";
import { update_client } from "../services/client";
import { GetClientDataFn } from "../query-mutation-fn/client";

const ClientEditForm = () => {
	const [, setHeaderText] = useAtom(headerText);
	const [, setAddValue] = useAtom(addValue);
	const [, setFormValue] = useAtom(formValue);
	const [, setDashboardValue] = useAtom(dashboardValue);
	const router = useRouter();
	const navigate = useNavigate();
	const clientId = useParams({
		from: "/client/edit/$clientId",
		select: (params) => params.clientId,
	});
	const ClientData = GetClientDataFn();
	console.log(ClientData.data, "client data....");
	const filteredData = ClientData.data?.find((item) => item.id === clientId);
	useMountEffect(() => {
		setHeaderText("Edit Client");
		setAddValue(true);
		setFormValue(true);
		setDashboardValue(false);
	});

	const toast = useToast();

	const mutation = useMutation({
		mutationFn: (clientData: Clients) => update_client(clientId, clientData),
		onSuccess: (_data) => {
			toast({
				title: "User Added",
				description: "User is added",
				status: "success",
				duration: 9000,
				isClosable: true,
			});
			navigate({ to: router.history.back() });
		},
		onError: (error) => {
			// Check if the error is an Axios error
			if (error) {
				// Extract error information
				const errorMessage = error.message;
				const errorCode = error.message;

				// Render the error message
				toast({
					title: `Error ${errorCode}`,
					description: errorMessage,
					status: "error",
					duration: 9000,
					isClosable: true,
				});
			} else {
				// Render a generic error message
				toast({
					title: "Error",
					description: "An error occurred",
					status: "error",
					duration: 9000,
					isClosable: true,
				});
			}
		},
	});
	return (
		<MetaClientForm
			onSubmit={(u: Clients): void => {
				console.log(u);
				mutation.mutate(u);
			}}
			initialValues={filteredData}
			ClientId={clientId}
			edit={true}
		/>
	);
};

export default ClientEditForm;
