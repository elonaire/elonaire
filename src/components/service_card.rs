use yew::prelude::*;

use crate::data::models::resource::UserService;

// #[derive(Properties, PartialEq)]
// pub struct ServiceCardProps {
//     pub title: String,
//     pub description: String,
//     pub cover_image: String
// }

#[function_component(ServiceCard)]
pub fn service_card(props: &UserService) -> Html {
    html! {
        <div class="service-card">

          <div class="wrapper">
            <div class="flip-front">
              <h3>{props.title.clone()}</h3>
              <img src={props.image.clone()} alt="cover-image" />
            </div>

            <div class="flip-back">
              <h3>{props.title.clone()}</h3>
              <p>{props.description.clone()}</p>
            </div>
          </div>

        </div>
    }
}
