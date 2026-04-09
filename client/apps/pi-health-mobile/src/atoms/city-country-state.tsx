import { atom, useAtom } from "jotai";

const countryAtom = atom("");
const stateAtom = atom("");
const cityAtom = atom("");
const pincodeAtom = atom("");

export function CountryData() {
	const [country, setCountry] = useAtom(countryAtom);
	const [state, setState] = useAtom(stateAtom);
	const [city, setCity] = useAtom(cityAtom);
	const [pincode, setPincode] = useAtom(pincodeAtom);

	const updateCountry = (country: string) => {
		setCountry(country);
	};

	const updateState = (state: string) => {
		setState(state);
	};

	const updateCity = (city: string) => {
		setCity(city);
	};
	const updatePincode = (pincode: string) => {
		setPincode(pincode);
	};

	return {
		country,
		state,
		city,
		pincode,
		updateCountry,
		updateState,
		updateCity,
		updatePincode,
	};
}
