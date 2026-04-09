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
import { DeleteDrugDataFn, GetDrugDataFn } from "../query-mutation-fn/drug";
import { useState } from "react";

const DrugList = () => {
	const [, setHeaderText] = useAtom(headerText);
	const [, setaddPath] = useAtom(addPath);
	const [, setAddValue] = useAtom(addValue);
	const [, setFormValue] = useAtom(formValue);
	const [, setDashboardValue] = useAtom(dashboardValue);
	const [deletedata, setDeletedata] = useState("");

	useMountEffect(() => {
		setHeaderText("Add New Drug");
		setaddPath("/drug/add");
		setAddValue(false);
		setFormValue(false);
		setDashboardValue(false);
	});
	const [searchQuery, setSearchQuery] = useState("");
	const { isOpen: isDeleteModalOpen, onOpen, onClose } = useDisclosure();
	const DrugData = GetDrugDataFn();
	console.log(DrugData.data, "drug data....");
	const deleteDrug = DeleteDrugDataFn();

	const handleDelete = async (id: string) => {
		try {
			await deleteDrug.mutateAsync(id);
			DrugData.refetch();
		} catch (error) {
			console.error("Error deleting organization:", error);
		}
		onClose();
	};

	const filteredData = (DrugData.data || []).filter((item) =>
		item.brand?.toString().toLowerCase().includes(searchQuery.toLowerCase()),
	);
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
									Drug List
								</Heading>
								<SearchBar
									value={searchQuery}
									onchange={(e) => setSearchQuery(e.target.value)}
									placeholder={"Search by brand name"}
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
										heading_1={"Brand Name"}
										result_1={items.brand}
										heading_2={"Manufacturer Name:"}
										result_2={items.manufacturer_name}
										editpath={`/drug/edit/${items.id}`}
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
export default DrugList;
