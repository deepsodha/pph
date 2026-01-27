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
import { useNavigate, useRouter } from "@tanstack/react-router";

import { useMountEffect } from "@react-hookz/web";
import { add_user } from "../services/user";

const UserAddForm = () => {
	const [, setHeaderText] = useAtom(headerText);
	const [, setAddValue] = useAtom(addValue);
	const [, setFormValue] = useAtom(formValue);
	const [, setDashboardValue] = useAtom(dashboardValue);
	const router = useRouter();
	const navigate = useNavigate();
	useMountEffect(() => {
		setHeaderText("Add New User");
		setAddValue(true);
		setFormValue(true);
		setDashboardValue(false);
	});

	const toast = useToast();

	const mutation = useMutation({
		mutationFn: add_user,
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
		onError: (error: string) => {
			toast({
				title: "Error",
				description: error as string,
				status: "error",
				duration: 9000,
				isClosable: true,
			});
		},
	});
	return (
		<UsersForm
			onSubmit={(u: Users): void => {
				console.log(u);
				mutation.mutate(u);
			}}
			edit={false}
		/>
	);
};

export default UserAddForm;
