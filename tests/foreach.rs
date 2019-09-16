mod support;

use csmlinterpreter::{interpret};
use csmlinterpreter::interpreter::{json_to_rust::*, message::MessageData};
use csmlinterpreter::parser::Parser;
use serde_json::Value;
use multimap::MultiMap;

use support::tools::{gen_context, message_to_jsonvalue, read_file};

fn format_message(event: Option<Event>) -> MessageData {
    let text = read_file("CSML/foreach.csml".to_owned()).unwrap();
    let flow = Parser::parse_flow(text.as_bytes()).unwrap();

    let memory = gen_context(MultiMap::new(), MultiMap::new(), MultiMap::new(), 0, false);

    interpret(&flow, "start", &memory, &event).unwrap()
}

#[test]
fn ok_foreach() {
    let data = r#"{"messages":[ {"content":{"text":"1"},"content_type":"text"}, {"content":{"text":"2"},"content_type":"text"}, {"content":{"text":"3"},"content_type":"text"} ],"next_flow":null,"memories":[],"next_step":"end"}"#;
    let msg = format_message(None);

    let v1: Value = message_to_jsonvalue(msg);
    let v2: Value = serde_json::from_str(data).unwrap();

    assert_eq!(v1, v2)
}