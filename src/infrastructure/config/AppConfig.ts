import * as dotenv from 'dotenv';

dotenv.config();

export const AppConfig = {
	transport: {
		http: {
			port: process.env.SERVER_PORT 
		}
	},
	logger: {
		level: process.env.LOG_LEVEL ?? 'debug',
		hostname: {
		  prefix: process.env.LOG_HOSTNAME_PREFIX ?? 'csml-engine',
		},
	  },
}