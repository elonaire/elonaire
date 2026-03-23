use chrono::Utc;
use icondata as IconId;
use js_sys::wasm_bindgen::prelude::Closure;
use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;
use web_sys::MouseEvent;

use crate::{
    components::general::button::BasicButton, data::models::graphql::shared::ReactionType,
    utils::time::get_elapsed_time,
};

#[derive(Clone, Debug)]
pub struct CommentReactionDetails {
    pub comment_id: String,
    pub reaction_type: ReactionType,
}

#[component]
pub fn BlogComment(
    #[prop(into)] author_avatar: String,
    #[prop(into)] author_name: String,
    #[prop(into)] date_of_creation: String,
    #[prop(into)] content: String,
    #[prop(into)] comment_id: String,
    #[prop(into)] reply_count: u32,
    #[prop(into)] reaction_count: Signal<u32>,
    #[prop(into)] current_user_reaction: Option<ReactionType>,
    #[prop(optional, default = Callback::new(|_| {}))] on_reaction: Callback<
        CommentReactionDetails,
    >,
) -> impl IntoView {
    let (datetime, _set_date_time) = signal(Utc::now());
    let (show_reactions, set_show_reactions) = signal(false);
    let hover_timer: StoredValue<Option<i32>> = StoredValue::new(None);

    let react_to_comment = move |reaction_details: CommentReactionDetails| {
        on_reaction.run(reaction_details);
    };

    let reactions = vec![
        (ReactionType::Like, "👍"),
        (ReactionType::Dislike, "👎"),
        (ReactionType::Love, "❤️"),
        (ReactionType::Haha, "😂"),
        (ReactionType::Wow, "😮"),
        (ReactionType::Sad, "😢"),
        (ReactionType::Angry, "😡"),
    ];

    let selected_reaction_icon = match current_user_reaction {
        Some(ReactionType::Like) => IconId::LuThumbsUp,
        Some(ReactionType::Dislike) => IconId::LuThumbsDown,
        Some(ReactionType::Love) => IconId::AiHeartFilled,
        Some(ReactionType::Haha) => IconId::FaFaceGrinTearsRegular,
        Some(ReactionType::Wow) => IconId::FaFaceSurpriseRegular,
        Some(ReactionType::Sad) => IconId::FaFaceSadTearRegular,
        Some(ReactionType::Angry) => IconId::FaFaceAngryRegular,
        _ => IconId::LuThumbsUp,
    };

    // Clear timer helper
    let clear_timer = move || {
        if let Some(id) = hover_timer.get_value() {
            window().clear_timeout_with_handle(id);
            hover_timer.set_value(None);
        }
    };

    let on_mouse_enter = move |_ev: MouseEvent| {
        let id = window()
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                &Closure::<dyn Fn()>::new(move || {
                    set_show_reactions.set(true);
                })
                .into_js_value()
                .unchecked_ref(),
                1000,
            )
            .unwrap_or_default();
        hover_timer.set_value(Some(id));
    };

    let on_mouse_leave = move |_ev: MouseEvent| {
        clear_timer();
        set_show_reactions.set(false);
    };

    let on_touch_start = move |_| {
        let id = window()
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                &Closure::<dyn Fn()>::new(move || {
                    set_show_reactions.set(true);
                })
                .into_js_value()
                .unchecked_ref(),
                1000,
            )
            .unwrap_or_default();
        hover_timer.set_value(Some(id));
    };

    let on_touch_end = move |_| {
        // Only clear timer, don't hide reactions — let user pick
        clear_timer();
    };

    view! {
        <div class="flex gap-[13px]">
            <img src=author_avatar alt="Avatar" class="size-8 rounded-full" />
            <div class="flex flex-col gap-[10px]">
                <div class="flex items-center gap-[8px] text-xs">
                    <p class="font-bold">{author_name}</p>
                    <div class="size-[5px] rounded-full bg-mid-gray" />
                    <p>{move || format!("{} ago", get_elapsed_time(&date_of_creation, &datetime.get()))}</p>
                </div>
                <div inner_html={content} />
                // Reactions
                <div class="flex gap-[25px]">
                    // <BasicButton icon=Some(IconId::BiHeartRegular) button_text="25" icon_before=true on:click=move |_| react_to_comment(CommentReactionDetails {
                    //     comment_id: comment_id.clone(),
                    //     reaction_type: ReactionType::Like
                    // }) children_style_ext="text-xs" />
                    <div
                        class="relative"
                        on:mouseenter=on_mouse_enter
                        on:mouseleave=on_mouse_leave
                        on:touchstart=on_touch_start
                        on:touchend=on_touch_end
                    >
                        {/* Reactions popup */}
                        <div class=move || format!(
                            "absolute bottom-full flex items-center gap-[20px] bg-contrast-white border border-light-gray rounded-full px-3 py-2 shadow-lg transition-all duration-200 {}",
                            if show_reactions.get() { "opacity-100 translate-y-0 pointer-events-auto" }
                            else { "opacity-0 translate-y-2 pointer-events-none" }
                        )>
                            {reactions.iter().map(|(reaction_type, emoji)| {
                                let reaction_type = *reaction_type;
                                let is_selected = current_user_reaction == Some(reaction_type);
                                let comment_id_clone = comment_id.clone();
                                view! {
                                    <button
                                        class=move || format!(
                                            "text-xl transition-transform duration-150 cursor-pointer hover:scale-125 flex flex-col items-center gap-1 {}",
                                            if is_selected { "scale-125" } else { "" }
                                        )
                                        on:click=move |_| {
                                            // set_selected_reaction.set(Some(reaction_type));
                                            let comment_id_clone = comment_id_clone.clone();
                                            set_show_reactions.set(false);
                                            react_to_comment(CommentReactionDetails {
                                                comment_id: comment_id_clone,
                                                reaction_type
                                            });
                                        }
                                    >
                                        {*emoji}
                                    </button>
                                }
                            }).collect::<Vec<_>>()}
                        </div>

                        {/* Like button — updates icon/text based on selected reaction */}
                        <div on:click=move |_| {
                            react_to_comment(CommentReactionDetails {
                                comment_id: comment_id.clone(),
                                reaction_type: ReactionType::Like
                            })
                        }>
                            {
                                move || {
                                    let reaction_count = reaction_count.get();
                                    view! {
                                        <BasicButton
                                            button_text=reaction_count.to_string()
                                            icon=Some(selected_reaction_icon)
                                            icon_before=true
                                            style_ext=format!("{}", if current_user_reaction.is_some() { "text-primary" } else { "" })
                                            children_style_ext="text-xs"
                                        />
                                    }
                                }
                            }
                        </div>
                    </div>
                    <BasicButton icon=Some(IconId::IoStatsChart) button_text="0" icon_before=true children_style_ext="text-xs" />
                    <BasicButton icon=Some(IconId::FaCommentRegular) button_text=format!("{} {}", reply_count, if reply_count == 1 { "reply" } else { "replies" }) icon_before=true children_style_ext="text-xs" />
                </div>
            </div>
        </div>
    }
}
