use leptos::{html::Form, prelude::*};
use serde::de::DeserializeOwned;
use web_sys::{CustomEvent, Event, EventTarget};

use js_sys::Array;
use serde_json::{Map, Value};
use web_sys::FormData;

pub fn get_form_data_from_form_ref(form_ref: &NodeRef<Form>) -> Option<FormData> {
    let form = form_ref.to_owned().get_untracked()?;
    let form_data = FormData::new_with_form(&form).ok()?;
    Some(form_data)
}

pub fn deserialize_form_data_to_struct<T: DeserializeOwned>(
    form_data: &FormData,
    deserialize_bool: bool,
) -> Option<T> {
    let entries = js_sys::try_iter(form_data).ok()?.unwrap();
    let mut map = Map::new();

    for entry in entries {
        let pair = entry.ok()?;
        let arr = Array::from(&pair);
        let key = arr.get(0).as_string()?;
        let value = arr.get(1);
        if let Some(s) = value.as_string() {
            if deserialize_bool && (s == "true" || s == "false") {
                let parsed_s: bool = s.parse().unwrap();

                map.insert(key, Value::Bool(parsed_s));
            } else {
                map.insert(key, Value::String(s));
            }
        }
    }

    let value = Value::Object(map);
    serde_json::from_value(value).ok()
}

pub fn fire_bubbled_and_cancelable_event<T>(
    event_type: &str,
    bubbles: bool,
    cancelable: bool,
    element: T,
) -> ()
where
    T: AsRef<EventTarget>,
{
    let _event = match Event::new(event_type) {
        Ok(ev) => {
            ev.init_event_with_bubbles_and_cancelable(event_type, bubbles, cancelable);
            element.as_ref().dispatch_event(&ev).unwrap();
        }
        Err(_e) => {}
    };
}

pub fn fire_custom_bubbled_and_cancelable_event<T>(
    event_type: &str,
    bubbles: bool,
    cancelable: bool,
    element: T,
) -> ()
where
    T: AsRef<EventTarget>,
{
    let _event = match CustomEvent::new(event_type) {
        Ok(ev) => {
            ev.init_event_with_bubbles_and_cancelable(event_type, bubbles, cancelable);
            element.as_ref().dispatch_event(&ev).unwrap();
        }
        Err(_e) => {}
    };
}
