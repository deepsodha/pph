import { useAtom } from "jotai";
import { useMountEffect } from "@react-hookz/web";
import {
	addValue,
	dashboardValue,
	formValue,
	headerText,
} from "../atoms/header";
import { useMutation } from "@tanstack/react-query";
import { add_drug } from "../services/drug";
import type { Drugs } from "@repo/types/dto";
import { MetaDrugForm } from "@repo/ui/forms";
import { useRouter, useNavigate } from "@tanstack/react-router";
import { useToast } from "@chakra-ui/react";

const DrugAddForm = () => {
	const [, setHeaderText] = useAtom(headerText);
	const [, setAddValue] = useAtom(addValue);
	const [, setFormValue] = useAtom(formValue);
	const [, setDashboardValue] = useAtom(dashboardValue);

	useMountEffect(() => {
		setHeaderText("Add New Drug");
		setAddValue(true);
		setFormValue(true);
		setDashboardValue(false);
	});
	const toast = useToast();
	const router = useRouter();
	const navigate = useNavigate();
	const mutation = useMutation({
		mutationFn: add_drug,
		onSuccess: (_data) => {
			toast({
				title: "Drug Added",
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
		<MetaDrugForm
			onSubmit={(u: Drugs): void => {
				console.log(u);
				mutation.mutate(u);
			}}
			edit={false}
		/>
	);
};

export default DrugAddForm;
