import { Box, Fade } from "@chakra-ui/react";
import {
	AddOrganizationDataFn,
	GetOrganizationCityDataFn,
	GetOrganizationCountryDataFn,
	GetOrganizationDataFn,
	GetOrganizationStateDataFn,
	UpdateOrganizationDataFn,
} from "../../query-mutation-services/organization-data-fn";
import { useParams, useRouterState } from "@tanstack/react-router";
import { useAtom } from "jotai";
import {
	headerText,
	addValue,
	formValue,
	dashboardValue,
} from "../../atoms/header";
import { OrganizationForm } from "@repo/ui/forms";
import type { OrganizationAdd, OrganizationUpdate } from "@repo/types/dto";
import { CountryData } from "../../atoms/city-country-state";
import { useMountEffect } from "@react-hookz/web";

const Organization = () => {
	const [, setHeaderText] = useAtom(headerText);
	const [, setAddValue] = useAtom(addValue);
	const [, setFormValue] = useAtom(formValue);
	const [, setDashboardValue] = useAtom(dashboardValue);
	const { country, state } = CountryData();

	const router = useRouterState();
	const role = router.location.pathname.split("/")[2];
	const { organizationId } = useParams({ from: "organizationId" });
	const addMutation = AddOrganizationDataFn();
	const organization = GetOrganizationDataFn();
	const updateMutation = UpdateOrganizationDataFn();
	const organizationCountry = GetOrganizationCountryDataFn();
	const organizationState = GetOrganizationStateDataFn(country);
	const organizationCity = GetOrganizationCityDataFn(state, country);
	const filteredData = (organization.data?.result || []).filter(
		(item) => organizationId && item.id.includes(organizationId),
	)[0];

	useMountEffect(() => {
		setHeaderText(
			role === "edit" ? "Edit Organization" : "Add New Organization",
		);
		setAddValue(true);
		setFormValue(true);
		setDashboardValue(false);
	});

	return (
		<div>
			<Fade in={true}>
				<Box minHeight={"100vh"} bgColor={"#F7F7F9"} overflowY="auto">
					<Box>
						<OrganizationForm
							onSubmit={(p) => {
								if (role === "edit") {
									const editVal = { ...p, id: organizationId };
									updateMutation.mutateAsync(editVal as OrganizationUpdate);
								} else {
									addMutation.mutateAsync(p as OrganizationAdd);
								}
							}}
							initialValues={filteredData}
							country={organizationCountry.data?.result}
							state={organizationState.data?.result}
							city={organizationCity.data?.result}
						/>
					</Box>
				</Box>
			</Fade>
		</div>
	);
};

export default Organization;
