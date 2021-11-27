import { HttpError, HttpErrorResponse } from '../../adapter/rest/HttpError';
import {Logger} from "../logger/Logger";
import {errorAlias} from "./errorAlias";
import {BaseError} from "./exception/BaseError";

export class ErrorHandler {

  private static createContext(error) {
    if (error instanceof BaseError) {
      return error.toObject();
    }

    return {
      errorCode: error.code,
      errorName: error.name,
      errorStatus: error.status,
      errorFileName: error.fileName,
      errorLineNumber: error.lineNumber,
      stack: error.stack
    };
  }

  static http(error, req, res, next) {
    Logger.error(
      errorAlias.UNHANDLED_HTTP_REJECTION + (error['message'] ?? ''),
      ErrorHandler.createContext(error)
    );

    if (!res?.headersSent) {
      let status = 500;
      let errors: HttpErrorResponse[] = [
        {alias: errorAlias.UNKNOWN, message: 'Something went wrong'}
      ];

      if (error instanceof HttpError) {
        status = error.getStatus();
        errors = error.getResponseErrors() ?? errors;
      }

      res.status(status).json({ errors: errors});
    }
  }

  static global() {
    process.on("unhandledRejection", (error) => {
      Logger.error(
        errorAlias.UNHANDLED_REJECTION + (error['message'] ?? ''),
        ErrorHandler.createContext(error)
      );
    });
  }

}