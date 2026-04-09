import { Box, Center } from "@chakra-ui/react";
import { HeadingTag } from "../../../components/heading-tag";
import { AddProblemDataFn } from "../../../query-mutation-services/problem-data-fn";
import { useParams } from "@tanstack/react-router";
import { useAtom } from "jotai";
import {
	headerText,
	addValue,
	formValue,
	dashboardValue,
} from "../../../atoms/header";
import { useLiveQuery } from "dexie-react-hooks";
import { ProblemForm } from "@repo/ui/forms";
import { useMountEffect } from "@react-hookz/web";
import type { ProblemsAdd } from "@repo/types/dto";
import { db } from "../../../db/db";
import { useEffect, useState } from "react";
import axios from "axios";
import { motion, AnimatePresence } from "framer-motion";

const MotionBox = motion(Box);

export type icd10data = {
	category: string;
	code: string;
	description: string;
	icd_code: string;
	id: number;
	long_description: string;
};

const ProblemAdd = () => {
	const [data, setData] = useState<icd10data[] | undefined>();

	useEffect(() => {
		const fetchData = async () => {
			try {
				const response = await axios.post("http://localhost:8080/icd10", {
					query: "A",
				});

				setData(response.data.hits);
			} catch (error) {
				console.error("Error fetching data:", error);
			}
		};

		fetchData();
	}, []);
	const [, setHeaderText] = useAtom(headerText);
	const [, setAddValue] = useAtom(addValue);
	const [, setFormValue] = useAtom(formValue);
	const [, setDashboardValue] = useAtom(dashboardValue);

	const patientAddId = useParams({
		from: "/problem/add/$patientId",
		select: (params) => params.patientId,
	});

	const addMutation = AddProblemDataFn();
	const headerData = useLiveQuery(() => db.patients.toArray());
	const headerFilterData = headerData?.find((item) => item.id === patientAddId);

	useMountEffect(() => {
		setHeaderText("Add New Problem");
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
							<ProblemForm
								onSubmit={(p) => {
									addMutation.mutateAsync(p as ProblemsAdd);
								}}
								patientId={patientAddId}
								edit={false}
								lastUpdatedInput={db.getLastUpdated}
								icd_data={data}
							/>
						</MotionBox>
					</Center>
				</Box>
			</AnimatePresence>
		</div>
	);
};

export default ProblemAdd;
