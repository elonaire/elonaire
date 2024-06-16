use yew::prelude::*;
use web_sys::{HtmlElement, window};

#[function_component(AdComponent)]
pub fn ad_component() -> Html {
    let ad_ref = use_node_ref();

    use_effect_with_deps({
        let ad_ref = ad_ref.clone();
        let google_ad_client = option_env!("GOOGLE_AD_CLIENT").expect("GOOGLE_AD_CLIENT env var not set");
        
        move |_| {
            if let Some(ad_div) = ad_ref.cast::<HtmlElement>() {
                let window = window().expect("no global `window` exists");
                let document = window.document().expect("should have a document on window");
                let script = document.create_element("script").expect("could not create script");
    
                let script_content = format!(r#"
                    (adsbygoogle = window.adsbygoogle || []).push({{
                        google_ad_client: "{}",
                        enable_page_level_ads: true
                    }});
                "#, google_ad_client);
    
                script.set_inner_html(&script_content);
    
                ad_div.append_child(&script).expect("could not append script");
            }
            || ()
        }
    }, ());

    html! {
        <div ref={ad_ref} class="ad-container">
            // Placeholder for ad
        </div>
    }
}
