import React from "react";
import { SignUp } from "@repo/ui/component";
const page = () => {
	return (
		<SignUp
			backgroundImage="none"
			logoImage="capsule logo.svg"
			heading="Create an account"
			continueButtonColour="#095FBA"
			linkTextColour="#095FBA"
			inputHeading="Phone Number"
			inputType="text"
			inputPlaceHolder="none"
			isVisible={false}
		/>
	);
};
export default page;
