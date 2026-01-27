import { Box, Center, Fade, useDisclosure } from "@chakra-ui/react";
import { Heading } from "@chakra-ui/react";
import Order from "../../../../assets/New order icon 36 x 36.svg";
import DashboardCard from "../../../../components/dashboard-card";
import { useAtom } from "jotai";
import SearchBar from "../../../../components/search-bar";
import OrderPopup from "./order-popup";
import { useState } from "react";
import { useLiveQuery } from "dexie-react-hooks";
import { DeleteOrderDataFn } from "../../../../query-mutation-services/order-data-fn";
import { useMountEffect } from "@react-hookz/web";
import {
	headerText,
	addImage,
	addPath,
	addValue,
	dashboardValue,
	formValue,
} from "../../../../atoms/header";
import { useParams } from "@tanstack/react-router";
import DeleteModal from "../../../../components/delete-modal";
import { db } from "../../../../db/db";

const OrderList = () => {
	const [, setHeaderText] = useAtom(headerText);
	const [, setaddImage] = useAtom(addImage);
	const [, setaddPath] = useAtom(addPath);
	const [, setAddValue] = useAtom(addValue);
	const [, setFormValue] = useAtom(formValue);
	const [, setDashboardValue] = useAtom(dashboardValue);

	const { patientId } = useParams({ from: "patientId" });
	const { isOpen: isDeleteModalOpen, onOpen, onClose } = useDisclosure();

	const [deletedata, setDeletedata] = useState("");
	const [searchQuery, setSearchQuery] = useState("");
	const orderData = useLiveQuery(() => db.orders.toArray());
	const deleteOrder = DeleteOrderDataFn();

	const handleDelete = async (id: string) => {
		try {
			await db.orders.where({ id }).delete();

			await deleteOrder.mutateAsync({
				id,
				patient_id: patientId,
				last_updated_input: await db.getLastUpdated(),
			});
		} catch (error) {
			console.error("Error deleting order:", error);
		}
		onClose();
	};

	useMountEffect(() => {
		setHeaderText("Add New Order");
		setaddImage(Order);
		setaddPath(`/order/add/${patientId}`);
		setAddValue(false);
		setFormValue(false);
		setDashboardValue(false);
	});
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
	const orderDataById = orderData?.filter(
		(item) => item.patient_id === patientId,
	);
	const filteredData = (orderDataById || []).filter((item) =>
		item.vaccine.toString().toLowerCase().includes(searchQuery.toLowerCase()),
	);

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
									Order List
								</Heading>
								<SearchBar
									value={searchQuery}
									onchange={(e) => setSearchQuery(e.target.value)}
									placeholder={"Search by Vaccine"}
								/>
							</Box>
						</Center>
					</Box>
					<Box zIndex={15} mt={5} bgColor={"#F7F7F9"}>
						{sortedData?.map((item) => (
							<Center key={item.id}>
								<Box
									key={item.created_at}
									width={{ md: "80%", base: "90%", lg: "70%" }}
								>
									<Box cursor={"pointer"}>
										<DashboardCard
											heading_1={"Vaccine:"}
											result_1={item.vaccine}
											heading_2={"Type:"}
											result_2={item.types}
											open_model={() => openModal(item.id)}
											handleDelete={() => {
												setDeletedata(item.id);
												onOpen();
											}}
											editpath={`/order/edit/${patientId}/${item.id}`}
										/>
									</Box>

									<DeleteModal
										open={isDeleteModalOpen}
										close={onClose}
										handle_delete={() => handleDelete(deletedata)}
									/>
									<OrderPopup
										open={isModalOpen}
										close={closeModal}
										id={activeId}
									/>
								</Box>
							</Center>
						))}
					</Box>
				</Box>
			</Fade>
		</div>
	);
};

export default OrderList;
