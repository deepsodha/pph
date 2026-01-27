import { useMutation, useQuery } from "@tanstack/react-query";
import { delete_client, get_client } from "../services/client";
import { useToast } from "@chakra-ui/react";

export function GetClientDataFn() {
	return useQuery({
		queryKey: ["GetClient"],
		queryFn: () => get_client(),
	});
}

export function DeleteClientDataFn() {
	const toast = useToast();
	return useMutation({
		mutationKey: ["DeleteClient"],
		mutationFn: delete_client,
		onSuccess: (_data) => {
			toast({
				title: "Client Deleted",
				description: "doctor is deleted",
				status: "success",
				duration: 9000,
				isClosable: true,
			});
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
}
