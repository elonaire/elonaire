use std::str::FromStr;

use chrono::{DateTime, Utc};
use icondata as IconId;
use leptos::prelude::*;

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
    #[prop(optional, default = Callback::new(|_| {}))] on_reaction: Callback<
        CommentReactionDetails,
    >,
) -> impl IntoView {
    let (datetime, _set_date_time) = signal(Utc::now());

    let react_to_comment = move |reaction_details: CommentReactionDetails| {
        on_reaction.run(reaction_details);
    };

    view! {
        <div class="flex gap-[13px]">
            <img src=author_avatar alt="Avatar" class="w-[63px] h-[63px] rounded-full" />
            <div class="flex flex-col gap-[10px]">
                <div class="flex items-center gap-[8px] text-xs">
                    <p class="font-bold">{author_name}</p>
                    <div class="size-[5px] rounded-full bg-mid-gray" />
                    <p>{move || format!("{} ago", get_elapsed_time(&date_of_creation, &datetime.get()))}</p>
                </div>
                <div inner_html={content} />
                // Reactions
                <div class="flex gap-[25px]">
                    <BasicButton icon=Some(IconId::BiHeartRegular) button_text="25" icon_before=true on:click=move |_| react_to_comment(CommentReactionDetails {
                        comment_id: comment_id.clone(),
                        reaction_type: ReactionType::Like
                    }) children_style_ext="text-xs" />
                    <BasicButton icon=Some(IconId::IoStatsChart) button_text="5" icon_before=true children_style_ext="text-xs" />
                    <BasicButton icon=Some(IconId::FaCommentRegular) button_text=format!("{} {}", reply_count, if reply_count == 1 { "reply" } else { "replies" }) icon_before=true children_style_ext="text-xs" />
                </div>
            </div>
        </div>
    }
}
