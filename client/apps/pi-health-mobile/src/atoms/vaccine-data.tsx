import { atom, useAtom } from "jotai";

const vaccineData = atom<string[] | undefined>([]);
export const VaccineData = () => {
	const [vaccine, setVaccineData] = useAtom(vaccineData);
	function UpdateVaccineData(vaccine: string[] | undefined) {
		setVaccineData(vaccine);
	}
	return {
		vaccine,
		UpdateVaccineData,
	};
};
