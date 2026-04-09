import { useMutation } from "@tanstack/react-query";
import { Fade, useToast } from "@chakra-ui/react";
import { useNavigate } from "@tanstack/react-router";
import { Login } from "@repo/ui/forms";
import Photo from "../assets/Login.png";
import { useAuthStore } from "../atoms/auth-store";
import axios, { type AxiosError } from "axios";
import { useAtom } from "jotai";
import { isSystemadmin, isSuperadmin } from "../atoms/roles";
import type { AuthLogin } from "../types/dto";

type ErrorResponseData = {
	message: string;
	errorCode: number;
};

type MyAxiosError = AxiosError<ErrorResponseData>;

const LoginSubmit = () => {
	const [, setsuperadmin] = useAtom(isSuperadmin);
	const [, setsystemadmin] = useAtom(isSystemadmin);
	const navigate = useNavigate();
	const toast = useToast();
	const { updateToken } = useAuthStore();
	const mutation = useMutation({
		mutationFn: async (loginData: AuthLogin) =>
			await axios.post("http://localhost:8080/login", loginData),
		onSuccess: (data) => {
			if (data) {
				console.log(data, "this is my login data");
				updateToken(data?.data?.token);
				if (data?.data?.role.includes("superadmin")) {
					toast({
						title: "Successfully logged in",
						description: "Welcome!",
						status: "success",
						duration: 9000,
						isClosable: true,
					});
					setsuperadmin(true);
					setsystemadmin(false);
					navigate({ to: "/dashboard" });
				} else if (data?.data?.role.includes("systemadmin")) {
					toast({
						title: "Successfully logged in",
						description: "Welcome!",
						status: "success",
						duration: 9000,
						isClosable: true,
					});
					setsuperadmin(false);
					setsystemadmin(true);
					navigate({ to: "/dashboard" });
				}
			} else {
				console.log("No Data Found");
			}
		},
		onError: (error) => {
			if (axios.isAxiosError(error)) {
				const axiosError: MyAxiosError = error;

				if (axiosError?.response) {
					console.log(
						axiosError?.response?.data?.message,
						"this is my login error",
					);
					toast({
						title: "Error",
						description: axiosError?.response?.data?.message,
						status: "error",
						duration: 9000,
						isClosable: true,
					});
				} else {
					toast({
						title: "Error",
						description: "Some Error",
						status: "error",
						duration: 9000,
						isClosable: true,
					});
				}
			} else {
				toast({
					title: "Error",
					description: "Invalid Login Data",
					status: "error",
					duration: 9000,
					isClosable: true,
				});
			}
		},
	});
	return (
		<div>
			<Fade in={true}>
				<Login
					onSubmit={(l: AuthLogin) => {
						mutation.mutate(l);
						console.log(l);
					}}
					image_url={Photo}
				/>
			</Fade>
		</div>
	);
};

export default LoginSubmit;
