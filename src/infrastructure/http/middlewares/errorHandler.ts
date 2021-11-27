import { ErrorRequestHandler, Request, Response, NextFunction } from "express";
import { Logger } from '../../logger/Logger';

const errorHandler: ErrorRequestHandler = (
	error: any,
	req: Request,
	res: Response,
	next: NextFunction
  ): any => {
	Logger.error(error.message);
  
	const response: any = {
	  message: error.message,
	  status: error.status || 500,
	};
  
	res.status(response.status).send(response);
  
	next();
  };
  
  export default errorHandler;