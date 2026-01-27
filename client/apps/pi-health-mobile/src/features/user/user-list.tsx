import { useMountEffect } from "@react-hookz/web";
import { useAtom } from "jotai";
import UserIcon from "../../assets/User_icon (Blue).svg";

import UserPopUp from "./user-popup";
import {
	headerText,
	addImage,
	addPath,
	addValue,
	formValue,
	displayMenu,
	dashboardValue,
} from "../../atoms/header";
import { Box, Center, Fade, Heading, useDisclosure } from "@chakra-ui/react";
import { useState } from "react";
import SearchBar from "../../components/search-bar";
import { deleteUserFn } from "../../query-mutation-services/user-data-fn";
import { useLiveQuery } from "dexie-react-hooks";
import { db } from "../../db/db";
import DashboardCardWithImage from "../../components/dashboard-card-with-image";
import DeleteModal from "../../components/delete-modal";

import { motion, AnimatePresence } from "framer-motion";
const MotionBox = motion(Box);
const UserList = () => {
	const [, setHeaderText] = useAtom(headerText);
	const [, setaddImage] = useAtom(addImage);
	const [, setaddPath] = useAtom(addPath);
	const [, setAddValue] = useAtom(addValue);
	const [, setFormValue] = useAtom(formValue);
	const [, setmenu] = useAtom(displayMenu);
	const [, setDashboardValue] = useAtom(dashboardValue);
	const [searchQuery, setSearchQuery] = useState("");
	useMountEffect(() => {
		setHeaderText("Add User");
		setaddImage(UserIcon);
		setaddPath("/user/add");
		setAddValue(true);
		setFormValue(false);
		setDashboardValue(false);
		setmenu(true);
	});

	const [deletedata, setDeletedata] = useState("");

	const { isOpen: isDeleteModalOpen, onOpen, onClose } = useDisclosure();
	const {
		isOpen: isModalOpen,
		onOpen: openModalBase,
		onClose: closeModal,
	} = useDisclosure();

	const [activeId, setActiveId] = useState<string>("");

	const openModal = (id: string) => {
		setActiveId(id);
		openModalBase();
	};

	const deleteUser = deleteUserFn();
	const userData = useLiveQuery(() => db.user.toArray());
	const filteredData = (userData || []).filter((item) =>
		item.phone?.number.toString().includes(searchQuery),
	);

	const handleDelete = async (id: string) => {
		try {
			await db.user.where({ id }).delete();

			await deleteUser.mutateAsync({
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
						width={"100%"}
						zIndex={10}
						bgColor={"#F7F7F9"}
						top={0}
						left={0}
						right={0}
					>
						<Center>
							<Box width={{ md: "80%", base: "90%", lg: "70%" }}>
								<Heading
									color="#095FBA"
									pt="1"
									fontSize="20px"
									mt="16px"
									mb="10px"
								>
									User List
								</Heading>
								<SearchBar
									value={searchQuery}
									onchange={(e) => setSearchQuery(e.target.value)}
									placeholder={"Search by Mobile Number"}
								/>
							</Box>
						</Center>
					</Box>
					<Box zIndex={15} mt={5} bgColor={"#F7F7F9"}>
						{sortedData?.map((items) => (
							<div key={items.created_at}>
								<Center>
									<AnimatePresence>
										<MotionBox
											key={items.created_at}
											initial={{ opacity: 0, y: -50 }}
											animate={{ opacity: 1, y: 0 }}
											exit={{ opacity: 0, y: -50 }}
											transition={{ duration: 0.65 }}
											width={{ md: "80%", base: "90%", lg: "70%" }}
										>
											<DashboardCardWithImage
												profile_img={items.user.photo_url}
												name={items.user.first_name}
												result={items.phone.number}
												heading={"Mobile No:"}
												open_model={() => openModal(items.id)}
												editpath={`/user/edit/${items.id}`}
												handleDelete={() => {
													setDeletedata(items.id);
													onOpen();
												}}
												gender={items.user.gender}
											/>
										</MotionBox>
									</AnimatePresence>
								</Center>
								<UserPopUp
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

export default UserList;
