use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct SkillProps {
    pub icon: String,
    pub name: String,
}

#[derive(Clone, PartialEq, Properties)]
pub struct SkillsProps {
    pub skills: Vec<SkillProps>,
}

#[function_component(Skill)]
pub fn skill(props: &SkillProps) -> Html {

    html! {
        <div class="skill">
            <img class="skill-icon" src={props.icon.clone()} alt="skill-icon" />
            <p class="skill-name">{props.name.clone()}</p>
        </div>
    }
}

#[function_component(Skills)]
pub fn skills(props: &SkillsProps) -> Html {

    html! {
        <div class="skills">
            <div class="three-d-card">
                { for props.skills.iter().map(|skill| html! { <Skill icon={skill.icon.clone()} name={skill.name.clone()} /> }) }
            </div>
        </div>
    }
}
