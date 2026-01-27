import { Box, useUpdateEffect } from "@chakra-ui/react";
import { useParams } from "@tanstack/react-router";

import { useAtom } from "jotai";
import {
	addValue,
	dashboardValue,
	displayMenu,
	formValue,
	headerText,
} from "../../atoms/header";
import { ServiceLocationForm } from "@repo/ui/forms";
import { UpdateServiceLocationDetailsDataFn } from "../../query-mutation-services/service-location-detail-data-fn";
import type {
	ServiceLocationAdd,
	ServiceLocationUpdate,
} from "@repo/types/dto";
import { db } from "../../db/db";
import { useLiveQuery } from "dexie-react-hooks";
import {
	GetOrganizationCityDataFn,
	GetOrganizationCountryDataFn,
	GetOrganizationLocationPinCodeDataFn,
	GetOrganizationStateDataFn,
} from "../../query-mutation-services/organization-data-fn";
import { CountryData } from "../../atoms/city-country-state";
import { useDebouncedState, useMountEffect } from "@react-hookz/web";
import { motion, AnimatePresence } from "framer-motion";

const MotionBox = motion(Box);

const ServiceLocationDetailsEditForm = () => {
	const [, setHeaderText] = useAtom(headerText);
	const [, setAddValue] = useAtom(addValue);
	const [, setFormValue] = useAtom(formValue);
	const [, setDashboardValue] = useAtom(dashboardValue);
	const [, setmenu] = useAtom(displayMenu);
	const { country, state } = CountryData();
	const updateMutation = UpdateServiceLocationDetailsDataFn();
	const organizationCountry = GetOrganizationCountryDataFn();
	const organizationState = GetOrganizationStateDataFn(country);
	const organizationCity = GetOrganizationCityDataFn(state, country);
	const servicelocation = useLiveQuery(() => db.servicelocation.toArray());
	const servicelocationId = useParams({
		from: "/servicelocation/edit/$servicelocationId",
		select: (params) => params.servicelocationId,
	});
	const filteredData = servicelocation?.find(
		(item) => item.id === servicelocationId,
	);
	useMountEffect(() => {
		setHeaderText("Edit Service Location");
		setAddValue(true);
		setFormValue(true);
		setDashboardValue(false);
		setmenu(false);
	});

	const [pincodedata, setPincodeData] = useDebouncedState("", 500);
	console.log(pincodedata, "i changed pincode data from form ");

	const locationData = GetOrganizationLocationPinCodeDataFn(pincodedata);
	console.log(locationData.data?.result);
	useUpdateEffect(() => {
		locationData.refetch();
	}, [pincodedata]);

	return (
		<Box>
			<AnimatePresence>
				<Box bgColor={"#F7F7F9"}>
					<MotionBox
						initial={{ opacity: 0, x: 50 }}
						animate={{ opacity: 1, x: 0 }}
						exit={{ opacity: 0, x: 50 }}
						transition={{ duration: 0.65 }}
					>
						<ServiceLocationForm
							onSubmit={(p) => {
								const editVal: ServiceLocationUpdate = {
									id: servicelocationId,
									input: { ...(p as ServiceLocationAdd) },
								};
								updateMutation.mutateAsync(editVal);
							}}
							servicelocationId={servicelocationId}
							initialValues={filteredData}
							lastUpdatedInput={db.getLastUpdated}
							edit={true}
							country={organizationCountry.data?.result}
							state={organizationState.data?.result}
							city={organizationCity.data?.result}
							locationData={locationData.data?.result}
							pincodeOnChange={(pincode) => setPincodeData(pincode)}
						/>
					</MotionBox>
				</Box>
			</AnimatePresence>
		</Box>
	);
};

export default ServiceLocationDetailsEditForm;
