const concatNamespace = (
	externalNamespace: string,
	internalNamespace: string,
): string => {
	if (externalNamespace !== "") {
		return concatMethodNames(externalNamespace, internalNamespace);
	}
	return internalNamespace;
};

export const concatMethodNames = (
	namespace: string,
	method: string,
): string => {
	return `${namespace}::${method}`;
};

class Auth {
	private readonly internalNamespace = "Auth";
	constructor(private externalNamespace: string) {}

	get LoginMobile() {
		return concatMethodNames(
			concatNamespace(this.externalNamespace, this.internalNamespace),
			"LoginMobile",
		);
	}
}

class Patients {
	private readonly internalNamespace = "Patients";
	constructor(private externalNamespace: string) {}

	get Add() {
		return concatMethodNames(
			concatNamespace(this.externalNamespace, this.internalNamespace),
			"Add",
		);
	}
	get Update() {
		return concatMethodNames(
			concatNamespace(this.externalNamespace, this.internalNamespace),
			"Update",
		);
	}
	get Delete() {
		return concatMethodNames(
			concatNamespace(this.externalNamespace, this.internalNamespace),
			"Delete",
		);
	}
	get Vitals() {
		return new Vitals(
			concatNamespace(this.externalNamespace, this.internalNamespace),
		);
	}
	get Allergies() {
		return new Allergies(
			concatNamespace(this.externalNamespace, this.internalNamespace),
		);
	}
	get Problems() {
		return new Problems(
			concatNamespace(this.externalNamespace, this.internalNamespace),
		);
	}

	get Order() {
		return new Order(
			concatNamespace(this.externalNamespace, this.internalNamespace),
		);
	}

	get AddHistorical() {
		return new AddHistorical(
			concatNamespace(this.externalNamespace, this.internalNamespace),
		);
	}

	get Addminister() {
		return new Addminister(
			concatNamespace(this.externalNamespace, this.internalNamespace),
		);
	}

	get Notaddminister() {
		return new Notaddminister(
			concatNamespace(this.externalNamespace, this.internalNamespace),
		);
	}

	get Medications() {
		return new Medications(
			concatNamespace(this.externalNamespace, this.internalNamespace),
		);
	}

	get History() {
		return new History(
			concatNamespace(this.externalNamespace, this.internalNamespace),
		);
	}
	get Notes() {
		return new Notes(
			concatNamespace(this.externalNamespace, this.internalNamespace),
		);
	}
}

class Vitals {
	private readonly internalNamespace = "Vitals";
	constructor(private externalNamespace: string) {}

	get Add() {
		return concatMethodNames(
			concatNamespace(this.externalNamespace, this.internalNamespace),
			"Add",
		);
	}
	get Update() {
		return concatMethodNames(
			concatNamespace(this.externalNamespace, this.internalNamespace),
			"Update",
		);
	}
	get Delete() {
		return concatMethodNames(
			concatNamespace(this.externalNamespace, this.internalNamespace),
			"Delete",
		);
	}
}

class Soap {
	private readonly internalNamespace = "Soap";
	constructor(private externalNamespace: string) {}

	get Add() {
		return concatMethodNames(
			concatNamespace(this.externalNamespace, this.internalNamespace),
			"Add",
		);
	}
	get Update() {
		return concatMethodNames(
			concatNamespace(this.externalNamespace, this.internalNamespace),
			"Update",
		);
	}
}

class HistoryAndPhysical {
	private readonly internalNamespace = "HistoryAndPhysical";
	constructor(private externalNamespace: string) {}

	get Add() {
		return concatMethodNames(
			concatNamespace(this.externalNamespace, this.internalNamespace),
			"Add",
		);
	}

	get Update() {
		return concatMethodNames(
			concatNamespace(this.externalNamespace, this.internalNamespace),
			"Update",
		);
	}
}
class Allergies {
	private readonly internalNamespace = "Allergies";
	constructor(private externalNamespace: string) {}

	get Add() {
		return concatMethodNames(
			concatNamespace(this.externalNamespace, this.internalNamespace),
			"Add",
		);
	}
	get Update() {
		return concatMethodNames(
			concatNamespace(this.externalNamespace, this.internalNamespace),
			"Update",
		);
	}
	get Delete() {
		return concatMethodNames(
			concatNamespace(this.externalNamespace, this.internalNamespace),
			"Delete",
		);
	}
	get Name() {
		return concatMethodNames(
			concatNamespace(this.externalNamespace, this.internalNamespace),
			"Name",
		);
	}
}

class Problems {
	private readonly internalNamespace = "Problems";
	constructor(private externalNamespace: string) {}

	get Add() {
		return concatMethodNames(
			concatNamespace(this.externalNamespace, this.internalNamespace),
			"Add",
		);
	}
	get Update() {
		return concatMethodNames(
			concatNamespace(this.externalNamespace, this.internalNamespace),
			"Update",
		);
	}
	get Delete() {
		return concatMethodNames(
			concatNamespace(this.externalNamespace, this.internalNamespace),
			"Delete",
		);
	}
}

class Order {
	private readonly internalNamespace = "Order";
	constructor(private externalNamespace: string) {}

	get Add() {
		return concatMethodNames(
			concatNamespace(this.externalNamespace, this.internalNamespace),
			"Add",
		);
	}

	get Update() {
		return concatMethodNames(
			concatNamespace(this.externalNamespace, this.internalNamespace),
			"Update",
		);
	}

	get Delete() {
		return concatMethodNames(
			concatNamespace(this.externalNamespace, this.internalNamespace),
			"Delete",
		);
	}
}

class AddHistorical {
	private readonly internalNamespace = "AddHistorical";
	constructor(private externalNamespace: string) {}

	get Add() {
		return concatMethodNames(
			concatNamespace(this.externalNamespace, this.internalNamespace),
			"Add",
		);
	}
	get Update() {
		return concatMethodNames(
			concatNamespace(this.externalNamespace, this.internalNamespace),
			"Update",
		);
	}
	get Delete() {
		return concatMethodNames(
			concatNamespace(this.externalNamespace, this.internalNamespace),
			"Delete",
		);
	}
}

class Addminister {
	private readonly internalNamespace = "Addminister";
	constructor(private externalNamespace: string) {}

	get Add() {
		return concatMethodNames(
			concatNamespace(this.externalNamespace, this.internalNamespace),
			"Add",
		);
	}
	get Update() {
		return concatMethodNames(
			concatNamespace(this.externalNamespace, this.internalNamespace),
			"Update",
		);
	}
	get Delete() {
		return concatMethodNames(
			concatNamespace(this.externalNamespace, this.internalNamespace),
			"Delete",
		);
	}
}

class Notaddminister {
	private readonly internalNamespace = "Notaddminister";
	constructor(private externalNamespace: string) {}

	get Add() {
		return concatMethodNames(
			concatNamespace(this.externalNamespace, this.internalNamespace),
			"Add",
		);
	}
	get Update() {
		return concatMethodNames(
			concatNamespace(this.externalNamespace, this.internalNamespace),
			"Update",
		);
	}
	get Delete() {
		return concatMethodNames(
			concatNamespace(this.externalNamespace, this.internalNamespace),
			"Delete",
		);
	}
}

class Medications {
	private readonly internalNamespace = "Medications";
	constructor(private externalNamespace: string) {}

	get Add() {
		return concatMethodNames(
			concatNamespace(this.externalNamespace, this.internalNamespace),
			"Add",
		);
	}
	get Update() {
		return concatMethodNames(
			concatNamespace(this.externalNamespace, this.internalNamespace),
			"Update",
		);
	}
	get Delete() {
		return concatMethodNames(
			concatNamespace(this.externalNamespace, this.internalNamespace),
			"Delete",
		);
	}
}
class Doctor {
	private readonly internalNamespace = "Doctor";
	constructor(private externalNamespace: string) {}

	get Add() {
		return concatMethodNames(
			concatNamespace(this.externalNamespace, this.internalNamespace),
			"Add",
		);
	}
	get Update() {
		return concatMethodNames(
			concatNamespace(this.externalNamespace, this.internalNamespace),
			"Update",
		);
	}
	get Delete() {
		return concatMethodNames(
			concatNamespace(this.externalNamespace, this.internalNamespace),
			"Delete",
		);
	}
}
class ServiceLocation {
	private readonly internalNamespace = "ServiceLocation";
	constructor(private externalNamespace: string) {}

	get Add() {
		return concatMethodNames(
			concatNamespace(this.externalNamespace, this.internalNamespace),
			"Add",
		);
	}
	get Update() {
		return concatMethodNames(
			concatNamespace(this.externalNamespace, this.internalNamespace),
			"Update",
		);
	}
	get Delete() {
		return concatMethodNames(
			concatNamespace(this.externalNamespace, this.internalNamespace),
			"Delete",
		);
	}
	get Select() {
		return concatMethodNames(
			concatNamespace(this.externalNamespace, this.internalNamespace),
			"Select",
		);
	}
}
class History {
	private readonly internalNamespace = "History";
	constructor(private externalNamespace: string) {}

	get FamilyHistory() {
		return new FamilyHistory(
			concatNamespace(this.externalNamespace, this.internalNamespace),
		);
	}

	get PastMedicalHistory() {
		return new PastMedicalHistory(
			concatNamespace(this.externalNamespace, this.internalNamespace),
		);
	}
	get Hospitalization() {
		return new Hospitalization(
			concatNamespace(this.externalNamespace, this.internalNamespace),
		);
	}

	get ImplantableDevices() {
		return new ImplantableDevices(
			concatNamespace(this.externalNamespace, this.internalNamespace),
		);
	}

	get SocialHistory() {
		return new SocialHistory(
			concatNamespace(this.externalNamespace, this.internalNamespace),
		);
	}

	get PastSurgicalHistory() {
		return new PastSurgicalHistory(
			concatNamespace(this.externalNamespace, this.internalNamespace),
		);
	}

	get ObAndPregnancy() {
		return new ObAndPregnancy(
			concatNamespace(this.externalNamespace, this.internalNamespace),
		);
	}
}
class Notes {
	private readonly internalNamespace = "Notes";
	constructor(private externalNamespace: string) {}

	get Soap() {
		return new Soap(
			concatNamespace(this.externalNamespace, this.internalNamespace),
		);
	}
	get HistoryAndPhysical() {
		return new HistoryAndPhysical(
			concatNamespace(this.externalNamespace, this.internalNamespace),
		);
	}
}
class FamilyHistory {
	private readonly internalNamespace = "FamilyHistory";
	constructor(private externalNamespace: string) {}

	get Add() {
		return concatMethodNames(
			concatNamespace(this.externalNamespace, this.internalNamespace),
			"Add",
		);
	}
	get Update() {
		return concatMethodNames(
			concatNamespace(this.externalNamespace, this.internalNamespace),
			"Update",
		);
	}
	get Delete() {
		return concatMethodNames(
			concatNamespace(this.externalNamespace, this.internalNamespace),
			"Delete",
		);
	}
}
class ImplantableDevices {
	private readonly internalNamespace = "ImplantableDevices";
	constructor(private externalNamespace: string) {}

	get Add() {
		return concatMethodNames(
			concatNamespace(this.externalNamespace, this.internalNamespace),
			"Add",
		);
	}
	get Update() {
		return concatMethodNames(
			concatNamespace(this.externalNamespace, this.internalNamespace),
			"Update",
		);
	}
	get Delete() {
		return concatMethodNames(
			concatNamespace(this.externalNamespace, this.internalNamespace),
			"Delete",
		);
	}
}
class SocialHistory {
	private readonly internalNamespace = "SocialHistory";
	constructor(private externalNamespace: string) {}

	get Add() {
		return concatMethodNames(
			concatNamespace(this.externalNamespace, this.internalNamespace),
			"Add",
		);
	}
	get Update() {
		return concatMethodNames(
			concatNamespace(this.externalNamespace, this.internalNamespace),
			"Update",
		);
	}
	get Delete() {
		return concatMethodNames(
			concatNamespace(this.externalNamespace, this.internalNamespace),
			"Delete",
		);
	}
}
class ObAndPregnancy {
	private readonly internalNamespace = "ObAndPregnancy";
	constructor(private externalNamespace: string) {}

	get Add() {
		return concatMethodNames(
			concatNamespace(this.externalNamespace, this.internalNamespace),
			"Add",
		);
	}
	get Update() {
		return concatMethodNames(
			concatNamespace(this.externalNamespace, this.internalNamespace),
			"Update",
		);
	}
	get Delete() {
		return concatMethodNames(
			concatNamespace(this.externalNamespace, this.internalNamespace),
			"Delete",
		);
	}
}
class PastSurgicalHistory {
	private readonly internalNamespace = "PastSurgicalHistory";
	constructor(private externalNamespace: string) {}

	get Add() {
		return concatMethodNames(
			concatNamespace(this.externalNamespace, this.internalNamespace),
			"Add",
		);
	}
	get Update() {
		return concatMethodNames(
			concatNamespace(this.externalNamespace, this.internalNamespace),
			"Update",
		);
	}
	get Delete() {
		return concatMethodNames(
			concatNamespace(this.externalNamespace, this.internalNamespace),
			"Delete",
		);
	}
}
class PastMedicalHistory {
	private readonly internalNamespace = "PastMedicalHistory";
	constructor(private externalNamespace: string) {}

	get Add() {
		return concatMethodNames(
			concatNamespace(this.externalNamespace, this.internalNamespace),
			"Add",
		);
	}
	get Update() {
		return concatMethodNames(
			concatNamespace(this.externalNamespace, this.internalNamespace),
			"Update",
		);
	}
	get Delete() {
		return concatMethodNames(
			concatNamespace(this.externalNamespace, this.internalNamespace),
			"Delete",
		);
	}
}
class Hospitalization {
	private readonly internalNamespace = "Hospitalization";
	constructor(private externalNamespace: string) {}

	get Add() {
		return concatMethodNames(
			concatNamespace(this.externalNamespace, this.internalNamespace),
			"Add",
		);
	}
	get Update() {
		return concatMethodNames(
			concatNamespace(this.externalNamespace, this.internalNamespace),
			"Update",
		);
	}
	get Delete() {
		return concatMethodNames(
			concatNamespace(this.externalNamespace, this.internalNamespace),
			"Delete",
		);
	}
}

class Appointment {
	private readonly internalNamespace = "Appointment";
	constructor(private externalNamespace: string) {}

	get Add() {
		return concatMethodNames(
			concatNamespace(this.externalNamespace, this.internalNamespace),
			"Add",
		);
	}
	get Update() {
		return concatMethodNames(
			concatNamespace(this.externalNamespace, this.internalNamespace),
			"Update",
		);
	}
	get Delete() {
		return concatMethodNames(
			concatNamespace(this.externalNamespace, this.internalNamespace),
			"Delete",
		);
	}
}

class Prescription {
	private readonly internalNamespace = "Prescription";
	constructor(private externalNamespace: string) {}

	get Add() {
		return concatMethodNames(
			concatNamespace(this.externalNamespace, this.internalNamespace),
			"Add",
		);
	}
	get Update() {
		return concatMethodNames(
			concatNamespace(this.externalNamespace, this.internalNamespace),
			"Update",
		);
	}
	get Delete() {
		return concatMethodNames(
			concatNamespace(this.externalNamespace, this.internalNamespace),
			"Delete",
		);
	}
}

class Organization {
	private readonly internalNamespace = "Organization";
	constructor(private externalNamespace: string) {}

	get Get() {
		return concatMethodNames(
			concatNamespace(this.externalNamespace, this.internalNamespace),
			"Get",
		);
	}

	get GetAll() {
		return concatMethodNames(
			concatNamespace(this.externalNamespace, this.internalNamespace),
			"GetAll",
		);
	}
	get GetLocation() {
		return concatMethodNames(
			concatNamespace(this.externalNamespace, this.internalNamespace),
			"Location",
		);
	}
	get GetAllCountries() {
		return concatMethodNames(
			concatNamespace(this.externalNamespace, this.internalNamespace),
			"Country",
		);
	}
	get GetAllPincode() {
		return concatMethodNames(
			concatNamespace(this.externalNamespace, this.internalNamespace),
			"PinCodes",
		);
	}
	get PinCodes() {
		return concatMethodNames(
			concatNamespace(this.externalNamespace, this.internalNamespace),
			"PinCodes",
		);
	}
	get GetAllState() {
		return concatMethodNames(
			concatNamespace(this.externalNamespace, this.internalNamespace),
			"States",
		);
	}
	get GetAllCity() {
		return concatMethodNames(
			concatNamespace(this.externalNamespace, this.internalNamespace),
			"Cities",
		);
	}

	get Vaccines() {
		return concatMethodNames(
			concatNamespace(this.externalNamespace, this.internalNamespace),
			"Vaccines",
		);
	}
	get Add() {
		return concatMethodNames(
			concatNamespace(this.externalNamespace, this.internalNamespace),
			"Add",
		);
	}
	get Update() {
		return concatMethodNames(
			concatNamespace(this.externalNamespace, this.internalNamespace),
			"Update",
		);
	}
	get Delete() {
		return concatMethodNames(
			concatNamespace(this.externalNamespace, this.internalNamespace),
			"Delete",
		);
	}
	get Select() {
		return concatMethodNames(
			concatNamespace(this.externalNamespace, this.internalNamespace),
			"Select",
		);
	}
	get Sync() {
		return concatMethodNames(
			concatNamespace(this.externalNamespace, this.internalNamespace),
			"Sync",
		);
	}
	get Drugs() {
		return concatMethodNames(
			concatNamespace(this.externalNamespace, this.internalNamespace),
			"Drugs",
		);
	}

	get PhoneCode() {
		return concatMethodNames(
			concatNamespace(this.externalNamespace, this.internalNamespace),
			"PhoneCode",
		);
	}
	get Location() {
		return concatMethodNames(
			concatNamespace(this.externalNamespace, this.internalNamespace),
			"Location",
		);
	}
}

class SystemAdmin {
	private readonly internalNamespace = "SystemAdmin";
	constructor(private externalNamespace: string) {}

	get Add() {
		return concatMethodNames(
			concatNamespace(this.externalNamespace, this.internalNamespace),
			"Add",
		);
	}

	get GetAll() {
		return concatMethodNames(
			concatNamespace(this.externalNamespace, this.internalNamespace),
			"GetAll",
		);
	}
	get Update() {
		return concatMethodNames(
			concatNamespace(this.externalNamespace, this.internalNamespace),
			"Update",
		);
	}
	get Delete() {
		return concatMethodNames(
			concatNamespace(this.externalNamespace, this.internalNamespace),
			"Delete",
		);
	}
}

class Staff {
	private readonly internalNamespace = "Staff";
	constructor(private externalNamespace: string) {}

	get Add() {
		return concatMethodNames(
			concatNamespace(this.externalNamespace, this.internalNamespace),
			"Add",
		);
	}
	get Update() {
		return concatMethodNames(
			concatNamespace(this.externalNamespace, this.internalNamespace),
			"Update",
		);
	}
	get Delete() {
		return concatMethodNames(
			concatNamespace(this.externalNamespace, this.internalNamespace),
			"Delete",
		);
	}
}

class User {
	private readonly internalNamespace = "User";
	constructor(private externalNamespace: string) {}

	get Add() {
		return concatMethodNames(
			concatNamespace(this.externalNamespace, this.internalNamespace),
			"Add",
		);
	}
	get Update() {
		return concatMethodNames(
			concatNamespace(this.externalNamespace, this.internalNamespace),
			"Update",
		);
	}
	get Delete() {
		return concatMethodNames(
			concatNamespace(this.externalNamespace, this.internalNamespace),
			"Delete",
		);
	}
}

export const auth = new Auth("");
export const patients = new Patients("");
export const doctors = new Doctor("");
export const appointment = new Appointment("");
export const prescription = new Prescription("");
export const organizations = new Organization("");
export const systemAdmin = new SystemAdmin("");
export const serviceLocation = new ServiceLocation("");
export const staff = new Staff("");
export const user = new User("");
