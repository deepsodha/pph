import { Box, Center, Fade } from "@chakra-ui/react";
import { useParams, useRouterState } from "@tanstack/react-router";
import { useAtom } from "jotai";
import { useMountEffect } from "@react-hookz/web";
import {
	headerText,
	addValue,
	formValue,
	dashboardValue,
} from "../../../../atoms/header";
import { OrderForm } from "@repo/ui/forms";
import type { OrderAdd, OrderUpdate } from "@repo/types/dto";
import { db } from "../../../../db/db";
import {
	AddOrderDataFn,
	UpdateOrderDataFn,
} from "../../../../query-mutation-services/order-data-fn";
import { useLiveQuery } from "dexie-react-hooks";
import { VaccineData } from "../../../../atoms/vaccine-data";

const Order = () => {
	const [, setHeaderText] = useAtom(headerText);
	const [, setAddValue] = useAtom(addValue);
	const [, setFormValue] = useAtom(formValue);
	const [, setDashboardValue] = useAtom(dashboardValue);
	const router = useRouterState();
	const role = router.location.pathname.split("/")[2];
	const { orderId } = useParams({ from: "orderId" });
	const { patientId } = useParams({ from: "patientId" });
	const orderData = useLiveQuery(() => db.orders.toArray());
	const doctorData = useLiveQuery(() => db.doctors.toArray());
	const firstNamesList = doctorData?.map(({ user }) => user.user.first_name);
	const filteredData = orderData?.find((item) => item.id === orderId);
	const { vaccine } = VaccineData();
	const addMutation = AddOrderDataFn();
	const updateMutation = UpdateOrderDataFn();
	useMountEffect(() => {
		setHeaderText(role === "edit" ? "Edit Order" : "Add New Order");
		setAddValue(true);
		setFormValue(true);
		setDashboardValue(false);
	});

	return (
		<div>
			<Fade in={true}>
				<Box minHeight={"100vh"} bgColor={"#F7F7F9"} overflowY="auto">
					<Center>
						<Box width={{ md: "80%", base: "90%", lg: "70%" }}>
							<OrderForm
								onSubmit={(p) => {
									if (role === "edit") {
										const editVal: OrderUpdate = {
											id: orderId,
											input: { ...(p as OrderAdd) },
										};
										updateMutation.mutateAsync(editVal);
									} else {
										addMutation.mutateAsync(p as OrderAdd);
									}
								}}
								lastUpdatedInput={db.getLastUpdated}
								initialValues={filteredData}
								edit={role === "edit"}
								patientId={patientId}
								vaccine={vaccine}
								doctorList={firstNamesList}
							/>
						</Box>
					</Center>
				</Box>
			</Fade>
		</div>
	);
};

export default Order;
