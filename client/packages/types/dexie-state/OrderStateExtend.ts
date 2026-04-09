import { parseISO } from "date-fns";
import type { OrderState } from "../dto";

export type OrderStateExtend = Omit<OrderState, "last_updated"> & {
	last_updated: Date;
};

export const mapOrder = (OrderStateExtend: OrderState): OrderStateExtend => {
	return {
		...OrderStateExtend,
		last_updated: parseISO(OrderStateExtend.last_updated),
	};
};
