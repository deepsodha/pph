import { OrganizationSync } from "@repo/services/src";
import type { LastUpdatedInput, SyncData } from "@repo/types/dto";
import {
	type AdministerStateExtend,
	type AllergiesStateExtend,
	type AppointmentStateExtend,
	type DoctorStateExtend,
	type FamilyHistoryStateExtend,
	type HistoricalStateExtend,
	type HospitalizationStateExtend,
	type ImplantableDevicesStateExtend,
	type MedicationsStateExtend,
	type NotAdministeredStateExtend,
	type OBandPregnancyStateExtend,
	type OrderStateExtend,
	type PastMedicalHistoryStateExtend,
	type PastSurgicalHistoryStateExtend,
	type PatientStateExtend,
	type PrescriptionStateExtend,
	type ProblemStateExtend,
	type ServiceLocationStateExtend,
	type SocialHistoryStateExtend,
	type SystemAdminStateExtend,
	type VitalsStateExtend,
	mapAdminister,
	mapAllergy,
	mapAppointment,
	mapDoctor,
	mapFamilyHistory,
	mapHistorical,
	mapHospitalization,
	mapImplantableDevices,
	mapMedication,
	mapNotAdminister,
	mapOBandPregnancy,
	mapOrder,
	mapPastMedicalHistory,
	mapPastSurgicalHistory,
	mapPatient,
	mapPrescription,
	mapProblem,
	mapServiceLocation,
	mapSocialHistory,
	mapSystemAdmin,
	mapVital,
	type StaffStateExtend,
	mapStaff,
	type UserStateExtend,
	mapUser,
	type OrganizationStateExtend,
	mapOrganization,
	type NoteStateExtend,
	mapNote,
} from "@repo/types/dexie-state";
import Dexie, { type Table } from "dexie";

class PiHealthDexie extends Dexie {
	patients!: Table<PatientStateExtend>;
	doctors!: Table<DoctorStateExtend>;
	appointments!: Table<AppointmentStateExtend>;
	prescription!: Table<PrescriptionStateExtend>;
	servicelocation!: Table<ServiceLocationStateExtend>;
	vitals!: Table<VitalsStateExtend>;
	allergy!: Table<AllergiesStateExtend>;
	problems!: Table<ProblemStateExtend>;
	historical!: Table<HistoricalStateExtend>;
	medications!: Table<MedicationsStateExtend>;
	administer!: Table<AdministerStateExtend>;
	notadminister!: Table<NotAdministeredStateExtend>;
	systemadmin!: Table<SystemAdminStateExtend>;
	orders!: Table<OrderStateExtend>;
	familyhistory!: Table<FamilyHistoryStateExtend>;
	hospitalization!: Table<HospitalizationStateExtend>;
	implantabledevices!: Table<ImplantableDevicesStateExtend>;
	obandpregnanacy!: Table<OBandPregnancyStateExtend>;
	pastmedicalhistory!: Table<PastMedicalHistoryStateExtend>;
	pastsurgicalhistory!: Table<PastSurgicalHistoryStateExtend>;
	socialhistory!: Table<SocialHistoryStateExtend>;
	staff!: Table<StaffStateExtend>;
	note!: Table<NoteStateExtend>;
	user!: Table<UserStateExtend>;
	organization!: Table<OrganizationStateExtend>;
	interval!: number;
	constructor() {
		super("pi-health");
		this.version(37).stores({
			patients: "&id,last_updated",
			doctors: "&id,last_updated",
			appointments: "&id,last_updated",
			prescription: "&id,last_updated",
			servicelocation: "&id,last_updated",
			vitals: "&id,last_updated",
			allergy: "&id,last_updated",
			problems: "&id,last_updated",
			historical: "&id,last_updated",
			medications: "&id,last_updated",
			administer: "&id,last_updated",
			notadminister: "&id,last_updated",
			orders: "&id,last_updated",
			systemadmin: "&id,last_updated",
			familyhistory: "&id,last_updated",
			hospitalization: "&id,last_updated",
			implantabledevices: "&id,last_updated",
			obandpregnanacy: "&id,last_updated",
			pastmedicalhistory: "&id,last_updated",
			pastsurgicalhistory: "&id,last_updated",
			socialhistory: "&id,last_updated",
			staff: "&id,last_updated",
			historyandphysical: "&id,last_updated",
			soap: "&id,last_updated",
			user: "&id,last_updated",
			organization: "&id,last_updated",
			note: "&id,last_updated",
		});
	}

	async reset() {
		this.patients.clear();
		this.doctors.clear();
		this.appointments.clear();
		this.prescription.clear();
		this.servicelocation.clear();
		this.vitals.clear();
		this.allergy.clear();
		this.problems.clear();
		this.historical.clear();
		this.medications.clear();
		this.administer.clear();
		this.notadminister.clear();
		this.orders.clear();
		this.systemadmin.clear();
		this.familyhistory.clear();
		this.hospitalization.clear();
		this.implantabledevices.clear();
		this.obandpregnanacy.clear();
		this.pastmedicalhistory.clear();
		this.pastsurgicalhistory.clear();
		this.socialhistory.clear();
		this.staff.clear();
		this.note.clear();
		this.user.clear();
		this.organization.clear();
	}

	async sync() {
		const lastUpdatedInput = await this.getLastUpdated();
		const res = await OrganizationSync(lastUpdatedInput);
		if (res.result) {
			this.bulkPut({
				patients: res.result.patients,
				system_admin: res.result.system_admin,
				doctors: res.result.doctors,
				appointments: res.result.appointments,
				prescription: res.result.prescription,
				service_location: res.result.service_location,
				add_historical: res.result.add_historical,
				administer: res.result.administer,
				allergy: res.result.allergy,
				medication: res.result.medication,
				not_administer: res.result.not_administer,
				order: res.result.order,
				problems: res.result.problems,
				vitals: res.result.vitals,
				familyhistory: res.result.familyhistory,
				hospitalization: res.result.hospitalization,
				implantabledevices: res.result.implantabledevices,
				obandpregnancy: res.result.obandpregnancy,
				pastmedicalhistory: res.result.pastmedicalhistory,
				pastsurgicalhistory: res.result.pastsurgicalhistory,
				socialhistory: res.result.socialhistory,
				staff: res.result.staff,
				user: res.result.user,
				organization: res.result.organization,
				note: res.result.note,
			});
		}

		if (res.error) {
			//TODO: somehow push it to UI
		}
	}

	async bulkPut(input: SyncData) {
		this.patients.bulkPut(input.patients.map(mapPatient));
		this.doctors.bulkPut(input.doctors.map(mapDoctor));
		this.appointments.bulkPut(input.appointments.map(mapAppointment));
		this.prescription.bulkPut(input.prescription.map(mapPrescription));
		this.servicelocation.bulkPut(
			input.service_location.map(mapServiceLocation),
		);
		this.vitals.bulkPut(input.vitals.map(mapVital));
		this.allergy.bulkPut(input.allergy.map(mapAllergy));
		this.historical.bulkPut(input.add_historical.map(mapHistorical));
		this.problems.bulkPut(input.problems.map(mapProblem));
		this.medications.bulkPut(input.medication.map(mapMedication));
		this.administer.bulkPut(input.administer.map(mapAdminister));
		this.notadminister.bulkPut(input.not_administer.map(mapNotAdminister));
		this.orders.bulkPut(input.order.map(mapOrder));
		this.systemadmin.bulkPut(input.system_admin.map(mapSystemAdmin));
		this.familyhistory.bulkPut(input.familyhistory.map(mapFamilyHistory));
		this.hospitalization.bulkPut(input.hospitalization.map(mapHospitalization));
		this.implantabledevices.bulkPut(
			input.implantabledevices.map(mapImplantableDevices),
		);
		this.obandpregnanacy.bulkPut(input.obandpregnancy.map(mapOBandPregnancy));
		this.pastmedicalhistory.bulkPut(
			input.pastmedicalhistory.map(mapPastMedicalHistory),
		);
		this.pastsurgicalhistory.bulkPut(
			input.pastsurgicalhistory.map(mapPastSurgicalHistory),
		);
		this.socialhistory.bulkPut(input.socialhistory.map(mapSocialHistory));
		this.staff.bulkPut(input.staff.map(mapStaff));
		this.note.bulkPut(input.note.map(mapNote));
		this.user.bulkPut(input.user.map(mapUser));
		this.organization.bulkPut(input.organization.map(mapOrganization));
	}

	getLastUpdated = async (): Promise<LastUpdatedInput> => {
		const getLastUpdatedDate = async <T extends { last_updated: Date }>(
			table: Table<T>,
		): Promise<string> => {
			const result = await table.orderBy("last_updated").last();

			return result?.last_updated instanceof Date
				? result.last_updated.toISOString()
				: String("1970-01-01T00:00:00.000Z");
		};
		return {
			system_admin: await getLastUpdatedDate(this.systemadmin),
			doctors: await getLastUpdatedDate(this.doctors),
			patients: await getLastUpdatedDate(this.patients),
			appointments: await getLastUpdatedDate(this.appointments),
			prescription: await getLastUpdatedDate(this.prescription),
			service_location: await getLastUpdatedDate(this.servicelocation),
			add_historical: await getLastUpdatedDate(this.historical),
			administer: await getLastUpdatedDate(this.administer),
			allergy: await getLastUpdatedDate(this.allergy),
			medication: await getLastUpdatedDate(this.medications),
			not_administer: await getLastUpdatedDate(this.notadminister),
			order: await getLastUpdatedDate(this.orders),
			problems: await getLastUpdatedDate(this.problems),
			vitals: await getLastUpdatedDate(this.vitals),
			familyhistory: await getLastUpdatedDate(this.familyhistory),
			hospitalization: await getLastUpdatedDate(this.hospitalization),
			implantabledevices: await getLastUpdatedDate(this.implantabledevices),
			obandpregnanacy: await getLastUpdatedDate(this.obandpregnanacy),
			pastmedicalhistory: await getLastUpdatedDate(this.pastmedicalhistory),
			pastsurgicalhistory: await getLastUpdatedDate(this.pastsurgicalhistory),
			socialhistory: await getLastUpdatedDate(this.socialhistory),
			staff: await getLastUpdatedDate(this.staff),
			note: await getLastUpdatedDate(this.note),
			user: await getLastUpdatedDate(this.user),
			organization: await getLastUpdatedDate(this.organization),
		};
	};

	/**
	 * @param timeInSecond number is given for time in seconds
	 */
	async startSync(timeInSecond: number) {
		await this.sync();
		this.interval = setInterval(async () => {
			await this.sync();
		}, timeInSecond * 1000);
	}

	/**
	 * @name StopSync
	 * @description It will clear the Interval and stop the sync.
	 * Also, it will reset the whole database
	 */
	async stopSync() {
		clearInterval(this.interval);
		this.reset();
	}
}

export const db = new PiHealthDexie();
