import { useMutation } from "@tanstack/react-query";
import { Box, Fade, useToast } from "@chakra-ui/react";
import { baseUrl, type JSONRPCError, login } from "@repo/services/src";
import type { LoginMobile } from "@repo/types/dto";
import { useNavigate } from "@tanstack/react-router";
import { Login } from "@repo/ui/forms";
import Photo from "../../assets/Login.png";
import { useAuthStore } from "../../store/store";
import InformationModal from "./information-modal";
import { useEffect, useState } from "react";
import axios from "axios";
import { useAtom } from "jotai";
import {
	isdoctor,
	isofficestaff,
	issuperadmin,
	issystemadmin,
} from "../../atoms/roles";

type T = {
	userId: number;
	username: string;
};
const LoginSubmit = () => {
	const [, setsuperadmin] = useAtom(issuperadmin);
	const [, setsystemadmin] = useAtom(issystemadmin);
	const [, setdoctor] = useAtom(isdoctor);
	const [, setofficestaff] = useAtom(isofficestaff);
	const [data, setData] = useState();
	// const token = localStorage.getItem("token");
	// const covertedtoken = JSON.stringify(token as string);
	useEffect(() => {
		axios.post(`${baseUrl()}/phone_code`).then((res) => setData(res.data));
	}, []);
	const navigate = useNavigate();
	const toast = useToast();
	const { updateToken } = useAuthStore();
	const mutation = useMutation({
		mutationFn: login,
		onSuccess: (data) => {
			console.log(data.result?.role);

			if (data.result) {
				updateToken(data.result?.token);
				if (data.result?.role.includes("superadmin")) {
					toast({
						title: "Successfully logged in",
						description: "Welcome!",
						status: "success",
						duration: 9000,
						isClosable: true,
					});
					setsuperadmin(true);
					setsystemadmin(false);
					setdoctor(false);
					setofficestaff(false);
					navigate({ to: "/dashboard" });
				} else if (
					data.result?.role.includes("systemadmin") &&
					data.result?.role.includes("doctor") &&
					data.result?.role.includes("officestaff") &&
					!data.result?.role.includes("superadmin")
				) {
					toast({
						title: "Successfully logged in",
						description: "Welcome!",
						status: "success",
						duration: 9000,
						isClosable: true,
					});
					setsuperadmin(false);
					setsystemadmin(true);
					setdoctor(true);
					setofficestaff(true);
					console.log("step 1");
					navigate({ to: "/hospital/list" });
				} else if (
					data.result?.role.includes("systemadmin") &&
					data.result?.role.includes("doctor") &&
					!data.result?.role.includes("officestaff") &&
					!data.result?.role.includes("superadmin")
				) {
					toast({
						title: "Successfully logged in",
						description: "Welcome!",
						status: "success",
						duration: 9000,
						isClosable: true,
					});
					setsuperadmin(false);
					setsystemadmin(true);
					setdoctor(true);
					setofficestaff(false);
					console.log("step 2");
					navigate({ to: "/hospital/list" });
				} else if (
					data.result?.role.includes("systemadmin") &&
					data.result?.role.includes("officestaff") &&
					!data.result?.role.includes("doctor") &&
					!data.result?.role.includes("superadmin")
				) {
					toast({
						title: "Successfully logged in",
						description: "Welcome!",
						status: "success",
						duration: 9000,
						isClosable: true,
					});
					setsuperadmin(false);
					setsystemadmin(true);
					setdoctor(false);
					setofficestaff(true);
					console.log("step 3");
					navigate({ to: "/hospital/list" });
				} else if (
					data.result?.role.includes("doctor") &&
					data.result?.role.includes("officestaff") &&
					!data.result?.role.includes("systemadmin") &&
					!data.result?.role.includes("superadmin")
				) {
					toast({
						title: "Successfully logged in",
						description: "Welcome!",
						status: "success",
						duration: 9000,
						isClosable: true,
					});
					setsuperadmin(false);
					setsystemadmin(false);
					setdoctor(true);
					setofficestaff(true);
					console.log("Step 4");
					navigate({ to: "/hospital/list" });
				} else if (
					data.result?.role.includes("systemadmin") &&
					!data.result?.role.includes("officestaff") &&
					!data.result?.role.includes("doctor") &&
					!data.result?.role.includes("superadmin")
				) {
					toast({
						title: "Successfully logged in",
						description: "Welcome!",
						status: "success",
						duration: 9000,
						isClosable: true,
					});
					setsuperadmin(false);
					setsystemadmin(true);
					setdoctor(false);
					setofficestaff(false);
					navigate({ to: "/dashboard" });
				} else if (
					data.result?.role.includes("officestaff") &&
					!data.result?.role.includes("systemadmin") &&
					!data.result?.role.includes("doctor") &&
					!data.result?.role.includes("superadmin")
				) {
					toast({
						title: "Successfully logged in",
						description: "Welcome!",
						status: "success",
						duration: 9000,
						isClosable: true,
					});
					setsuperadmin(false);
					setsystemadmin(false);
					setdoctor(false);
					setofficestaff(true);
					console.log("Step 5");
					navigate({ to: "/hospital/list" });
				} else if (
					data.result?.role.includes("doctor") &&
					!data.result?.role.includes("officestaff") &&
					!data.result?.role.includes("systemadmin") &&
					!data.result?.role.includes("superadmin")
				) {
					toast({
						title: "Successfully logged in",
						description: "Welcome!",
						status: "success",
						duration: 9000,
						isClosable: true,
					});
					setsuperadmin(false);
					setsystemadmin(false);
					setdoctor(true);
					setofficestaff(false);
					console.log("Step 4");
					navigate({ to: "/hospital/list" });
				}
			} else if (data.error) {
				toast({
					description: data.error.message,
					status: "error",
					duration: 3000,
				});
			}
		},
		onError: (error: JSONRPCError<T> | unknown) => {
			console.error("Error from server:", error);

			const errorMessage =
				(error as JSONRPCError<T>)?.message || "An error occurred";
			toast({
				title: "Error",
				description: errorMessage,
				status: "error",
				duration: 9000,
				isClosable: true,
			});
		},
	});

	// TODO : to be implemented
	// useMountEffect(() => {
	// 	if (token !== null) {
	// 		navigate({ to: "/dashboard" });
	// 	} else {
	// 		navigate({ to: "/" });
	// 	}
	// });
	return (
		<div>
			<Fade in={true}>
				<Login
					onSubmit={(l: LoginMobile) => {
						mutation.mutate(l);
					}}
					image_url={Photo}
					phonecode={data}
				/>
				<Box _hover={{ cursor: "pointer" }}>
					<InformationModal />
				</Box>
			</Fade>
		</div>
	);
};

export default LoginSubmit;
