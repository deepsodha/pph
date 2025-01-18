import { useMutation, useQuery } from "@tanstack/react-query";
import { delete_user, get_user } from "../services/user";
import { useToast } from "@chakra-ui/react";

export function GetUserDataFn() {
	return useQuery({
		queryKey: ["GetUser"],
		queryFn: () => get_user(),
	});
}
export function DeleteUserDataFn() {
	const toast = useToast();
	return useMutation({
		mutationKey: ["DeleteUser"],
		mutationFn: delete_user,
		onSuccess: (_data) => {
			toast({
				title: "User Deleted",
				description: "User is deleted",
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
