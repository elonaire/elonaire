use leptos::prelude::*;

#[component]
pub fn AuthorInfo(
    #[prop(into)] profile_pic: String,
    #[prop(into)] author_name: String,
) -> impl IntoView {
    let full_name = author_name.clone();

    let short_name = {
        let parts: Vec<&str> = author_name.split_whitespace().collect();
        match parts.as_slice() {
            [first, .., last] => format!("{} {}.", first, last.chars().next().unwrap_or_default()),
            [first] => first.to_string(),
            _ => "".to_string(),
        }
    };

    view! {
        <div class="flex items-center gap-[10px]">
            <img src=profile_pic alt="Author Avatar" class="rounded-full size-6" />
            // Mobile
            <p class="text-xs md:hidden">{short_name}</p>

            // md and above
            <p class="text-xs hidden md:block">{full_name}</p>
        </div>
    }
}
