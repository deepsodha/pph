import { Box, Button, Select, Center } from "@chakra-ui/react";
import type { z } from "zod";
import { useForm } from "@felte/react";
import { validator } from "@felte/validator-zod";
import reporterDom from "@felte/reporter-dom";

import type { SelectServiceLocation } from "@repo/types/dto";
import { SelectServiceLocationSchema } from "@repo/types/validation";
import type { ServiceLocationStateExtend } from "@repo/types/dexie-state";

export type ServiceLocationProps = {
	onSubmit: (o: SelectServiceLocation) => void;
	servicelocation: ServiceLocationStateExtend[] | undefined;
};
export const HospitalSelect = (props: ServiceLocationProps) => {
	const { form } = useForm<z.infer<typeof SelectServiceLocationSchema>>({
		onSubmit: async (values) => {
			const combinedValue: string = values.name;

			const [id = "", name = ""] = combinedValue.split(":");
			props.onSubmit({ ...values, id, name });
		},
		initialValues: { name: "", id: "" },
		extend: [validator({ schema: SelectServiceLocationSchema }), reporterDom()],
	});

	return (
		<>
			<form ref={form}>
				<Box
					border="1px"
					borderColor="#095FBA"
					rounded="md"
					marginTop="10px"
					mb="8%"
					width="100%"
				>
					<Select
						placeholder="Select Hospital / Clinic"
						bgColor={"#FFFFFF"}
						borderColor={"#095FBA"}
						name="name"
					>
						{props.servicelocation && props.servicelocation
							? props.servicelocation?.map((items) => (
									<option
										key={items.service_location_name}
										value={`${items.id}:${items.service_location_name}`}
									>
										{items.service_location_name}
									</option>
								))
							: null}
					</Select>
				</Box>
				<Center>
					<Button
						size="md"
						type="submit"
						height="48px"
						width="375px"
						color={"#FFFFFF"}
						bgColor={"#095FBA"}
						border="2px"
						mt={8}
					>
						Continue
					</Button>
				</Center>
			</form>
		</>
	);
};
