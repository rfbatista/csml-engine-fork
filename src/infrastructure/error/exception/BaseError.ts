export interface BaseErrorObject {
  name: string,
  message: string,
  context: any,
  stack: string
}

export class BaseError extends Error {

  private context: any;

  constructor(message?: string, context?: any) {
    super(message);

    this.name = this.constructor.name;
    this.context = context;

    Object.setPrototypeOf(this, BaseError.prototype);
    Error.captureStackTrace(this, this.constructor);
  }

  getName(): string {
    return this.name;
  }

  getContext(): any {
    return this.context;
  }

  toObject(): BaseErrorObject {
    return {
      name: this.name,
      message: this.message,
      context: this.context,
      stack: this.stack
    };
  }

}