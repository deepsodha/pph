import React from "react";
import { SignIn } from "@repo/ui/component";
const page = () => {
	return (
		<SignIn
			backgroundImage="none"
			logoImage="findmydoc logo.svg"
			heading="Login, Enter your email"
			continueButtonColour="#FED337"
			linkTextColour="#5FC69B"
		/>
	);
};

export default page;
