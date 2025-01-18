import { useMutation } from "@tanstack/react-query";
import { useToast } from "@chakra-ui/react";
import { useAtom } from "jotai";
import { UsersForm } from "@repo/ui/forms";
import {
	addValue,
	dashboardValue,
	formValue,
	headerText,
} from "../atoms/header";
import type { Users } from "@repo/types/dto";
import { useNavigate, useParams, useRouter } from "@tanstack/react-router";

import { useMountEffect } from "@react-hookz/web";
import { update_user } from "../services/user";
import { GetUserDataFn } from "../query-mutation-fn/user";

const UserEditForm = () => {
	const [, setHeaderText] = useAtom(headerText);
	const [, setAddValue] = useAtom(addValue);
	const [, setFormValue] = useAtom(formValue);
	const [, setDashboardValue] = useAtom(dashboardValue);
	const router = useRouter();
	const navigate = useNavigate();
	const UserData = GetUserDataFn();
	const userId = useParams({
		from: "/user/edit/$userId",
		select: (params) => params.userId,
	});
	const filteredData = UserData.data?.find((item) => item.id === userId);

	useMountEffect(() => {
		setHeaderText("Add New User");
		setAddValue(true);
		setFormValue(true);
		setDashboardValue(false);
	});

	const toast = useToast();

	const mutation = useMutation({
		mutationFn: (userData: Users) => update_user(userId, userData),
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
		<UsersForm
			onSubmit={(u: Users): void => {
				console.log(u);
				mutation.mutate(u);
			}}
			edit={true}
			initialValues={filteredData}
			UserId={userId}
		/>
	);
};

export default UserEditForm;
