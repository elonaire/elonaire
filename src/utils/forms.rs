use web_sys::{Event, EventTarget};

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
