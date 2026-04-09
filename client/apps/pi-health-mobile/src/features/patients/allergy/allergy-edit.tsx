import { Box, Center } from "@chakra-ui/react";
import {
	GetAllergyNameDataFn,
	UpdateAllergyFn,
} from "../../../query-mutation-services/allergy-data-fn";
import { HeadingTag } from "../../../components/heading-tag";
import { useParams } from "@tanstack/react-router";
import { useAtom } from "jotai";
import {
	headerText,
	addValue,
	formValue,
	dashboardValue,
} from "../../../atoms/header";
import { AlergyForm } from "@repo/ui/forms";
import type { AllergyAdd, AllergyUpdate } from "@repo/types/dto";
import { db } from "../../../db/db";
import { useLiveQuery } from "dexie-react-hooks";
import { useMountEffect } from "@react-hookz/web";
import { motion, AnimatePresence } from "framer-motion";

const MotionBox = motion(Box);

const AlergyEdit = () => {
	const [, setHeaderText] = useAtom(headerText);
	const [, setAddValue] = useAtom(addValue);
	const [, setFormValue] = useAtom(formValue);
	const [, setDashboardValue] = useAtom(dashboardValue);
	const patientEditId = useParams({
		from: "/allergy/edit/$patientId/$allergyId",
		select: (params) => params.patientId,
	});

	const allergyId = useParams({
		from: "/allergy/edit/$patientId/$allergyId",
		select: (params) => params.allergyId,
	});
	const updateMutation = UpdateAllergyFn();

	const allergy = useLiveQuery(() => db.allergy.toArray());
	const allergyName = GetAllergyNameDataFn();
	const filteredData = allergy?.find((item) => item.id === allergyId);

	const headerData = useLiveQuery(() => db.patients.toArray());
	const headerFilterData = headerData?.find(
		(item) => item.id === patientEditId,
	);

	useMountEffect(() => {
		setHeaderText("Edit Allergy");
		setAddValue(true);
		setFormValue(true);
		setDashboardValue(false);
	});
	return (
		<div>
			<AnimatePresence>
				<Box bgColor={"#F7F7F9"}>
					<Center>
						<MotionBox
							initial={{ opacity: 0, x: 50 }}
							animate={{ opacity: 1, x: 0 }}
							exit={{ opacity: 0, x: 50 }}
							transition={{ duration: 0.65 }}
							width={{ md: "80%", base: "90%", lg: "70%" }}
						>
							<HeadingTag
								label_text={
									headerFilterData?.user.first_name || "Default Value"
								}
							/>
							<AlergyForm
								onSubmit={(p) => {
									const editVal: AllergyUpdate = {
										id: allergyId,
										input: { ...(p as AllergyAdd) },
									};
									updateMutation.mutateAsync(editVal);
								}}
								patientId={patientEditId}
								allergyId={allergyId}
								allergen={allergyName.data?.result}
								initialValues={filteredData}
								lastUpdatedInput={db.getLastUpdated}
								edit={true}
							/>
						</MotionBox>
					</Center>
				</Box>
			</AnimatePresence>
		</div>
	);
};

export default AlergyEdit;
