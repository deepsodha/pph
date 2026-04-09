import {
	Fade,
	Box,
	Tabs,
	TabList,
	Tab,
	TabIndicator,
	TabPanels,
	TabPanel,
} from "@chakra-ui/react";
import type { AllergiesStateExtend } from "@repo/types/dexie-state";
import ActiveInactiveAllergyListTab from "./active-inactive-allergy-list-tab";
type AllergyListProps = {
	filterData: AllergiesStateExtend[];
};
const AllergyListTabs = (props: AllergyListProps) => {
	return (
		<Fade in={true}>
			<Tabs variant="unstyled">
				<Box position="sticky" top={110} zIndex={50} bgColor={"#F7F7F9"}>
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
						<ActiveInactiveAllergyListTab
							filterData={props.filterData}
							active={true}
						/>
					</TabPanel>
					<TabPanel flex="1">
						<ActiveInactiveAllergyListTab
							filterData={props.filterData}
							active={false}
						/>
					</TabPanel>
				</TabPanels>
			</Tabs>
		</Fade>
	);
};

export default AllergyListTabs;
