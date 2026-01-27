import {
	Tabs,
	TabList,
	Tab,
	TabIndicator,
	TabPanels,
	TabPanel,
	Box,
	Fade,
	Flex,
} from "@chakra-ui/react";
import ScheduledFinishedApppointmentTab from "./ScheduledFinishedApppointmentTab";
import type { AppointmentStateExtend } from "@repo/types/dexie-state";

export type NewAppointmentProps = {
	filterData: AppointmentStateExtend[];
};

const AppointmentTabs = (props: NewAppointmentProps) => {
	return (
		<Fade in={true}>
			<Flex direction="column" minHeight={"100vh"} bgColor={"#F7F7F9"}>
				<Tabs variant="unstyled">
					<Box position={"sticky"} top={0} zIndex={10} bgColor={"#F7F7F9"}>
						<TabList
							display={"Flex"}
							justifyContent={"space-around"}
							alignItems={"center"}
						>
							<Tab mx={6}>Scheduled</Tab>
							<Tab mx={6}>Completed</Tab>
						</TabList>
						<TabIndicator
							mt="-3px"
							height="2px"
							bg="blue.500"
							borderRadius="1px"
						/>
					</Box>
					<TabPanels>
						<TabPanel flex="1">
							<ScheduledFinishedApppointmentTab
								filterData={props.filterData}
								scheduled={true}
							/>
						</TabPanel>
						<TabPanel flex="1">
							<ScheduledFinishedApppointmentTab
								filterData={props.filterData}
								scheduled={false}
							/>
						</TabPanel>
					</TabPanels>
				</Tabs>
			</Flex>
		</Fade>
	);
};

export default AppointmentTabs;
