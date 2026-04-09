import { useToast } from "@chakra-ui/react";
import { useAtom } from "jotai";
import { useMountEffect } from "@react-hookz/web";
import {
	addValue,
	dashboardValue,
	formValue,
	headerText,
} from "../atoms/header";
import type { Clients } from "@repo/types/dto";
import { MetaClientForm } from "@repo/ui/forms";
import { useMutation } from "@tanstack/react-query";
import { useRouter, useNavigate } from "@tanstack/react-router";
import { add_client } from "../services/client";

const ClientAddForm = () => {
	const [, setHeaderText] = useAtom(headerText);
	const [, setAddValue] = useAtom(addValue);
	const [, setFormValue] = useAtom(formValue);
	const [, setDashboardValue] = useAtom(dashboardValue);

	useMountEffect(() => {
		setHeaderText("Add New Client");
		setAddValue(true);
		setFormValue(true);
		setDashboardValue(false);
	});
	const toast = useToast();
	const router = useRouter();
	const navigate = useNavigate();
	const mutation = useMutation({
		mutationFn: add_client,
		onSuccess: (_data) => {
			toast({
				title: "Client Added",
				description: "Drug is added",
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
		<MetaClientForm
			onSubmit={(u: Clients): void => {
				console.log(u);
				mutation.mutate(u);
			}}
			edit={false}
		/>
	);
};

export default ClientAddForm;
