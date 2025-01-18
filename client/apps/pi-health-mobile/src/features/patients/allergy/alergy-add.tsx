import { Box, Center } from "@chakra-ui/react";
import {
	AddAllergyDataFn,
	GetAllergyNameDataFn,
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
import type { AllergyAdd } from "@repo/types/dto";
import { db } from "../../../db/db";
import { useLiveQuery } from "dexie-react-hooks";
import { useMountEffect } from "@react-hookz/web";
import { motion, AnimatePresence } from "framer-motion";

const MotionBox = motion(Box);
const AlergyAdd = () => {
	const [, setHeaderText] = useAtom(headerText);
	const [, setAddValue] = useAtom(addValue);
	const [, setFormValue] = useAtom(formValue);
	const [, setDashboardValue] = useAtom(dashboardValue);

	const patientAddId = useParams({
		from: "/allergy/add/$patientId",
		select: (params) => params.patientId,
	});

	const addMutation = AddAllergyDataFn();

	const allergyName = GetAllergyNameDataFn();

	const headerData = useLiveQuery(() => db.patients.toArray());
	const headerFilterData = headerData?.find((item) => item.id === patientAddId);

	useMountEffect(() => {
		setHeaderText("Add New Allergy");
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
							initial={{ opacity: 0, x: -50 }}
							animate={{ opacity: 1, x: 0 }}
							exit={{ opacity: 0, x: -50 }}
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
									addMutation.mutateAsync(p as AllergyAdd);
								}}
								patientId={patientAddId}
								allergen={allergyName.data?.result}
								lastUpdatedInput={db.getLastUpdated}
								edit={false}
							/>
						</MotionBox>
					</Center>
				</Box>
			</AnimatePresence>
		</div>
	);
};

export default AlergyAdd;
