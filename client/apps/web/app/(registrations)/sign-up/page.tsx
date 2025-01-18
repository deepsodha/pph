import { SignUp } from "@repo/ui/component";
const page: React.FC = () => {
	return (
		<SignUp
			backgroundImage="Background Image.svg"
			logoImage="Pi-health logo (2).svg"
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
