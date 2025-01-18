import { atom, useAtom } from "jotai";

const countryAtom = atom("");
const stateAtom = atom("");
const cityAtom = atom("");
const pincodeAtom = atom("");
export function SystemAdminCountryData() {
	const [country1, setCountry] = useAtom(countryAtom);
	const [state1, setState] = useAtom(stateAtom);
	const [city1, setCity] = useAtom(cityAtom);
	const [pincode1, setPincode] = useAtom(pincodeAtom);
	const updateCountryAdmin = (country: string) => {
		setCountry(country);
	};

	const updateStateAdmin = (state: string) => {
		setState(state);
	};

	const updateCityAdmin = (city: string) => {
		setCity(city);
	};

	const updatePincodeAdmin = (pincode: string) => {
		setPincode(pincode);
	};

	return {
		country1,
		state1,
		city1,
		pincode1,
		updatePincodeAdmin,
		updateCountryAdmin,
		updateStateAdmin,
		updateCityAdmin,
	};
}
