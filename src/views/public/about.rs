use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_meta::*;
use reactive_stores::Store;

use crate::components::molecules::{
    flip_card::FlipCard, headline::Headline, section_title::SectionTitle, top_nav::TopNav,
};
use crate::data::context::shared::fetch_services;
use crate::data::context::store::{AppStateContext, AppStateContextStoreFields};
use crate::data::context::users::fetch_site_owner_info;

#[component]
pub fn About() -> impl IntoView {
    let current_state = expect_context::<Store<AppStateContext>>();
    let site_owner_info = move || current_state.site_owner_info();
    let services = move || current_state.services();
    let (is_loading, set_is_loading) = signal(false);

    Effect::new(move || {
        set_is_loading.set(true);
        spawn_local(async move {
            let _site_owner_info = fetch_site_owner_info(&current_state, None).await;
            let _fetch_services_res = fetch_services(&current_state, None).await;
            set_is_loading.set(false);
        });
    });

    view! {
        <Title text="About"/>
        <main>
            <div class="min-h-svh flex flex-col gap-[40px]">
                <div class="sticky top-0 z-10 bg-contrast-white dark:bg-navy">
                    <TopNav />
                </div>
                <Headline title="About Me" description="Who Am I?" />
                <div class="flex flex-col md:flex-row md:justify-center md:gap-[20px] display-constraints">
                    <div class="max-w-[400px] h-[479px] relative md:basis-1/2">
                        <img src="https://api.techietenka.com/files/view/ba183079-0126-4fc5-b870-de2ffdbe3bca?width=600" alt="gallery-pic" class="rounded-[5px] w-[299px] h-[429px] object-cover"/>
                        <img src="https://api.techietenka.com/files/view/a92c140b-9871-419c-adef-96d44d8f49c0?width=600" alt="gallery-pic" class="rounded-[5px] w-[196px] h-[218px] absolute bottom-0 right-0 object-cover"/>
                    </div>
                    <div class="max-w-[400px] flex flex-col gap-[20px] md:basis-1/2">
                        <div class="flex flex-col gap-[20px]">
                            <h1>"Hello, I am "<span class="text-primary">{move || site_owner_info().get().full_name}</span></h1>
                            <p>{move || site_owner_info().get().bio}</p>
                        </div>

                        <div class="flex flex-col gap-[20px]">
                            <p><strong class="text-primary">"Age: "</strong>30 years</p>
                            <p><strong class="text-primary">"Country of Residence: "</strong>Kenya</p>
                            <p><strong class="text-primary">"Relocation: "</strong>Open to relocation</p>
                        </div>
                    </div>
                </div>
                <div class="display-constraints">
                    <SectionTitle title="My Services" />
                </div>
                <div class="display-constraints grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-[20px]">
                    {
                        move || services()
                            .get()
                            .iter()
                            .map(|service| {
                                view! {
                                    <FlipCard title={service.title.clone().unwrap_or_default()} image_url={service.thumbnail.clone().unwrap_or_default()} description={service.description.clone().unwrap_or_default()} />
                                }
                            })
                            .collect::<Vec<_>>()
                    }
                </div>
            </div>
        </main>
    }
}
