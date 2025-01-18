import { Box, Center, Fade, Heading, useDisclosure } from "@chakra-ui/react";

import { useLiveQuery } from "dexie-react-hooks";
import { useAtom } from "jotai";
import { useState } from "react";
import AddStaff from "../../assets/Add_staff_icon.svg";
import { useMountEffect } from "@react-hookz/web";
import {
	headerText,
	addImage,
	addPath,
	addValue,
	formValue,
	dashboardValue,
	displayMenu,
} from "../../atoms/header";
import DashboardCardWithImage from "../../components/dashboard-card-with-image";
import DeleteModal from "../../components/delete-modal";
import SearchBar from "../../components/search-bar";
import { db } from "../../db/db";
import { deleteStaffDetailsFn } from "../../query-mutation-services/staff-detail-data-fn";
import StaffDetailsPopUp from "./staff-details-popup";
import { motion, AnimatePresence } from "framer-motion";
const MotionBox = motion(Box);

const StaffList = () => {
	const [, setHeaderText] = useAtom(headerText);
	const [, setaddImage] = useAtom(addImage);
	const [, setaddPath] = useAtom(addPath);
	const [, setAddValue] = useAtom(addValue);
	const [, setFormValue] = useAtom(formValue);
	const [, setmenu] = useAtom(displayMenu);
	const [, setDashboardValue] = useAtom(dashboardValue);

	const [searchQuery, setSearchQuery] = useState("");
	const [deletedata, setDeletedata] = useState("");
	const { isOpen: isDeleteModalOpen, onOpen, onClose } = useDisclosure();

	useMountEffect(() => {
		setHeaderText("Add Staff");
		setaddImage(AddStaff);
		setaddPath("/staff/add");
		setAddValue(true);
		setFormValue(false);
		setDashboardValue(false);
		setmenu(true);
	});
	const {
		isOpen: isModalOpen,
		onOpen: openModalBase,
		onClose: closeModal,
	} = useDisclosure();
	const deleteStaff = deleteStaffDetailsFn();
	const staffData = useLiveQuery(() => db.staff.toArray());
	const [activeId, setActiveId] = useState<string>("");
	const openModal = (id: string) => {
		setActiveId(id);
		openModalBase();
	};
	const filteredData = (staffData || [])
		.reverse()
		.filter((item) => item.user.phone.number?.toString().includes(searchQuery));

	const handleDelete = async (id: string) => {
		try {
			await db.staff.where({ id }).delete();

			await deleteStaff.mutateAsync({
				id,
				last_updated_input: await db.getLastUpdated(),
			});
		} catch (error) {
			console.error("Error deleting complaint:", error);
		}
		onClose();
	};
	const sortedData = filteredData.slice().sort((a, b) => {
		const dateA = new Date(a.created_at);
		const dateB = new Date(b.created_at);
		return dateB.getTime() - dateA.getTime();
	});
	return (
		<div>
			<Fade in={true}>
				<Box minHeight={"100vh"} bgColor={"#F7F7F9"}>
					<Box
						position="sticky"
						zIndex={10}
						bgColor={"#F7F7F9"}
						top={0}
						left={0}
						right={0}
					>
						{/* <Progress size="xs" isIndeterminate={patients.isLoading} /> */}
						<Center>
							<Box width={{ md: "80%", base: "90%", lg: "70%" }}>
								<Heading
									color="#095FBA"
									pt="1"
									fontSize="20px"
									mt="16px"
									mb="10px"
								>
									Staff List
								</Heading>
								<SearchBar
									value={searchQuery}
									onchange={(e) => setSearchQuery(e.target.value)}
									placeholder={"Search by Mobile Number"}
								/>
							</Box>
						</Center>{" "}
					</Box>
					<Box zIndex={15} mt={5} bgColor={"#F7F7F9"} pb="10%">
						{sortedData?.map((item) => (
							<div key={item.created_at}>
								<Center>
									<AnimatePresence>
										<MotionBox
											key={item.created_at}
											initial={{ opacity: 0, y: -50 }}
											animate={{ opacity: 1, y: 0 }}
											exit={{ opacity: 0, y: -50 }}
											transition={{ duration: 0.65 }}
											width={{ md: "80%", base: "90%", lg: "70%" }}
										>
											<DashboardCardWithImage
												profile_img={item.user.user.photo_url}
												name={item.user.user.first_name}
												result={item.user.phone.number}
												heading={"Mobile No:"}
												editpath={`/staff/edit/${item.id}`}
												open_model={() => openModal(item.id)}
												handleDelete={() => {
													setDeletedata(item.id);
													onOpen();
												}}
												emergency={item.emergency}
												gender={item.user.user.gender}
											/>
										</MotionBox>
									</AnimatePresence>
								</Center>
								<StaffDetailsPopUp
									open={isModalOpen}
									close={closeModal}
									id={activeId}
								/>
								<DeleteModal
									open={isDeleteModalOpen}
									close={onClose}
									handle_delete={() => handleDelete(deletedata)}
								/>
							</div>
						))}
					</Box>
				</Box>
			</Fade>
		</div>
	);
};

export default StaffList;
