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
import ActiveInactiveMedicationListTab from "./active-inactive-medication-list-tab";
import type { MedicationsStateExtend } from "@repo/types/dexie-state";

type MedicationListProps = {
	filterData: MedicationsStateExtend[];
	selectedDate: Date | null;
};
const MedicationListTabs = (props: MedicationListProps) => {
	return (
		<Fade in={true}>
			<Flex direction="column" minHeight={"100vh"} bgColor={"#F7F7F9"}>
				<Tabs variant="unstyled">
					<Box position="sticky" top={180} zIndex={10} bgColor={"#F7F7F9"}>
						<TabList
							display={"Flex"}
							justifyContent={"space-around"}
							alignItems={"center"}
						>
							<Tab mx={6}>Active</Tab>
							<Tab mx={6}>InActive</Tab>
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
							<ActiveInactiveMedicationListTab
								filterData={props.filterData}
								active={true}
								selectedDate={props.selectedDate}
							/>
						</TabPanel>
						<TabPanel flex="1">
							<ActiveInactiveMedicationListTab
								filterData={props.filterData}
								active={false}
								selectedDate={props.selectedDate}
							/>
						</TabPanel>
					</TabPanels>
				</Tabs>
			</Flex>
		</Fade>
	);
};
export default MedicationListTabs;
