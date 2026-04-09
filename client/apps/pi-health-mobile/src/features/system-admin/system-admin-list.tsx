import { Box, Center, Fade, Heading, useDisclosure } from "@chakra-ui/react";
import Admin from "../../assets/System Admin Icon.svg";

import DashboardCardWithImage from "../../components/dashboard-card-with-image";
import SearchBar from "../../components/search-bar";
import SystenAdminPopup from "./system-admin-popup";
import { useState } from "react";
import { useAtom } from "jotai";
import {
	DeleteSystemAdminDataFn,
	GetSystemAdminDataFn,
} from "../../query-mutation-services/system-admin-data-fn";
import {
	headerText,
	addImage,
	addPath,
	addValue,
	dashboardValue,
	formValue,
} from "../../atoms/header";
import DeleteModal from "../../components/delete-modal";
import { db } from "../../db/db";
import { useMountEffect } from "@react-hookz/web";
import { motion, AnimatePresence } from "framer-motion";
import { GetOrganizationLocationPinCodeDataFn } from "../../query-mutation-services/organization-data-fn";

const MotionBox = motion(Box);

const SystemAdminList = () => {
	const {
		isOpen: isModalOpen,
		onOpen: openModalBase,
		onClose: closeModal,
	} = useDisclosure();
	const { isOpen: isDeleteModalOpen, onOpen, onClose } = useDisclosure();
	const [deletedata, setDeletedata] = useState("");

	const [activeId, setActiveId] = useState<string>("");
	const [orgId, setorgId] = useState<string>("");

	const openModal = (id: string) => {
		setActiveId(id);
		openModalBase();
	};

	const deleteSystemAdmin = DeleteSystemAdminDataFn();
	const handleDelete = async (id: string) => {
		try {
			await db.systemadmin.where({ id }).delete();

			await deleteSystemAdmin.mutateAsync({
				id: id,
				org_id: orgId,
				last_updated_input: await db.getLastUpdated(),
			});
			systemAdminData.refetch();
		} catch (error) {
			console.error("Error deleting allergy:", error);
		}
		onClose();
	};
	const [, setHeaderText] = useAtom(headerText);
	const [, setaddImage] = useAtom(addImage);
	const [, setaddPath] = useAtom(addPath);
	const [, setAddValue] = useAtom(addValue);
	const [, setFormValue] = useAtom(formValue);
	const [, setDashboardValue] = useAtom(dashboardValue);

	useMountEffect(() => {
		setHeaderText("Add System Admin");
		setaddImage(Admin);
		setaddPath("/systemadmin/add");
		setAddValue(false);
		setFormValue(false);
		setDashboardValue(false);
	});

	const [searchQuery, setSearchQuery] = useState("");
	// const systemAdminData = useLiveQuery(() => db.systemadmin.toArray());
	const systemAdminData = GetSystemAdminDataFn();
	const filteredData = (systemAdminData.data?.result || []).filter((item) =>
		item?.number.toString().includes(searchQuery),
	);
	const getOrgLocData = GetOrganizationLocationPinCodeDataFn("364001");
	console.log(getOrgLocData.data?.result);

	// const sortedData = filteredData.slice().sort((a, b) => {
	// 	const dateA = new Date(a.created_at);
	// 	const dateB = new Date(b.created_at);
	// 	return dateB.getTime() - dateA.getTime();
	// });
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
									System Admin List
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
						{filteredData?.map((item) => (
							<div key={item.id}>
								<Center>
									<AnimatePresence>
										<MotionBox
											key={item.id}
											initial={{ opacity: 0, y: -50 }}
											animate={{ opacity: 1, y: 0 }}
											exit={{ opacity: 0, y: -50 }}
											transition={{ duration: 0.65 }}
											width={{ md: "80%", base: "90%", lg: "70%" }}
										>
											<DashboardCardWithImage
												profile_img={item.photo_url}
												name={item.first_name}
												result={item.number}
												heading={"Mobile No:"}
												open_model={() => openModal(item.id)}
												handleDelete={() => {
													setDeletedata(item.id);
													setorgId(item.org_id);
													onOpen();
												}}
												editpath={`/systemadmin/edit/${item.id}`}
												gender={item.gender}
											/>
										</MotionBox>
									</AnimatePresence>
								</Center>
								<SystenAdminPopup
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

export default SystemAdminList;
