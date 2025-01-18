import { atom, useAtom } from "jotai";

const tokenAtom = atom(localStorage.getItem("token") || "");

export function useAuthStore() {
	const [token, setToken] = useAtom(tokenAtom);

	const updateToken = (newToken: string) => {
		localStorage.setItem("token", newToken);
		setToken(newToken);
	};

	const reset = () => {
		localStorage.removeItem("token");
		setToken("");
	};

	return {
		token,
		updateToken,
		reset,
	};
}
