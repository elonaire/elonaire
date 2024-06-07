use yew::prelude::*;

use crate::{app::AppStateContext, components::{
    back_home::BackHome, page_header::{PageHeader, PageHeaderProps}, skill::Skills, styled_heading::StyledHeading, timeline::Timeline, transition::Transition
}, data::{context::user_resources::get_user_resources, models::resource::{UserResume, UserResumeSection, UserSkill, UserSkillType}}};

#[function_component(Resume)]
pub fn resume() -> Html {
    let current_state = use_context::<AppStateContext>().unwrap();
    let resoures_state_clone = current_state.clone();
    let state_clone = current_state.clone();
    let state_clone_for_deps = current_state.clone();

    let education_items = use_state_eq(|| match current_state.user_resources.resume.clone() {
        Some(items) => items.into_iter().filter(|item| item.section.unwrap() == UserResumeSection::Education).collect::<Vec<UserResume>>(),
        None => vec![],
    });
    let experience_items = use_state_eq(|| match current_state.user_resources.resume.clone() {
        Some(items) => items.into_iter().filter(|item| item.section.unwrap() == UserResumeSection::Experience).collect::<Vec<UserResume>>(),
        None => vec![],
    });

    
    let technical_skills = use_state_eq(|| match current_state.user_resources.skills.clone() {
        Some(skills) => skills.into_iter().filter(|skill| skill.r#type.unwrap() == UserSkillType::Technical).collect::<Vec<UserSkill>>(),
        None => vec![],
    });
    let soft_skills = use_state_eq(|| match current_state.user_resources.skills.clone() {
        Some(skills) => skills.into_iter().filter(|skill| skill.r#type.unwrap() == UserSkillType::Soft).collect::<Vec<UserSkill>>(),
        None => vec![],
    });

    let page_header_props = use_state_eq(|| PageHeaderProps {
        heading: "Resume".to_owned(),
        hint: "I'm available for hire".to_owned(),
    });

    use_effect(move || {
        wasm_bindgen_futures::spawn_local(async move {
            // log::info!("Resume component: {:?}", current_state.user_resources.resume.clone());
            if current_state.user_resources.resume.is_none() {
                let _user_resources = get_user_resources("pni9fr7u9gf2bzkf6dmf".to_string(), resoures_state_clone).await;
            }
        }); // Await the async block
        || () 
    });

    let education_items_clone = education_items.clone();
    let experience_items_clone = experience_items.clone();
    let technical_skills_clone = technical_skills.clone();
    let soft_skills_clone = soft_skills.clone();
    use_effect_with_deps(
        move |_| {
            match state_clone.user_resources.resume.clone() {
                Some(resume) => {
                    education_items_clone.set(resume.clone().into_iter().filter(|item| item.section.unwrap() == UserResumeSection::Education).collect::<Vec<UserResume>>());
                        experience_items_clone.set(resume.clone().into_iter().filter(|item| item.section.unwrap() == UserResumeSection::Experience).collect::<Vec<UserResume>>());
                }
                None => {}
            }

            match state_clone.user_resources.skills.clone() {
                Some(skills) => {
                    technical_skills_clone.set(skills.clone().into_iter().filter(|skill| skill.r#type.unwrap() == UserSkillType::Technical).collect::<Vec<UserSkill>>());
                        soft_skills_clone.set(skills.clone().into_iter().filter(|skill| skill.r#type.unwrap() == UserSkillType::Soft).collect::<Vec<UserSkill>>());
                }
                None => {}
            }
        },
        state_clone_for_deps.user_resources.clone(),
    );

    html! {
        <>
            <Transition />
            <main class="resume">
                <BackHome />
                <PageHeader hint={page_header_props.hint.clone()} heading={page_header_props.heading.clone()} />
                <div class="wrapper">
                    <div class="education">
                        <StyledHeading heading={"Education".to_owned()} />
                        <Timeline items={education_items.to_vec()} />
                    </div>
                    <div class="experience">
                        <StyledHeading heading={"Work Experience".to_owned()} />
                        <Timeline items={experience_items.to_vec()} />
                    </div>
                </div>

                <div class="skills">
                    <div class="technical">
                        <StyledHeading heading={"Technical Skills".to_owned()} />
                        <Skills skills={technical_skills.to_vec()} />
                    </div>
                    <div class="soft">
                        <StyledHeading heading={"Soft Skills".to_owned()} />
                        <Skills skills={soft_skills.to_vec()} />
                    </div>
                </div>
            </main>
        </>
    }
}
