import pino from 'pino'
import { AppConfig } from '../config/AppConfig';
import {BaseError} from "../error/exception/BaseError";
import os from "os";

export enum LoggerLevel {
  fatal = "fatal",
  error = "error",
  warn = "warn",
  info = "info",
  debug = "debug",
  trace = "trace",
  silent = "silent"
}

export class Logger {

  public static fixed: any;
  private static logger: pino.Logger;

  private static getClient(): pino.Logger {
    if (!Logger.logger) {
      let hostname = os.hostname();
      if (AppConfig.logger?.hostname?.prefix) {
        hostname = AppConfig.logger.hostname.prefix + '--' + hostname;
      }

      Logger.logger = pino({
        level: LoggerLevel.info,
        base: {
          pid: process.pid,
          hostname: hostname
        }
      });
    }
    return Logger.logger;
  }

  private static make(): pino.Logger {
    return Logger.fixed
      ? Logger.getClient().child(Logger.fixed)
      : Logger.getClient();
  }

  static setFixed(fixed: any): void {
    Logger.fixed = fixed;
  }

  static setLevel(level: LoggerLevel): void {
    Logger.getClient().level = level;
  }

  static trace(message: string, context: any = null): void {
    context = Logger.parseContext(context);
    Logger.make().trace(context, message);
  }

  static debug(message: string, context: any = null): void {
    context = Logger.parseContext(context);
    Logger.make().debug(context, message);
  }

  static info(message: string, context: any = null): void {
    context = Logger.parseContext(context);
    Logger.make().info(context, message);
  }

  static warn(message: string, context: any = null): void {
    context = Logger.parseContext(context);
    Logger.make().warn(context, message);
  }

  static error(message: string, context: any = null): void {
    context = Logger.parseContext(context);
    Logger.make().error(context, message);
  }

  static fatal(message: string, context: any = null): void {
    context = Logger.parseContext(context);
    Logger.make().fatal(context, message)
  }

  private static parseContext(context: any) {
    if(context instanceof BaseError) {
      return { context: context.toObject() };
    }

    if (context instanceof Error) {
      return {
        errorCode: context['code'],
        errorName: context['name'],
        errorStatus: context['status'],
        errorFileName: context['fileName'],
        errorLineNumber: context['lineNumber'],
        stack: context['stack']
      }
    }

    return {context: context};
  }
}