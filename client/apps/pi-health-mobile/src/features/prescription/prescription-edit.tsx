import { Box } from "@chakra-ui/react";
import { UpdatePrescriptionFn } from "../../query-mutation-services/prescription-data-fn";
import { useParams } from "@tanstack/react-router";
import {
	addValue,
	dashboardValue,
	displayMenu,
	formValue,
	headerText,
} from "../../atoms/header";
import { useAtom } from "jotai";
import { PrescriptionFrom } from "@repo/ui/forms";
import type { PrescriptionUpdate, PrescriptionAdd } from "@repo/types/dto";
import { db } from "../../db/db";
import { useMountEffect } from "@react-hookz/web";
import { useLiveQuery } from "dexie-react-hooks";
import axios from "axios";
import { useState, useEffect } from "react";
import { motion, AnimatePresence } from "framer-motion";

const MotionBox = motion(Box);

export type drugData = {
	Brand: string;
	Generic: string;
	Details: string;
	Category: string;
	SideEffects: string;
	DrugDoseInfo: string;
	Precaution: string;
	ManufacturerName: string;
	Medicines: string;
	ContraIndications: string;
	Diseases: string;
	Interactions: string;
	Contains: string;
	id: number;
};

const PrescriptionEditForm = () => {
	const [, setHeaderText] = useAtom(headerText);
	const [, setAddValue] = useAtom(addValue);
	const [, setFormValue] = useAtom(formValue);
	const [, setDashboardValue] = useAtom(dashboardValue);

	const [, setmenu] = useAtom(displayMenu);
	const patient = useLiveQuery(() => db.patients.toArray());
	const prescription = useLiveQuery(() => db.prescription.toArray());
	const prescriptionId = useParams({
		from: "/prescription/edit/$prescriptionId",
		select: (params) => params.prescriptionId,
	});

	const updateMutation = UpdatePrescriptionFn();
	const filteredData = prescription?.find((item) => item.id === prescriptionId);
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
		setHeaderText("Edit Prescription");
		setAddValue(true);
		setFormValue(true);
		setDashboardValue(false);
		setmenu(false);
	});
	return (
		<div>
			<AnimatePresence>
				<Box bgColor={"#F7F7F9"}>
					<MotionBox
						initial={{ opacity: 0, x: 50 }}
						animate={{ opacity: 1, x: 0 }}
						exit={{ opacity: 0, x: 50 }}
						transition={{ duration: 0.65 }}
					>
						<PrescriptionFrom
							onSubmit={(p) => {
								const editVal: PrescriptionUpdate = {
									id: prescriptionId,
									input: { ...(p as PrescriptionAdd) },
								};
								updateMutation.mutateAsync(editVal);
							}}
							prescriptionId={prescriptionId}
							patient={patient}
							initialValues={filteredData}
							lastUpdatedInput={db.getLastUpdated}
							edit={true}
							drug_name={data}
						/>
					</MotionBox>
				</Box>
			</AnimatePresence>
		</div>
	);
};

export default PrescriptionEditForm;
