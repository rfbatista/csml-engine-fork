import { CsmlController } from '../../adapter/rest/CsmlController';
import { Router, json, urlencoded } from "express";
import errorHandler from './middlewares/errorHandler';

const router = () => {
	const apiRoute = Router();
	const csml = new CsmlController()
	apiRoute
		.use(json())
		.use(urlencoded({ extended: true }))
		.use('/api/v1', csml.router)
		.use(errorHandler);

	return apiRoute
}

export default router