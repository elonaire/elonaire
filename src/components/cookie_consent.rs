use chrono::{Duration, Utc};
use gloo::utils::document;
use web_sys::{HtmlDocument, wasm_bindgen::JsCast};
use yew::prelude::*;

#[function_component(CookieConsent)]
pub fn cookie_consent() -> Html {
    let accepted = use_state(|| get_cookie("cookieConsent").unwrap_or_default() == "accepted");

    let accept = {
        let accepted = accepted.clone();
        Callback::from(move |_| {
            set_cookie("cookieConsent", "accepted", 365);
            accepted.set(true);
        })
    };

    let reject = {
        let accepted = accepted.clone();
        Callback::from(move |_| {
            set_cookie("cookieConsent", "rejected", 365);
            accepted.set(true);
        })
    };

    if *accepted {
        html! {}
    } else {
        html! {
            <div id="cookieConsentBanner" class="cookie-banner">
                <p>{ "We use cookies to enhance your experience. By continuing to visit this site you agree to our use of cookies." }</p>
                <button class="button button-primary" onclick={accept}>{ "Accept" }</button>
                <button class="button button-outlined button-warn" onclick={reject}>{ "Reject" }</button>
            </div>
        }
    }
}

fn set_cookie(name: &str, value: &str, days: i32) {
    let document = document().unchecked_into::<HtmlDocument>();
    let expiration_date = Utc::now() + Duration::days(days.into());
    let expires = expiration_date.format("%a, %d %b %Y %H:%M:%S GMT").to_string();

    let cookie = format!("{}={}; expires={}; path=/", name, value, expires);
    document.set_cookie(&cookie).unwrap();
}

fn get_cookie(name: &str) -> Option<String> {
    let document = document().unchecked_into::<HtmlDocument>();
    let cookies = document.cookie().unwrap();
    for cookie in cookies.split(';') {
        let parts: Vec<&str> = cookie.split('=').collect();
        if parts.len() == 2 && parts[0].trim() == name {
            return Some(parts[1].trim().to_string());
        }
    }
    None
}
