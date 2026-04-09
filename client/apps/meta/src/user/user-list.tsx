import { Fade, Center, Heading, Box, useDisclosure } from "@chakra-ui/react";
// import DeleteModal from "../components/delete-modal";
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
import { DeleteUserDataFn, GetUserDataFn } from "../query-mutation-fn/user";
import { useState } from "react";

const UserList = () => {
	const [, setHeaderText] = useAtom(headerText);
	const [, setaddPath] = useAtom(addPath);
	const [, setAddValue] = useAtom(addValue);
	const [, setFormValue] = useAtom(formValue);
	const [, setDashboardValue] = useAtom(dashboardValue);
	const [deletedata, setDeletedata] = useState("");

	useMountEffect(() => {
		setHeaderText("Add New User");
		setaddPath("/user/add");
		setAddValue(false);
		setFormValue(false);
		setDashboardValue(false);
	});
	const { isOpen: isDeleteModalOpen, onOpen, onClose } = useDisclosure();

	const UserData = GetUserDataFn();
	console.log(UserData.data, "user data....");
	const deleteUser = DeleteUserDataFn();

	const handleDelete = async (id: string) => {
		try {
			await deleteUser.mutateAsync(id);
			UserData.refetch();
		} catch (error) {
			console.error("Error deleting organization:", error);
		}
		onClose();
	};

	const [searchQuery, setSearchQuery] = useState("");

	const filteredData = (UserData.data || []).filter((item) =>
		item.mobile_number?.toString().includes(searchQuery),
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
					<Center>
						<Box
							mt="4%"
							cursor={"pointer"}
							width={{ md: "80%", base: "90%", lg: "70%" }}
						>
							{filteredData?.map((items) => (
								<div key={items.id}>
									<DashboardListCard
										heading_1={"Mobile No"}
										result_1={items.mobile_number}
										heading_2={"Role:"}
										result_2={items.role}
										editpath={`/user/edit/${items.id}`}
										handleDelete={() => {
											setDeletedata(items.id);
											items.id;
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
export default UserList;
