import { Fade, Center, Heading, Box, useDisclosure } from "@chakra-ui/react";
import SearchBar from "../components/search-bar";
import { useAtom } from "jotai";
import {
	addPath,
	addValue,
	dashboardValue,
	formValue,
	headerText,
} from "../atoms/header";
import { useMountEffect } from "@react-hookz/web";
import DeleteModal from "../components/delete-modal";
import DashboardListCard from "../components/dashboard-list-card";
import {
	DeleteClientDataFn,
	GetClientDataFn,
} from "../query-mutation-fn/client";
import { useState } from "react";

const ClientList = () => {
	const [, setHeaderText] = useAtom(headerText);
	const [, setaddPath] = useAtom(addPath);
	const [, setAddValue] = useAtom(addValue);
	const [, setFormValue] = useAtom(formValue);
	const [, setDashboardValue] = useAtom(dashboardValue);

	useMountEffect(() => {
		setHeaderText("Add New Client");
		setaddPath("/client/add");
		setAddValue(false);
		setFormValue(false);
		setDashboardValue(false);
	});
	const { isOpen: isDeleteModalOpen, onClose, onOpen } = useDisclosure();
	const ClientData = GetClientDataFn();
	console.log(ClientData.data, "client data....");

	const [searchQuery, setSearchQuery] = useState("");
	const filteredData = (ClientData.data || []).filter((item) =>
		item.name?.toString().toLowerCase().includes(searchQuery.toLowerCase()),
	);
	const deleteClient = DeleteClientDataFn();

	const handleDelete = async (id: string) => {
		try {
			await deleteClient.mutateAsync(id);
			ClientData.refetch();
		} catch (error) {
			console.error("Error deleting organization:", error);
		}
		onClose();
	};

	const [deletedata, setDeletedata] = useState("");
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
									Client List
								</Heading>
								<SearchBar
									value={searchQuery}
									onchange={(e) => setSearchQuery(e.target.value)}
									placeholder={"Search by Client name"}
								/>
							</Box>
						</Center>
					</Box>
					<Center>
						<Box
							mt="4%"
							cursor={"pointer"}
							width={{ md: "80%", base: "90%", lg: "70%" }}
						>
							{filteredData?.map((items) => (
								<div key={items.id}>
									<DashboardListCard
										heading_1={"Client Name"}
										result_1={items.name}
										heading_2={"Mobile Number:"}
										result_2={items.mobile_number}
										editpath={`/client/edit/${items.id}`}
										handleDelete={() => {
											setDeletedata(items.id);
											onOpen();
										}}
									/>
								</div>
							))}
						</Box>
					</Center>
					<DeleteModal
						open={isDeleteModalOpen}
						close={onClose}
						handle_delete={() => handleDelete(deletedata)}
					/>
				</Box>
			</Fade>
		</div>
	);
};
export default ClientList;
