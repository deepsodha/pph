import { Box, Center, Heading, Image, InputGroup } from "@chakra-ui/react";
import { useState } from "react";
import DatePicker from "react-datepicker";
import "react-datepicker/dist/react-datepicker.css";
import Calender from "../assets/Calander Icon.svg";

type CalenderBarProps = {
	heading: string;
	onSelectDate: (date: Date) => void;
};

const CalenderBar = ({ heading, onSelectDate }: CalenderBarProps) => {
	const [selectedDate, setSelectedDate] = useState<Date | null>(null);
	const [isDatePickerOpen, setIsDatePickerOpen] = useState(false);

	const handleDateChange = (date: Date | null) => {
		setSelectedDate(date);
		if (date !== null) {
			onSelectDate(date);
		}
		setIsDatePickerOpen(false);
	};

	return (
		<div>
			<Box mt={4}>
				<Center>
					<InputGroup
						border="1px"
						width="100%"
						height={{
							md: "50px",
							base: "50px",
							lg: "50px",
							xl: "50px",
						}}
						rounded="md"
						borderColor="#095FBA"
						bgColor="#095FBA"
						display={"flex"}
						alignItems={"center"}
						justifyItems={"center"}
						justifyContent={"center"}
						alignContent={"center"}
					>
						<Heading
							_placeholder={{ opacity: 1, color: "#FFFFFF" }}
							bgColor="white"
							border="1px"
							rounded="md"
							height="95%"
							borderColor="#095FBA"
							color="#095FBA"
							fontSize={{ base: "medium", md: "large" }}
							fontWeight="bold"
							pt={2.5}
							pl={3}
							width="100%"
						>
							{selectedDate ? (
								<>
									<span>{"Selected Date"}: </span>
									<span style={{ color: "black", fontWeight: "bold" }}>
										{selectedDate.toLocaleDateString("en-GB")}
									</span>
								</>
							) : (
								heading
							)}
						</Heading>
						<Box
							bg="#095FBA"
							rounded="md"
							width={16}
							height={8}
							cursor={"pointer"}
							onClick={() => setIsDatePickerOpen(!isDatePickerOpen)}
						>
							<Center>
								<Image src={Calender} mt={1} color="#2D3748" />
							</Center>
						</Box>
					</InputGroup>
					{isDatePickerOpen && (
						<Center>
							<DatePicker
								selected={selectedDate}
								onChange={handleDateChange}
								dateFormat="dd/MM/yyyy"
								inline
							/>
						</Center>
					)}
				</Center>
			</Box>
		</div>
	);
};

export default CalenderBar;
