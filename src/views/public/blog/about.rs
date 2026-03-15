use icondata as IconId;
use leptos::prelude::*;
use leptos_icons::Icon;
use leptos_router::components::A;
use reactive_stores::Store;
use wasm_bindgen_futures::spawn_local;

use crate::data::context::{
    store::{AppStateContext, AppStateContextStoreFields},
    users::fetch_site_owner_info,
};

#[component]
pub fn About() -> impl IntoView {
    let current_state = expect_context::<Store<AppStateContext>>();
    let site_owner_info = move || current_state.site_owner_info();

    Effect::new(move || {
        spawn_local(async move {
            let _site_owner_info = fetch_site_owner_info(&current_state, None).await;
        });
    });

    view! {
        <div class="min-h-svh bg-contrast-white">
            <div class="display-constraints">

                // Header — editorial masthead style
                <header class="mb-16 border-b-2 border-mid-gray pb-8">
                    <p class="text-xs tracking-[0.3em] uppercase text-primary mb-3">
                        "About This Blog"
                    </p>
                    <h1
                        class="text-6xl md:text-8xl font-black text-mid-gray leading-none tracking-tight"
                    >
                        "The"
                        <br/>
                        <span class="italic text-primary">"Editorial Chief."</span>
                    </h1>
                </header>

                // Main content — asymmetric two-column
                <div class="grid grid-cols-1 md:grid-cols-12 gap-12 mb-20">

                    // Left column — large pull quote
                    <div class="md:col-span-4">
                        <div class="sticky top-12">
                            <div class="border-l-4 border-primary pl-6 mb-8">
                                <p class="text-2xl leading-snug text-mid-gray italic font-serif">
                                    "\"Sometimes your whole life boils down to one insane move.\""
                                    <br/>
                                    "~Jake Sully"
                                </p>
                            </div>
                            {
                                move || {
                                    let site_owner_info = site_owner_info().get();

                                    view!{
                                        // Avatar placeholder
                                        <div class="w-full aspect-square bg-mid-gray rounded relative overflow-hidden mb-4">
                                            <div class="absolute inset-0 flex items-end p-4 z-10">
                                                <div>
                                                    <p class="text-primary text-lg font-serif italic">{site_owner_info.full_name}</p>
                                                </div>
                                            </div>
                                            // Decorative grid overlay
                                            <div
                                                class="absolute inset-0"
                                                style=format!(
                                                    "background-image: repeating-linear-gradient(0deg, rgba(247,244,239,0.15) 0px, rgba(247,244,239,0.15) 1px, transparent 1px, transparent 40px), repeating-linear-gradient(90deg, rgba(247,244,239,0.15) 0px, rgba(247,244,239,0.15) 1px, transparent 1px, transparent 40px), url('{}?width=800'); background-size: cover; background-position: center;",
                                                    site_owner_info.profile_picture.unwrap_or_default()
                                                )
                                            ></div>
                                        </div>
                                        // Social links
                                        <div class="flex gap-[15px] text-mid-gray">
                                            <A href="#" target="_blank">
                                                <Icon width="1.5rem" height="1.5rem" icon=IconId::BsLinkedin />
                                            </A>
                                            <A href="#" target="_blank">
                                                <Icon width="1.5rem" height="1.5rem" icon=IconId::BsTwitterX />
                                            </A>
                                            <A href="#" target="_blank">
                                                <Icon width="1.5rem" height="1.5rem" icon=IconId::BsGithub />
                                            </A>
                                        </div>
                                    }
                                }
                            }
                        </div>
                    </div>

                    // Right column — body copy
                    <div class="md:col-span-8 md:border-l md:border-light-gray md:pl-12">

                        // Drop cap first paragraph
                        <p class="text-lg leading-relaxed text-gray mb-6">
                            <span
                                class="float-left text-8xl leading-none mr-3 mt-1 text-primary font-black"
                                style="font-family: 'Georgia', serif; line-height: 0.75;"
                            >
                                "H"
                            </span>
                            "ello. I'm a writer, software engineer, and curious mind who believes that the best ideas live at the intersection of technology and nature. This blog is my attempt to map that territory by focusing on how technology can be used to solve real-life problems and not just financial problems."
                        </p>

                        <p class="text-lg leading-relaxed text-gray mb-6">
                            "I've spent years building things on the web, and somewhere along the way I realized that the most interesting problems aren't in the browser. They sit somewhere at the intersection of nature(analog) and the digital realm. I discovered the fun of converting physical quantities into electrical signals, making meaning out of it and using the newfound information to inspire decision making over the internet. This is how I began my journey in the Internet of Things(IoT)."
                        </p>

                        <p class="text-lg leading-relaxed text-gray mb-10">
                            "You'll find articles on software engineering, systems design, the Internet of Things, lifestyle, and whatever else has captured my attention this week. I try to write things I'd want to read: precise, honest, and occasionally opinionated."
                        </p>

                        // Divider
                        <div class="flex items-center gap-4 mb-10">
                            <div class="flex-1 h-px bg-light-gray"></div>
                            <span class="text-primary text-lg">"✦"</span>
                            <div class="flex-1 h-px bg-light-gray"></div>
                        </div>

                        // What to expect section
                        <h2 class="text-xs tracking-[0.3em] uppercase text-primary mb-6">"What You'll Find Here"</h2>

                        <div class="grid grid-cols-1 sm:grid-cols-2 gap-0 mb-10">
                            {[
                                ("Engineering", "Deep dives into software, architecture, and the craft of building."),
                                ("Essays", "Long-form thinking on technology, culture, and everything between."),
                                ("Tutorials", "Practical guides from hard-won experience in the trenches."),
                                ("Notes", "Short observations, links, and things worth remembering."),
                            ].into_iter().map(|(title, desc)| view! {
                                <div class="border-t border-light-gray py-5 pr-6">
                                    <h3 class="text-sm font-bold tracking-wide text-mid-gray mb-1 uppercase">{title}</h3>
                                    <p class="text-sm text-mid-gray leading-relaxed">{desc}</p>
                                </div>
                            }).collect::<Vec<_>>()}
                        </div>

                        // Divider
                        <div class="flex items-center gap-4 mb-10">
                            <div class="flex-1 h-px bg-light-gray"></div>
                            <span class="text-primary text-lg">"✦"</span>
                            <div class="flex-1 h-px bg-light-gray"></div>
                        </div>

                        // Contact
                        <h2 class="text-xs tracking-[0.3em] uppercase text-primary mb-4">"Get In Touch"</h2>
                        <p class="text-lg leading-relaxed text-gray mb-6">
                            "Have a question, a project idea, or just want to say hello? I read every email and try to reply to all of them."
                        </p>
                        <A
                            href="mailto:info@techietenka.com"
                            attr:class="inline-block bg-mid-gray text-contrast-white px-8 py-3 text-sm tracking-widest uppercase hover:bg-primary transition-colors duration-300 rounded-[5px] cursor-pointer"
                        >
                            "Write To Me"
                        </A>
                    </div>
                </div>

                // Bottom stats strip
                <div class="border-t-2 border-mid-gray pt-8 grid grid-cols-2 md:grid-cols-4 gap-8">
                    {[
                        ("48+", "Articles Published"),
                        ("2024", "Year Founded"),
                        ("12k+", "Monthly Readers"),
                        ("Weekly", "Publishing Cadence"),
                    ].into_iter().map(|(stat, label)| view! {
                        <div>
                            <p class="text-4xl font-black text-mid-gray leading-none mb-1">{stat}</p>
                            <p class="text-xs tracking-widest uppercase text-primary">{label}</p>
                        </div>
                    }).collect::<Vec<_>>()}
                </div>

            </div>

            // Bottom rule
            <div class="w-full h-[1px] bg-mid-gray mt-1"></div>
            <div class="w-full h-[3px] bg-mid-gray mt-1"></div>
        </div>
    }
}
