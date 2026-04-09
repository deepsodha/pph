import { useMutation, useQuery } from "@tanstack/react-query";
import { delete_doctor, get_doctor } from "../services/doctor";
import { useToast } from "@chakra-ui/react";

export function GetDoctorDataFn() {
	return useQuery({
		queryKey: ["GetUser"],
		queryFn: () => get_doctor(),
	});
}

export function DeleteDoctorDataFn() {
	const toast = useToast();
	return useMutation({
		mutationKey: ["DeleteDoctor"],
		mutationFn: delete_doctor,
		onSuccess: (_data) => {
			toast({
				title: "Doctor Deleted",
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
