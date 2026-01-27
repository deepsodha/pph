import { Box, Center } from "@chakra-ui/react";
import DashboardCard from "../../components/dashboard-card";

const FinishedAppointmentTab = () => {
	return (
		<div>
			<Box>
				<Box>
					<Center>
						<Box
							cursor={"pointer"}
							width={{ md: "80%", base: "90%", lg: "70%" }}
						>
							<DashboardCard
								heading_1={"Patient Name:"}
								result_1={"Dummy"}
								heading_2={"Visit Type:"}
								result_2={"Data"}
								editpath={""}
							/>
						</Box>
					</Center>
				</Box>
			</Box>
		</div>
	);
};

export default FinishedAppointmentTab;
