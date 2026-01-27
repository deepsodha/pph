/// <reference types="vite/client" />
export function baseUrl() {
	if (import.meta.env?.PROD) {
		//TODO: there should be better solution here
		return window.location.origin;
	}
	return "http://localhost:5000";
}
