import {
	Tabs,
	TabList,
	Tab,
	TabIndicator,
	TabPanels,
	TabPanel,
	Box,
	Fade,
} from "@chakra-ui/react";
import ActiveInactiveProblemListTab from "./active-inactive-problem-list-tab";
import type { ProblemStateExtend } from "@repo/types/dexie-state";

type ProblemListProps = {
	filterData: ProblemStateExtend[];
};
const ProblemListTabs = (props: ProblemListProps) => {
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
						<ActiveInactiveProblemListTab
							filterData={props.filterData}
							active={true}
						/>
					</TabPanel>
					<TabPanel flex="1">
						<ActiveInactiveProblemListTab
							filterData={props.filterData}
							active={false}
						/>
					</TabPanel>
				</TabPanels>
			</Tabs>
		</Fade>
	);
};
export default ProblemListTabs;
