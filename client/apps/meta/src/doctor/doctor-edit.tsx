import { useMountEffect } from "@react-hookz/web";
import { useRouter, useNavigate, useParams } from "@tanstack/react-router";
import { useAtom } from "jotai";
import {
	headerText,
	addValue,
	formValue,
	dashboardValue,
} from "../atoms/header";
import { useMutation } from "@tanstack/react-query";
import { update_doctor } from "../services/doctor";
import type { Doctors } from "@repo/types/dto";
import { useToast } from "@chakra-ui/react";
import DoctorAddForm from "../../../../packages/ui/forms/meta-doctor-form";
import { GetDoctorDataFn } from "../query-mutation-fn/doctor";

const DoctorEdit = () => {
	const [, setHeaderText] = useAtom(headerText);
	const [, setAddValue] = useAtom(addValue);
	const [, setFormValue] = useAtom(formValue);
	const [, setDashboardValue] = useAtom(dashboardValue);
	const router = useRouter();
	const navigate = useNavigate();
	const toast = useToast();
	useMountEffect(() => {
		setHeaderText("Edit Doctor");
		setAddValue(true);
		setFormValue(true);
		setDashboardValue(false);
	});

	console.log("doctor edit form");

	const doctorId = useParams({
		from: "/doctor/edit/$doctorId",
		select: (params) => params.doctorId,
	});

	const DoctorData = GetDoctorDataFn();
	console.log(DoctorData.data, "Doctor data....");
	const filteredData = DoctorData.data?.find((item) => item.id === doctorId);

	const mutation = useMutation({
		mutationFn: (doctorData: Doctors) => update_doctor(doctorId, doctorData),
		onSuccess: (_data) => {
			toast({
				title: "Doctor Edited",
				description: "doctor is edited",
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
		<div>
			<DoctorAddForm
				onSubmit={(p: Doctors): void => {
					console.log(p, "values submitted");
					mutation.mutate(p);
				}}
				initialValues={filteredData}
				edit={true}
			/>
		</div>
	);
};

export default DoctorEdit;
