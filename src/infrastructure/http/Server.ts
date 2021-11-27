import { AppConfig } from '../config/AppConfig';
import { Logger } from '../logger/Logger';
import router from './router';
import express from 'express';

export class Server {
	static start() {
	  const config = AppConfig.transport.http;
	  const app = express();
	  
	  app.use(router());
	  app.listen(config.port, () => {
		Logger.info(`Express server running - PORT: ${config.port}`);
	  });
	}
  }