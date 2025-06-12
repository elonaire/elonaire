use web_sys::{Event, EventTarget};

use js_sys::Array;
use serde_json::{Map, Value};
use web_sys::FormData;

pub fn form_data_to_json_string(form_data: &FormData) -> Option<String> {
    let entries = js_sys::try_iter(form_data).ok()?.unwrap();
    let mut map = Map::new();

    for entry in entries {
        let pair = entry.ok()?;
        let arr = Array::from(&pair);
        let key = arr.get(0).as_string()?;
        let value = arr.get(1);
        if let Some(s) = value.as_string() {
            map.insert(key, Value::String(s));
        }
    }

    serde_json::to_string(&Value::Object(map)).ok()
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
