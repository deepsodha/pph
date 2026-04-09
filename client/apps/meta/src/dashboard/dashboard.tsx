import { Box, Center } from "@chakra-ui/react";
import Boy from "../assets/Boy Icon.svg";
import { useMountEffect } from "@react-hookz/web";
import { useAtom } from "jotai";
import { addValue, formValue, dashboardValue } from "../atoms/header";
import DashboardCard from "../components/dashboard-card";

const dashboard = () => {
	const [, setAddValue] = useAtom(addValue);
	const [, setFormValue] = useAtom(formValue);
	const [, setDashboardValue] = useAtom(dashboardValue);
	useMountEffect(() => {
		setAddValue(true);
		setFormValue(false);
		setDashboardValue(true);
	});
	const DashboardData = [
		{
			paths: "/user/list",
			headings: "Users",
			icon: Boy,
			id: 1,
		},
		{
			paths: "/client/list",
			headings: "Clients",
			icon: Boy,
			id: 2,
		},
		{
			paths: "/doctor/list",
			headings: "Doctors",
			icon: Boy,
			id: 3,
		},
		{
			paths: "/drug/list",
			headings: "Druges",
			icon: Boy,
			id: 4,
		},
	];
	return (
		<div>
			<Box top={0} height={0} minHeight={"100vh"} bgColor={"#F7F7F9"}>
				<Center>
					<Box width={{ md: "80%", base: "90%", lg: "70%" }}>
						<Box width={"100%"}>
							{DashboardData.map((items) => (
								<Box cursor={"pointer"} key={items.id}>
									<DashboardCard
										heading={items.headings}
										img_src={items.icon}
										paths={items.paths}
									/>
								</Box>
							))}
						</Box>
					</Box>
				</Center>
			</Box>
		</div>
	);
};

export default dashboard;
