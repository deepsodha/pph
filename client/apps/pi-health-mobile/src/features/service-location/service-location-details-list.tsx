import { Box, Center, Fade, Heading, useDisclosure } from "@chakra-ui/react";
import Icon from "../../assets/Add Service Location Icon.svg";

import SearchBar from "../../components/search-bar";
import DashboardCard from "../../components/dashboard-card";
import ServiceLocationDetailsPopUp from "./service-location-details-popup";
import { useAtom } from "jotai";
import { useState } from "react";
import {
	headerText,
	addImage,
	addPath,
	addValue,
	dashboardValue,
	formValue,
	displayMenu,
} from "../../atoms/header";
import { DeleteServiceLocationDataFn } from "../../query-mutation-services/service-location-detail-data-fn";
import DeleteModal from "../../components/delete-modal";
import { db } from "../../db/db";
import { useLiveQuery } from "dexie-react-hooks";
import { useMountEffect } from "@react-hookz/web";
import { motion, AnimatePresence } from "framer-motion";
const MotionBox = motion(Box);

const ServiceLocationDetailsList = () => {
	const {
		isOpen: isModalOpen,
		onOpen: openModalBase,
		onClose: closeModal,
	} = useDisclosure();
	const { isOpen: isDeleteModalOpen, onOpen, onClose } = useDisclosure();
	const [deletedata, setDeletedata] = useState("");

	const [activeId, setActiveId] = useState<string>("");
	const [, setHeaderText] = useAtom(headerText);
	const [, setaddImage] = useAtom(addImage);
	const [, setaddPath] = useAtom(addPath);
	const [, setAddValue] = useAtom(addValue);
	const [, setFormValue] = useAtom(formValue);
	const [, setDashboardValue] = useAtom(dashboardValue);
	const [, setmenu] = useAtom(displayMenu);

	const openModal = (id: string) => {
		setActiveId(id);
		openModalBase();
	};

	const deleteServiceLocation = DeleteServiceLocationDataFn();

	const handleDelete = async (id: string) => {
		try {
			await db.servicelocation.where({ id }).delete();

			await deleteServiceLocation.mutateAsync({
				id,
				last_updated_input: await db.getLastUpdated(),
			});
		} catch (error) {
			console.error("Error Deleting ServiceLocation", error);
		}
		onClose();
	};

	useMountEffect(() => {
		setHeaderText("Add Service Location");
		setaddImage(Icon);
		setaddPath("/servicelocation/add");
		setAddValue(true);
		setFormValue(false);
		setDashboardValue(false);
		setmenu(true);
	});

	const [searchQuery, setSearchQuery] = useState("");
	const serviceLocationData = useLiveQuery(() => db.servicelocation.toArray());
	const filteredData = (serviceLocationData || []).filter((item) =>
		item?.service_location_name
			.toString()
			.toLowerCase()
			.includes(searchQuery.toLowerCase()),
	);

	const sortedData = filteredData.slice().sort((a, b) => {
		const dateA = new Date(a.created_at);
		const dateB = new Date(b.created_at);
		return dateB.getTime() - dateA.getTime();
	});
	return (
		<>
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
						<Center>
							<Box width={{ md: "80%", base: "90%", lg: "70%" }}>
								<Heading
									color="#095FBA"
									pt="1"
									fontSize="20px"
									mt="16px"
									mb="10px"
								>
									Service Location List
								</Heading>
								<SearchBar
									value={searchQuery}
									onchange={(e) => setSearchQuery(e.target.value)}
									placeholder={"Search  by Name"}
								/>
							</Box>
						</Center>
					</Box>

					<Box zIndex={15} mt={5} bgColor={"#F7F7F9"} pb="10%">
						{sortedData.map((items) => (
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
											<DashboardCard
												heading_1="Name :"
												result_1={items?.service_location_name}
												heading_2="Address :"
												result_2={items?.address.address_line}
												open_model={() => openModal(items.id)}
												handleDelete={() => {
													setDeletedata(items.id);
													onOpen();
												}}
												editpath={`/serviceLocation/edit/${items.id}`}
											/>
										</MotionBox>
									</AnimatePresence>
								</Center>
								<ServiceLocationDetailsPopUp
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
		</>
	);
};

export default ServiceLocationDetailsList;
