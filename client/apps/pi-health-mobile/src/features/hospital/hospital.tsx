import { Box, Button, Fade, Image, Text } from "@chakra-ui/react";
import { HospitalSelect } from "@repo/ui/forms";
import Hospital from "../../assets/Hospital Selection Illustration.png";
import type { SelectServiceLocation } from "@repo/types/dto";
import { ServiceLocationSelectDataFn } from "../../query-mutation-services/service-location-detail-data-fn";
import { useLiveQuery } from "dexie-react-hooks";
import { db } from "../../db/db";
import { useMountEffect } from "@react-hookz/web";
import { useAtom } from "jotai";
import {
	headerText,
	addValue,
	formValue,
	dashboardValue,
} from "../../atoms/header";
import { Link, useNavigate } from "@tanstack/react-router";
import { AddIcon } from "@chakra-ui/icons";
const selectServiceLocation = () => {
	const mutation = ServiceLocationSelectDataFn();
	const servicelocation = useLiveQuery(() => db.servicelocation.toArray());
	const [, setHeaderText] = useAtom(headerText);
	const [, setAddValue] = useAtom(addValue);
	const [, setFormValue] = useAtom(formValue);
	const [, setDashboardValue] = useAtom(dashboardValue);
	const navigate = useNavigate();
	useMountEffect(() => {
		setHeaderText("Select Hospital");
		setAddValue(true);
		setFormValue(true);
		setDashboardValue(false);
	});

	return (
		<div>
			<Fade in={true}>
				<Box
					display={"flex"}
					justifyContent={"center"}
					flexWrap={"wrap"}
					alignItems={"center"}
				>
					<Box>
						<Image
							width={{ lg: "600px", base: "auto" }}
							height={{ lg: "400px", base: "auto" }}
							objectFit={"cover"}
							src={Hospital}
							mb={{ base: "6%", md: "8%" }}
						/>
					</Box>
					<Box width={{ lg: "400px", base: "100%" }} height={"100%"}>
						<Box
							roundedTopLeft={"2xl"}
							roundedTopRight={"2xl"}
							padding="5%"
							ml={{ lg: "10%" }}
							mr={{ lg: "10%" }}
						>
							<Link to={"/servicelocation/add"}>
								<Button
									width="100%"
									mt="5%"
									backgroundColor="white"
									border="1px"
									borderColor="#095FBA"
									rounded="md"
									fontSize={"16px"}
								>
									<AddIcon color="#095FBA" mr="3%" />
									<Text color="#095FBA">Add Service Location</Text>
								</Button>
							</Link>
							<Text fontSize="lg" textColor="#575858" paddingTop="40px">
								Please select your Hospital <br />
								to continue
							</Text>
							<HospitalSelect
								onSubmit={(o: SelectServiceLocation) => {
									mutation.mutate(o);
									navigate({ to: "/dashboard" });
								}}
								servicelocation={servicelocation}
							/>
						</Box>
					</Box>
				</Box>
			</Fade>
		</div>
	);
};

export default selectServiceLocation;
