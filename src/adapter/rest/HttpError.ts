import { BaseError, BaseErrorObject } from '../../infrastructure/error/exception/BaseError';

export interface HttpErrorObject extends BaseErrorObject {
  status: number
}

export type HttpErrorResponse = {
  alias: string,
  message?: string
};

export class HttpError extends BaseError {

  private status: number;
  private responseErrors: HttpErrorResponse[];

  constructor(
    alias?: string,
    context?: any,
    message?: string,
    status?: number,
    responseErrors?: HttpErrorResponse[]
  ) {
    super(alias + ' ' + message, context);

    Object.setPrototypeOf(this, HttpError.prototype);
    this.status = status ?? 400;

    if (!responseErrors) {
      responseErrors = [{ alias: alias, message: message }];
    }

    this.responseErrors = responseErrors;
  }

  getStatus(): number {
    return this.status;
  }

  getResponseErrors(): HttpErrorResponse[] {
    return this.responseErrors;
  }

  toObject(): HttpErrorObject {
    return {
      status: this.status,
      ...super.toObject()
    };
  }
}