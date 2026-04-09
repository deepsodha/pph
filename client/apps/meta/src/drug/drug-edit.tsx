import { useAtom } from "jotai";
import { useMountEffect } from "@react-hookz/web";
import {
	addValue,
	dashboardValue,
	formValue,
	headerText,
} from "../atoms/header";
import { useMutation } from "@tanstack/react-query";
import { update_drug } from "../services/drug";
import type { Drugs } from "@repo/types/dto";
import { MetaDrugForm } from "@repo/ui/forms";
import { useRouter, useNavigate, useParams } from "@tanstack/react-router";
import { useToast } from "@chakra-ui/react";
import { GetDrugDataFn } from "../query-mutation-fn/drug";

const DrugEditForm = () => {
	const [, setHeaderText] = useAtom(headerText);
	const [, setAddValue] = useAtom(addValue);
	const [, setFormValue] = useAtom(formValue);
	const [, setDashboardValue] = useAtom(dashboardValue);

	useMountEffect(() => {
		setHeaderText("Edit Drug");
		setAddValue(true);
		setFormValue(true);
		setDashboardValue(false);
	});
	const toast = useToast();
	const router = useRouter();
	const navigate = useNavigate();

	const drugId = useParams({
		from: "/drug/edit/$drugId",
		select: (params) => params.drugId,
	});
	const DrugData = GetDrugDataFn();
	const filteredData = DrugData.data?.find((item) => item.id === drugId);
	const mutation = useMutation({
		mutationFn: (drugData: Drugs) => update_drug(drugId, drugData),
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
		onError: (error) => {
			if (error) {
				// Extract error information
				const errorMessage = error.message;
				const errorCode = error.message;

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
		<MetaDrugForm
			onSubmit={(u: Drugs): void => {
				console.log(u);
				mutation.mutate(u);
			}}
			edit={true}
			initialValues={filteredData}
		/>
	);
};

export default DrugEditForm;
