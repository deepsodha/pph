import { useMutation } from "@tanstack/react-query";
import { useToast } from "@chakra-ui/react";
import { useAtom } from "jotai";
import {
	addValue,
	dashboardValue,
	formValue,
	headerText,
} from "../atoms/header";
import type { Doctors } from "@repo/types/dto";
import { useNavigate, useRouter } from "@tanstack/react-router";
import { useMountEffect } from "@react-hookz/web";
import { add_doctor } from "../services/doctor";
import DoctorAddForm from "../../../../packages/ui/forms/meta-doctor-form";

const DoctorAdd = () => {
	const [, setHeaderText] = useAtom(headerText);
	const [, setAddValue] = useAtom(addValue);
	const [, setFormValue] = useAtom(formValue);
	const [, setDashboardValue] = useAtom(dashboardValue);
	const router = useRouter();
	const navigate = useNavigate();
	useMountEffect(() => {
		setHeaderText("Add New Doctor");
		setAddValue(true);
		setFormValue(true);
		setDashboardValue(false);
	});

	const toast = useToast();

	const mutation = useMutation({
		mutationFn: add_doctor,
		onSuccess: (_data) => {
			toast({
				title: "Doctor Added",
				description: "Doctor is added",
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
		<DoctorAddForm
			onSubmit={(u: Doctors): void => {
				console.log(u);
				mutation.mutate(u);
			}}
			edit={false}
		/>
	);
};

export default DoctorAdd;
