import { SignIn } from "@repo/ui/component";

const page: React.FC = () => {
	return (
		<div>
			<SignIn
				backgroundImage="Login bg.svg"
				logoImage="Logo - 2 3.svg"
				heading="Login"
				continueButtonColour="#095FBA"
				linkTextColour="#095FBA"
			/>
		</div>
	);
};

export default page;
