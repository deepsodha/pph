import { Box, Center, Fade } from "@chakra-ui/react";
import Order from "../../../assets/order.svg";
import Historical from "../../../assets/Add Historical Icon 24 x 24.svg";
import Administer from "../../../assets/New Administer Icon 24 x 24.svg";
import Administered from "../../../assets/Not administered Icon 24 x 24.svg";
import PatientDetailImmunizationsCard from "../../../components/patient-details-immunizations-card";
import {
	headerText,
	addValue,
	formValue,
	dashboardValue,
} from "../../../atoms/header";
import { useAtom } from "jotai";
import { useParams } from "@tanstack/react-router";
import { useMountEffect } from "@react-hookz/web";

const Immunizations = () => {
	const [, setHeaderText] = useAtom(headerText);
	const [, setAddValue] = useAtom(addValue);
	const [, setFormValue] = useAtom(formValue);
	const [, setDashboardValue] = useAtom(dashboardValue);
	const { patientId } = useParams({ from: "patientId" });

	useMountEffect(() => {
		setHeaderText("Immunizations");
		setAddValue(true);
		setFormValue(true);
		setDashboardValue(false);
	});

	const ImmunizationsData = [
		{ paths: "/order/list", headings: "New Order", icon: Order, id: 1 },
		{
			paths: "/historical/list",
			headings: "Add Historical",
			icon: Historical,
			id: 2,
		},
		{
			paths: "/administer/list",
			headings: "New Administer",
			icon: Administer,
			id: 3,
		},
		{
			paths: "/notadminister/list",
			headings: "Not Administered",
			icon: Administered,
			id: 4,
		},
	];
	return (
		<div>
			<Fade in={true}>
				<Box bgColor={"#F7F7F9"} height={"100vh"}>
					<Center>
						<Box width={{ md: "80%", base: "90%", lg: "70%" }}>
							{ImmunizationsData.map((items) => (
								<Box cursor={"pointer"} key={items.id}>
									<PatientDetailImmunizationsCard
										heading={items.headings}
										paths={`${items.paths}/${patientId}`}
										img_src={items.icon}
									/>
								</Box>
							))}
						</Box>
					</Center>
				</Box>
			</Fade>
		</div>
	);
};
export default Immunizations;
