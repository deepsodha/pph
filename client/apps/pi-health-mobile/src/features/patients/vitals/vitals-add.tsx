import { Box, Center } from "@chakra-ui/react";
import { AddVitalsDataFn } from "../../../query-mutation-services/vital-data-fn";
import { HeadingTag } from "../../../components/heading-tag";
import { useParams } from "@tanstack/react-router";
import { useAtom } from "jotai";
import { useMountEffect } from "@react-hookz/web";
import {
	headerText,
	addValue,
	formValue,
	dashboardValue,
} from "../../../atoms/header";
import { VitalsForm } from "@repo/ui/forms";
import type { VitalsAdd } from "@repo/types/dto";
import { db } from "../../../db/db";
import { useLiveQuery } from "dexie-react-hooks";
import { motion, AnimatePresence } from "framer-motion";

const MotionBox = motion(Box);
const VitalsAddForm = () => {
	const [, setHeaderText] = useAtom(headerText);
	const [, setAddValue] = useAtom(addValue);
	const [, setFormValue] = useAtom(formValue);
	const [, setDashboardValue] = useAtom(dashboardValue);
	const addMutation = AddVitalsDataFn();

	const patientAddId = useParams({
		from: "/vitals/add/$patientId",
		select: (params) => params.patientId,
	});

	const headerData = useLiveQuery(() => db.patients.toArray());
	const headerFilterData = headerData?.find((item) => item.id === patientAddId);
	useMountEffect(() => {
		setHeaderText("Add New Vitals");
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
							<VitalsForm
								onSubmit={(p) => {
									addMutation.mutateAsync(p as VitalsAdd);
								}}
								patientId={patientAddId}
								edit={false}
								lastUpdatedInput={db.getLastUpdated}
							/>
						</MotionBox>
					</Center>
				</Box>
			</AnimatePresence>
		</div>
	);
};

export default VitalsAddForm;
