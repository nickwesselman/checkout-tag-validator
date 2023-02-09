import {
  InputQuery,
  FunctionResult,
  ProductVariant
} from "../generated/api";

const EMPTY_RESULT: FunctionResult = {
  errors: []
};

type Configuration = {};

export default (input: InputQuery): FunctionResult => {
  const isMember = input.cart.buyerIdentity?.customer?.isMember ?? false;

  const hasMemberProducts = input.cart.lines
    .findIndex(line => line.merchandise.__typename == "ProductVariant"
      && line.merchandise.product.isMembersProduct) != -1;

  if (hasMemberProducts && !isMember) {
    return {
      errors: [
        {
          localizedMessage: "Cannot purchase member products as non member",
          target: "cart"
        }
      ]
    }
  }

  return EMPTY_RESULT;
};