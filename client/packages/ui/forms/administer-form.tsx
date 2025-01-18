import {
	Input,
	Box,
	FormControl,
	FormErrorMessage,
	FormLabel,
	Select,
	Textarea,
	Center,
} from "@chakra-ui/react";
import { useForm } from "@felte/react";
import { validator } from "@felte/validator-zod";
import type { z } from "zod";
import { convertToUTC } from "../component/convert-to-utc";
import type {
	AdministerAdd,
	Brand,
	DoctorType,
	LastUpdatedInput,
	Types,
} from "@repo/types/dto";
import { administerAddSchema } from "@repo/types/validation";
import type { AdministerStateExtend } from "@repo/types/dexie-state";
import { SubmitButton } from "../component/submit-button";
import { convertUTCtoLocal } from "../component/utc-date-to-normal-date";
import DateComponent from "../component/date-comp";

export type AdminsterFormProps = {
	onSubmit: (p: AdministerAdd) => void;
	lastUpdatedInput: () => Promise<LastUpdatedInput>;
	initialValues: AdministerStateExtend | undefined;
	patientId: string;
	edit?: boolean;
	serviceLocationList?: string[] | [" "];
	vaccine?: string[];
};
export const AdministerForm = (props: AdminsterFormProps) => {
	if (props.initialValues === undefined && props.edit === true) {
		return <p>Loading...</p>;
	}
	const { form, errors, isSubmitting } = useForm<
		z.infer<typeof administerAddSchema>
	>({
		onSubmit: async (values) => {
			const changedDates = {
				utcDate1: convertToUTC(values.ordered),
				utcDate2: convertToUTC(values.recorded),
				utcDate3: convertToUTC(values.expiration),
				utcDate4: convertToUTC(values.vis_date),
			};
			const modifiedValues = {
				...values,
				last_updated_input: await props.lastUpdatedInput(),
				ordered: changedDates.utcDate1,
				recorded: changedDates.utcDate2,
				expiration: changedDates.utcDate3,
				vis_date: changedDates.utcDate4,
			};
			props.onSubmit(modifiedValues);
		},
		initialValues: {
			patient_id: props.patientId,
			vaccine: props.initialValues?.vaccine || "",
			types: (props.initialValues?.types as Types) || "Type1",
			generic: props.initialValues?.generic || "",
			ordered:
				(props.initialValues?.ordered &&
					convertUTCtoLocal(props.initialValues.ordered)) ||
				"",
			recorded:
				(props.initialValues?.recorded &&
					convertUTCtoLocal(props.initialValues.recorded)) ||
				"",
			dose: props.initialValues?.dose || "",
			site: props.initialValues?.site || "",
			brand: (props.initialValues?.brand as Brand) || "Brand1",
			number_of_serial: props.initialValues?.number_of_series || 1,
			lot: props.initialValues?.lot || 3,
			expiration:
				(props.initialValues?.expiration &&
					convertUTCtoLocal(props.initialValues.expiration)) ||
				"",
			consent_obtain: props.initialValues?.consent_obtain || "",
			administrated_by: props.initialValues?.administrated_by || "",
			provider: (props.initialValues?.provider as DoctorType) || "Doctor",
			vis_date:
				(props.initialValues?.vis_date &&
					convertUTCtoLocal(props.initialValues.vis_date)) ||
				"",
			clinic_location: props.initialValues?.clinic_location || "",
			vfs_financial_class: props.initialValues?.vfs_financial_class || "Mild",
			comments: props.initialValues?.comments || "",
		},
		extend: [validator({ schema: administerAddSchema })],
	});
	return (
		<Center>
			<Box width={{ md: "80%", base: "90%", lg: "70%" }}>
				<form ref={form}>
					<FormLabel mb={2} my={3} color={"#095FBA"}>
						Vaccine
						<span style={{ color: "red" }}>*</span>
					</FormLabel>
					<FormControl mt={2} isInvalid={(errors().vaccine || []).length !== 0}>
						<Select
							placeholder="vaccine"
							name="vaccine"
							bgColor="#FFFFFF"
							borderColor="#095FBA"
						>
							{props.vaccine?.map((vaccine) => (
								<option key={vaccine} value={vaccine}>
									{vaccine}
								</option>
							))}
						</Select>
						{errors().vaccine && (
							<FormErrorMessage>{errors().vaccine}</FormErrorMessage>
						)}
					</FormControl>
					<FormLabel mb={2} my={3} color={"#095FBA"}>
						Type
						<span style={{ color: "red" }}>*</span>
					</FormLabel>
					<FormControl mt={2} isInvalid={(errors().types || []).length !== 0}>
						<Select
							placeholder="Select Type"
							name="types"
							bgColor="#FFFFFF"
							borderColor="#095FBA"
						>
							<option value="Type1">Type 1</option>
							<option value="Type2">Type 2</option>
						</Select>
						{errors().types && (
							<FormErrorMessage>{errors().types}</FormErrorMessage>
						)}
					</FormControl>
					<FormLabel mb={2} my={3} color={"#095FBA"}>
						Brand
						<span style={{ color: "red" }}>*</span>
					</FormLabel>
					<FormControl mt={2} isInvalid={(errors().brand || []).length !== 0}>
						<Select
							placeholder="Select Brand"
							name="brand"
							bgColor="#FFFFFF"
							borderColor="#095FBA"
						>
							<option value="Brand1">Brand 1</option>
							<option value="Brand2">Brand 2</option>
						</Select>
						{errors().brand && (
							<FormErrorMessage>{errors().brand}</FormErrorMessage>
						)}
					</FormControl>
					<FormLabel mb={2} my={3} color={"#095FBA"}>
						Generic
						<span style={{ color: "red" }}>*</span>
					</FormLabel>
					<FormControl mt={2} isInvalid={(errors().generic || []).length !== 0}>
						<Input
							type="text"
							name="generic"
							bgColor="#FFFFFF"
							borderColor="#095FBA"
							placeholder="Static Value"
						/>
						{errors().generic && (
							<FormErrorMessage>{errors().generic}</FormErrorMessage>
						)}
					</FormControl>
					<FormLabel mb={2} my={3} color={"#095FBA"}>
						Ordered Date & Time
						<span style={{ color: "red" }}>*</span>
					</FormLabel>
					<FormControl mt={2} isInvalid={(errors().ordered || []).length !== 0}>
						<DateComponent
							type={"datetime-local"}
							name={"ordered"}
							placeholder={"Select Ordered Date & Time"}
							min={new Date().toISOString().slice(0, 16)}
						/>
						{errors().ordered && (
							<FormErrorMessage>{errors().ordered}</FormErrorMessage>
						)}
					</FormControl>
					<FormLabel mb={2} my={3} color={"#095FBA"}>
						Recorded Date & Time
						<span style={{ color: "red" }}>*</span>
					</FormLabel>
					<FormControl
						mt={2}
						isInvalid={(errors().recorded || []).length !== 0}
					>
						<DateComponent
							type={"datetime-local"}
							name={"recorded"}
							placeholder={"Select Recorded Date & Time"}
							min={new Date().toISOString().slice(0, 16)}
						/>
						{errors().recorded && (
							<FormErrorMessage>{errors().recorded}</FormErrorMessage>
						)}
					</FormControl>
					<FormLabel mb={2} my={3} color={"#095FBA"}>
						Dose
						<span style={{ color: "red" }}>*</span>
					</FormLabel>
					<FormControl mt={2} isInvalid={(errors().dose || []).length !== 0}>
						<Input
							type="text"
							name="dose"
							bgColor="#FFFFFF"
							borderColor="#095FBA"
							placeholder="Enter Dose"
						/>
						{errors().dose && (
							<FormErrorMessage>{errors().dose}</FormErrorMessage>
						)}
					</FormControl>
					<FormLabel mb={2} my={3} color={"#095FBA"}>
						Site
						<span style={{ color: "red" }}>*</span>
					</FormLabel>
					<FormControl mt={2} isInvalid={(errors().site || []).length !== 0}>
						<Select
							placeholder="Select Site"
							name="site"
							bgColor="#FFFFFF"
							borderColor="#095FBA"
						>
							<option value="Site1">Site 1</option>
							<option value="Site2">Site 2</option>
						</Select>
						{errors().site && (
							<FormErrorMessage>{errors().site}</FormErrorMessage>
						)}
					</FormControl>
					<FormLabel mb={2} my={3} color={"#095FBA"}>
						Number in Series
						<span style={{ color: "red" }}>*</span>
					</FormLabel>
					<FormControl
						mt={2}
						isInvalid={(errors().number_of_serial || []).length !== 0}
					>
						<Input
							type="number"
							name="number_of_serial"
							bgColor="#FFFFFF"
							borderColor="#095FBA"
							placeholder="Enter Number"
						/>
						{errors().number_of_serial && (
							<FormErrorMessage>{errors().number_of_serial}</FormErrorMessage>
						)}
					</FormControl>
					<FormLabel mb={2} my={3} color={"#095FBA"}>
						Lot#
						<span style={{ color: "red" }}>*</span>
					</FormLabel>
					<FormControl mt={2} isInvalid={(errors().lot || []).length !== 0}>
						<Input
							type="number"
							name="lot"
							bgColor="#FFFFFF"
							borderColor="#095FBA"
							placeholder="Enter Lot Number"
						/>
						{errors().lot && (
							<FormErrorMessage>{errors().lot}</FormErrorMessage>
						)}
					</FormControl>
					<FormLabel mb={2} my={3} color={"#095FBA"}>
						Expiration
						<span style={{ color: "red" }}>*</span>
					</FormLabel>
					<FormControl
						mt={2}
						isInvalid={(errors().expiration || []).length !== 0}
					>
						<DateComponent
							type={"datetime-local"}
							name={"expiration"}
							placeholder={"Select Expiration Date & Time"}
						/>
						{errors().expiration && (
							<FormErrorMessage>{errors().expiration}</FormErrorMessage>
						)}
					</FormControl>
					<FormLabel mb={2} my={3} color={"#095FBA"}>
						Consent Obtained
						<span style={{ color: "red" }}>*</span>
					</FormLabel>
					<FormControl
						mt={2}
						isInvalid={(errors().consent_obtain || []).length !== 0}
					>
						<Select
							placeholder="Select Consent Obtained"
							name="consent_obtain"
							bgColor="#FFFFFF"
							borderColor="#095FBA"
						>
							<option value="Verbal">Verbal</option>
							<option value="Written">Written</option>
						</Select>
						{errors().consent_obtain && (
							<FormErrorMessage>{errors().consent_obtain}</FormErrorMessage>
						)}
					</FormControl>
					<FormLabel mb={2} my={3} color={"#095FBA"}>
						Administered By
						<span style={{ color: "red" }}>*</span>
					</FormLabel>
					<FormControl
						mt={2}
						isInvalid={(errors().administrated_by || []).length !== 0}
					>
						<Select
							placeholder="Select Administered"
							name="administrated_by"
							bgColor="#FFFFFF"
							borderColor="#095FBA"
						>
							<option value="Mild">Mild</option>
							<option value="Moderate">Moderate</option>
							<option value="Severe">Severe</option>
						</Select>
						{errors().administrated_by && (
							<FormErrorMessage>{errors().administrated_by}</FormErrorMessage>
						)}
					</FormControl>
					<FormLabel mb={2} my={3} color={"#095FBA"}>
						Service Location
						<span style={{ color: "red" }}>*</span>
					</FormLabel>
					<FormControl
						mt={2}
						isInvalid={(errors().clinic_location || []).length !== 0}
					>
						<Select
							placeholder="Select Service Location"
							name="clinic_location"
							bgColor="#FFFFFF"
							borderColor="#095FBA"
						>
							{props.serviceLocationList?.map((serviceLocation) => (
								<option key={serviceLocation} value={serviceLocation}>
									{serviceLocation}
								</option>
							))}
						</Select>
						{errors().clinic_location && (
							<FormErrorMessage>{errors().clinic_location}</FormErrorMessage>
						)}
					</FormControl>
					<FormLabel mb={2} my={3} color={"#095FBA"}>
						Provider
						<span style={{ color: "red" }}>*</span>
					</FormLabel>
					<FormControl
						mt={2}
						isInvalid={(errors().provider || []).length !== 0}
					>
						<Select
							placeholder="Select Provider"
							name="provider"
							bgColor="#FFFFFF"
							borderColor="#095FBA"
						>
							<option value="FamilyMedicinePhysician">
								FamilyMedicinePhysician
							</option>
							<option value="Pediatrician">Pediatrician</option>
							<option value="Gynecologist">Gynecologist</option>
							<option value="Cardiologist">Cardiologist</option>
							<option value="Pharmacist">Pharmacist</option>
							<option value="Dermatologist">Dermatologist</option>
							<option value="Psychiatrist">Psychiatrist</option>
							<option value="Surgeon">Surgeon</option>
							<option value="Doctor">Doctor</option>
						</Select>
						{errors().provider && (
							<FormErrorMessage>{errors().provider}</FormErrorMessage>
						)}
					</FormControl>
					<FormLabel mb={2} my={3} color={"#095FBA"}>
						VIS Date
						<span style={{ color: "red" }}>*</span>
					</FormLabel>
					<FormControl
						mt={2}
						isInvalid={(errors().vis_date || []).length !== 0}
					>
						<DateComponent
							type={"datetime-local"}
							name={"vis_date"}
							placeholder={"Select VIS Date"}
						/>
						{errors().vis_date && (
							<FormErrorMessage>{errors().vis_date}</FormErrorMessage>
						)}
					</FormControl>
					<FormLabel mb={2} my={3} color={"#095FBA"}>
						VFC Financial Class
						<span style={{ color: "red" }}>*</span>
					</FormLabel>
					<FormControl
						mt={2}
						isInvalid={(errors().vfs_financial_class || []).length !== 0}
					>
						<Select
							placeholder="Select VFC Financial Class"
							name="vfs_financial_class"
							bgColor="#FFFFFF"
							borderColor="#095FBA"
						>
							<option value="Mild">Mild</option>
							<option value="Moderate">Moderate</option>
							<option value="Severe">Severe</option>
						</Select>
						{errors().vfs_financial_class && (
							<FormErrorMessage>
								{errors().vfs_financial_class}
							</FormErrorMessage>
						)}
					</FormControl>
					<FormLabel mb={2} my={3} color={"#095FBA"}>
						Comments
						<span style={{ color: "red" }}>*</span>
					</FormLabel>
					<FormControl
						mt={2}
						isInvalid={(errors().comments || []).length !== 0}
					>
						<Textarea
							bgColor="#FFFFFF"
							name="comments"
							borderColor="#095FBA"
							placeholder="Comments"
							maxLength={100}
						/>
						{errors().comments && (
							<FormErrorMessage>{errors().comments}</FormErrorMessage>
						)}
					</FormControl>
					<SubmitButton loading={isSubmitting()} />
				</form>
			</Box>
		</Center>
	);
};
