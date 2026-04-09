import { Box, Center } from "@chakra-ui/react";
import { AddMedicationDataFn } from "../../../query-mutation-services/medication-data-fn";
import { useParams } from "@tanstack/react-router";
import { useAtom } from "jotai";
import {
	headerText,
	addValue,
	formValue,
	dashboardValue,
} from "../../../atoms/header";
import { MedicationForm } from "@repo/ui/forms";
import type { MedicationsAdd } from "@repo/types/dto";
import { db } from "../../../db/db";
import { useMountEffect } from "@react-hookz/web";
import { useLiveQuery } from "dexie-react-hooks";

import { GetDrugsDataFn } from "../../../query-mutation-services/organization-data-fn";
import { HeadingTag } from "../../../components/heading-tag";
import axios from "axios";
import { useState, useEffect } from "react";
import type { drugData } from "../../prescription/prescription-add";
import { motion, AnimatePresence } from "framer-motion";

const MotionBox = motion(Box);

const MedicationAddForm = () => {
	const [, setHeaderText] = useAtom(headerText);
	const [, setAddValue] = useAtom(addValue);
	const [, setFormValue] = useAtom(formValue);
	const [, setDashboardValue] = useAtom(dashboardValue);
	const addMutation = AddMedicationDataFn();

	const patientAddId = useParams({
		from: "/medication/add/$patientId",
		select: (params) => params.patientId,
	});

	const patient = useLiveQuery(() => db.patients.toArray());

	const drugQuery = GetDrugsDataFn();
	const drugsArray = Array.isArray(drugQuery.data?.result)
		? drugQuery.data?.result
		: [];
	const brandNames = (drugsArray as { Brand?: string }[]).map(
		(drug) => drug.Brand || "",
	);
	const headerData = useLiveQuery(() => db.patients.toArray());
	const headerFilterData = headerData?.find((item) => item.id === patientAddId);

	const [data, setData] = useState<drugData[] | undefined>();

	useEffect(() => {
		const fetchData = async () => {
			try {
				const response = await axios.post("http://localhost:8080/drugs", {
					query: "",
				});

				setData(response.data.hits);
			} catch (error) {
				console.error("Error fetching data:", error);
			}
		};

		fetchData();
	}, []);

	useMountEffect(() => {
		setHeaderText("Add New Medication");
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
							<MedicationForm
								onSubmit={(p) => {
									addMutation.mutateAsync(p as MedicationsAdd);
								}}
								lastUpdatedInput={db.getLastUpdated}
								drug={brandNames}
								patient={patient}
								edit={false}
								drug_name={data}
								patientId={patientAddId}
							/>
						</MotionBox>
					</Center>
				</Box>
			</AnimatePresence>
		</div>
	);
};

export default MedicationAddForm;
