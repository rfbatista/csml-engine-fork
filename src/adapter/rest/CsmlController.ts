import { NextFunction, Request, Response, Router } from 'express';

const csml = require('../../../bindings/node/native');

export class CsmlController {
  get router(): Router {
    const router = Router();

    /**
     * Default route
     */
    router.get('/health', this.health);

    router.post('/run', this.run);

    router.post('/get_bot_steps', this.getBotSteps);

    return router;
  }

  health = (req: Request, res: Response, next: NextFunction) => {
    res.send(`Running CSML Server`);
  };

  run = (req: Request, res: Response, next: NextFunction) => {
    const { bot, event } = req.body;
    if (
      event.payload &&
      event.payload.content &&
      event.payload.content.close_flows === true
    ) {
      csml.closeAllConversations(event.client);
    }
    const data = csml.run(event, bot);
    return res.json(data);
  };

  getBotSteps = (req, res) => {
    const data = csml.getBotSteps(req.body);
    return res.json(data);
  };
}
