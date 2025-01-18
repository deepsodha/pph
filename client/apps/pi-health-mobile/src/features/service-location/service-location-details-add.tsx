import { Box, useUpdateEffect } from "@chakra-ui/react";

import { useAtom } from "jotai";
import {
	addValue,
	dashboardValue,
	displayMenu,
	formValue,
	headerText,
} from "../../atoms/header";
import { ServiceLocationForm } from "@repo/ui/forms";
import { addServiceLocationDetailsDataFn } from "../../query-mutation-services/service-location-detail-data-fn";
import type { ServiceLocationAdd } from "@repo/types/dto";
import { db } from "../../db/db";
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

const ServiceLocationDetailsAddForm = () => {
	const [, setHeaderText] = useAtom(headerText);
	const [, setAddValue] = useAtom(addValue);
	const [, setFormValue] = useAtom(formValue);
	const [, setDashboardValue] = useAtom(dashboardValue);
	const [, setmenu] = useAtom(displayMenu);
	const { country, state } = CountryData();
	const organizationCountry = GetOrganizationCountryDataFn();
	const organizationState = GetOrganizationStateDataFn(country);
	const organizationCity = GetOrganizationCityDataFn(state, country);
	const addMutation = addServiceLocationDetailsDataFn();

	useMountEffect(() => {
		setHeaderText("Add Service Location");
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
						initial={{ opacity: 0, x: -50 }}
						animate={{ opacity: 1, x: 0 }}
						exit={{ opacity: 0, x: -50 }}
						transition={{ duration: 0.65 }}
					>
						<ServiceLocationForm
							onSubmit={(p) => {
								addMutation.mutateAsync(p as ServiceLocationAdd);
							}}
							lastUpdatedInput={db.getLastUpdated}
							edit={false}
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

export default ServiceLocationDetailsAddForm;
