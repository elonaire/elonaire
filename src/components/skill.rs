use yew::prelude::*;

use crate::data::models::resource::UserSkill;

// #[derive(Clone, PartialEq, Properties)]
// pub struct SkillProps {
//     pub icon: String,
//     pub name: String,
// }

#[derive(Clone, PartialEq, Properties)]
pub struct SkillsProps {
    pub skills: Vec<UserSkill>,
}

#[function_component(Skill)]
pub fn skill(props: &UserSkill) -> Html {

    html! {
        <div class="skill">
            <img class="skill-icon" src={props.image.clone()} alt="skill-icon" />
            <p class="skill-name">{props.name.clone()}</p>
        </div>
    }
}

#[function_component(Skills)]
pub fn skills(props: &SkillsProps) -> Html {

    html! {
        <div class="skills">
            <div class="three-d-card">
                { for props.skills.iter().map(|skill| html! { <Skill image={skill.image.clone()} name={skill.name.clone()} /> }) }
            </div>
        </div>
    }
}
