import { SignIn } from "@repo/ui/component";

const page: React.FC = () => {
	return (
		<SignIn
			backgroundImage="Background Image.svg"
			logoImage="Pi-health logo (2).svg"
			heading="Login"
			continueButtonColour="#095FBA"
			linkTextColour="#095FBA"
		/>
	);
};

export default page;
