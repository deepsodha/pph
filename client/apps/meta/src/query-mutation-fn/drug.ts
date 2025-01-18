import { useMutation, useQuery } from "@tanstack/react-query";
import { delete_drug, get_drug } from "../services/drug";
import { useToast } from "@chakra-ui/react";

export function GetDrugDataFn() {
	return useQuery({
		queryKey: ["GetDrug"],
		queryFn: () => get_drug(),
	});
}
export function DeleteDrugDataFn() {
	const toast = useToast();
	return useMutation({
		mutationKey: ["DeleteDrug"],
		mutationFn: delete_drug,
		onSuccess: (_data) => {
			toast({
				title: "Drug Deleted",
				description: "Drug is deleted",
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
