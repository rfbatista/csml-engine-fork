use crate::error_format::*;
use crate::data::{
    Literal, primitive::PrimitiveType,
    ast::Interval,
    primitive::{
        Data,
    },
    error_info::ErrorInfo,
    position::Position
};
use std::{collections::HashMap};

use lettre::message::Mailbox;
use lettre::transport::smtp::authentication::Credentials;

////////////////////////////////////////////////////////////////////////////////
// PRIVATE FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

fn format_email_value<'a>(
    email: &'a HashMap<String, Literal>,
    value: &'a str,
    additional_info: &'a str,
    data: &'a Data,
    interval: Interval,
) -> Result<Option<&'a String>, ErrorInfo> {
    let error_message = format!("email [{}] value need to be of type String {}", value, additional_info);

    match email.get(value) {
        Some(lit) => {
            if lit.primitive.get_type() != PrimitiveType::PrimitiveString {
                return Err(gen_error_info(
                    Position::new(interval, &data.context.flow,),
                    error_message,
                ))
            }

            let value = Literal::get_value::<String>(
                &lit.primitive,
                &data.context.flow,
                lit.interval,
                error_message,
            )?;

            Ok(Some(value))
        }
        None => Ok(None)
    }
}

fn parse_email(email_str: &str, data: &Data, interval: Interval) -> Result<Mailbox, ErrorInfo>{
    match email_str.parse::<Mailbox>() {
        Ok(mbox) => Ok(mbox),
        Err(_) => Err(gen_error_info(
            Position::new(interval, &data.context.flow,),
            "Invalid email format".to_owned(),
        ))
    }
}

fn get_value<'a, T>(
    value: Option<&'a Literal>,
    data: &Data,
    error_message: String,
    interval: Interval
) -> Result<&'a T, ErrorInfo>
where
    T: 'static,
{
    match value {
        Some(lit) => Literal::get_value::<T>(
            &lit.primitive,
            &data.context.flow,
            lit.interval,
            error_message
        ),
        None => Err(gen_error_info(
            Position::new(interval, &data.context.flow,),
            error_message,
        ))
    }
}

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

pub fn format_email(
    email: &HashMap<String, Literal>,
    data: &Data,
    interval: Interval,
) -> Result<lettre::Message, ErrorInfo> {
    let mut message_builder = lettre::Message::builder();

    let from = format_email_value(&email, "from", "and a valid email", data, interval)?;
    if let Some(form) = from {
        let mbox = parse_email(form.as_ref(), data, interval)?;
        message_builder = message_builder.from(mbox);
    }

    let sender = format_email_value(&email, "sender", "and a valid email", data, interval)?;
    if let Some(sender) = sender {
        let mbox = parse_email(sender.as_ref(), data, interval)?;
        message_builder = message_builder.sender(mbox);
    }

    let reply_to = format_email_value(&email, "reply_to", "and a valid email", data, interval)?;
    if let Some(reply_to) = reply_to {
        let mbox = parse_email(reply_to.as_ref(), data, interval)?;
        message_builder = message_builder.reply_to(mbox);
    }

    let bcc = format_email_value(&email, "bcc", "and a valid email", data, interval)?;
    if let Some(bcc) = bcc {
        let mbox = parse_email(bcc.as_ref(), data, interval)?;
        message_builder = message_builder.bcc(mbox);
    }

    let cc = format_email_value(&email, "cc", "and a valid email", data, interval)?;
    if let Some(cc) = cc {
        let mbox = parse_email(cc.as_ref(), data, interval)?;
        message_builder = message_builder.cc(mbox);
    }

    let to = format_email_value(&email, "to", "and a valid email", data, interval)?;
    if let Some(to) = to {
        let mbox = parse_email(to.as_ref(), data, interval)?;
        message_builder = message_builder.to(mbox);
    }

    let subject = format_email_value(&email, "subject", "", data, interval)?;
    if let Some (subject) = subject {
        message_builder = message_builder.subject(subject.to_owned());
    }

    let body = format_email_value(&email, "body", "", data, interval)?;

    match body {
        Some(body) => match message_builder.body(body.to_owned()) {
            Ok(message) => Ok(message),
            Err(_) => Err(gen_error_info(
                Position::new(interval, &data.context.flow,),
                "missing mandatory email parameter [from] or [to]".to_owned(),
            ))
        },
        None => Err(gen_error_info(
            Position::new(interval, &data.context.flow,),
            "email body parameter is mandatory".to_owned(),
        ))
    }
}

pub fn get_mailer(
    object: &mut HashMap<String, Literal>,
    data: &Data,
    interval: Interval,
) -> Result<lettre::SmtpTransport, ErrorInfo> {

    let username = get_value::<String>(
        object.get("username"), 
        data, 
        "username is missing or invalid type".to_owned(), 
        interval
    )?;
    let password = get_value::<String>(
        object.get("password"), 
        data, 
        "password is missing or invalid type".to_owned(), 
        interval
    )?;
    // set default port to [465] for TLS connections. RFC8314](https://tools.ietf.org/html/rfc8314)
    let port = match get_value::<u16>(
        object.get("port"), 
        data, 
        "".to_owned(), 
        interval
    ) {
        Ok(port_value) => port_value.to_owned(),
        Err(_) => 465
    };
    let smtp_server = get_value::<String>(
        object.get("smtp_server"), 
        data, 
        "SMTP server address is missing or invalid type".to_owned(), 
        interval
    )?;

    let credentials = Credentials::new(username.to_string(), password.to_string());

    match lettre::SmtpTransport::relay(smtp_server){
        Ok(smtp_server) => {
            let mailer =  smtp_server
            .credentials(credentials)
            .port(port)
            .build();

            Ok(mailer)
        }
        Err(_) => Err(gen_error_info(
            Position::new(interval, &data.context.flow,),
            "invalid SMTP address".to_owned(),
        ))
    }
}