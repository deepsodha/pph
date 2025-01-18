import { Fade, Center, Box, useDisclosure } from "@chakra-ui/react";
import { useMountEffect } from "@react-hookz/web";
import { useParams } from "@tanstack/react-router";
import { useAtom } from "jotai";
import {
	headerText,
	addImage,
	addPath,
	addValue,
	formValue,
	dashboardValue,
} from "../../../../atoms/header";
import Icon from "../../../../assets/Add Service Location Icon.svg";
import SearchBar from "../../../../components/search-bar";
import { useState } from "react";
import { useLiveQuery } from "dexie-react-hooks";
import { db } from "../../../../db/db";
import DeleteModal from "../../../../components/delete-modal";
import { DeleteImplantableDevicesDataFn } from "../../../../query-mutation-services/implantable-devices";
import ImplantableDeviceCard from "./implantable-devices-card";
const ImplantableDevicesList = () => {
	const [, setHeaderText] = useAtom(headerText);
	const [, setaddImage] = useAtom(addImage);
	const [, setaddPath] = useAtom(addPath);
	const [, setAddValue] = useAtom(addValue);
	const [, setFormValue] = useAtom(formValue);
	const [, setDashboardValue] = useAtom(dashboardValue);

	useMountEffect(() => {
		setHeaderText("Implantable Device");
		setaddImage(Icon);
		setaddPath(`/implantabledevice/add/${patientId}`);
		setAddValue(false);
		setFormValue(false);
		setDashboardValue(false);
	});
	const [searchQuery, setSearchQuery] = useState("");

	const { patientId } = useParams({ from: "patientId" });
	const { isOpen: isDeleteModalOpen, onClose, onOpen } = useDisclosure();
	const [deletedata, setDeletedata] = useState("");
	const deleteImplantableDevices = DeleteImplantableDevicesDataFn();
	const handleDelete = async (id: string) => {
		try {
			await db.implantabledevices.where({ id }).delete();

			await deleteImplantableDevices.mutateAsync({
				id,
				patient_id: patientId,
				last_updated_input: await db.getLastUpdated(),
			});
		} catch (error) {
			console.error("Error deleting ImplantableDevices:", error);
		}
		onClose();
	};

	const implantabledevicesData = useLiveQuery(() =>
		db.implantabledevices.toArray(),
	);
	const implantabledevicesDataById = implantabledevicesData?.filter(
		(items) => items.patient_id === patientId,
	);

	const filteredData = (implantabledevicesDataById || []).filter((items) =>
		items.status?.toLowerCase().toString().includes(searchQuery.toLowerCase()),
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
						top={0}
						zIndex={10}
						bgColor={"#F7F7F9"}
					>
						<Center>
							<Box width={{ md: "80%", base: "90%", lg: "70%" }}>
								<SearchBar
									value={searchQuery}
									onchange={(e) => setSearchQuery(e.target.value)}
									placeholder={"Implantable Device Details"}
								/>
							</Box>
						</Center>
					</Box>
					<Center>
						<Box
							zIndex={15}
							mt={5}
							bgColor={"#F7F7F9"}
							width={{ md: "80%", base: "90%", lg: "70%" }}
						>
							{sortedData?.map((items) => (
								<div key={items.created_at}>
									<ImplantableDeviceCard
										result_1={items.status}
										result_2={items.udi}
										result_3={items.comments}
										editpath={`/implantabledevice/edit/${patientId}/${items.id}`}
										handleDelete={() => {
											setDeletedata(items.id);
											onOpen();
										}}
									/>
									<DeleteModal
										open={isDeleteModalOpen}
										close={onClose}
										handle_delete={() => handleDelete(deletedata)}
									/>
								</div>
							))}
						</Box>
					</Center>
				</Box>
			</Fade>
		</div>
	);
};

export default ImplantableDevicesList;
