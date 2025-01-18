"use client";
import React from "react";
import { SignUp } from "@repo/ui/component";

const page = () => {
	return (
		<SignUp
			backgroundImage="none"
			logoImage="findmydoc logo.svg"
			heading="Create an account"
			continueButtonColour="#FED337"
			linkTextColour="#5FC69B"
			inputHeading="Date of birth"
			inputType="date"
			inputPlaceHolder="DD/MM/YYYY"
			isVisible={true}
		/>
	);
};

export default page;
