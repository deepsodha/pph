import React from "react";
import { SignIn } from "@repo/ui/component";

const page = () => {
	return (
		<SignIn
			backgroundImage="none"
			logoImage="capsule logo.svg"
			heading="Login"
			continueButtonColour="#095FBA"
			linkTextColour="#095FBA"
		/>
	);
};
export default page;
