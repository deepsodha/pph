import { Fade, Box, Center } from "@chakra-ui/react";
import { useMountEffect } from "@react-hookz/web";
import type {
	SocialHistoryAdd,
	SocialHistoryState,
	SocialHistoryUpdate,
} from "@repo/types/dto";
import { useRouterState, useParams } from "@tanstack/react-router";
import { useLiveQuery } from "dexie-react-hooks";
import { useAtom } from "jotai";
import {
	headerText,
	addValue,
	formValue,
	dashboardValue,
} from "../../../../atoms/header";
import { HeadingTag } from "../../../../components/heading-tag";
import { db } from "../../../../db/db";
import { SocialHistoryForm } from "@repo/ui/forms";
import {
	AddSocialHistoryFn,
	UpdateSocialHistoryFn,
} from "../../../../query-mutation-services/social-history";

const socialhistory = () => {
	const [, setHeaderText] = useAtom(headerText);
	const { patientId } = useParams({ from: "patientId" });
	const [, setAddValue] = useAtom(addValue);
	const [, setFormValue] = useAtom(formValue);
	const [, setDashboardValue] = useAtom(dashboardValue);
	const addMutation = AddSocialHistoryFn();
	const updateMutation = UpdateSocialHistoryFn();
	const router = useRouterState();
	const role = router.location.pathname.split("/")[2];
	const { socialHistoryId } = useParams({ from: "socialHistoryId" });
	const socialHistoryData = useLiveQuery(() => db.socialhistory.toArray());
	const filteredData = socialHistoryData?.find(
		(item) => item.id === socialHistoryId,
	) as SocialHistoryState | undefined;

	const headerData = useLiveQuery(() => db.patients.toArray());
	const headerFilterData = headerData?.find((item) => item.id === patientId);
	useMountEffect(() => {
		setHeaderText(
			role === "edit" ? "Edit social History" : "Add New social History",
		);
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
							<HeadingTag
								label_text={
									headerFilterData?.user.first_name || "Default Value"
								}
							/>
							<SocialHistoryForm
								onSubmit={(p) => {
									if (role === "edit") {
										const editVal: SocialHistoryUpdate = {
											input: { ...(p as SocialHistoryAdd) },
											id: socialHistoryId,
										};
										updateMutation.mutateAsync(editVal);
									} else {
										addMutation.mutateAsync(p as SocialHistoryAdd);
									}
								}}
								patientId={patientId}
								socialHistoryId={socialHistoryId}
								initialValues={filteredData}
								edit={role === "edit"}
								lastUpdatedInput={db.getLastUpdated}
							/>
						</Box>
					</Center>
				</Box>
			</Fade>
		</div>
	);
};

export default socialhistory;
