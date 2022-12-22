import { customErrorFactory } from 'ts-custom-error';

export const ValidationError = customErrorFactory(function ValidationError(
  message = 'Invalid parameter'
) {
  this.message = message;
},
TypeError);
