use leptos::prelude::*;
use leptos_meta::*;

use crate::components::molecules::{
    flip_card::FlipCard, headline::Headline, section_title::SectionTitle, top_nav::TopNav,
};

#[island]
pub fn About() -> impl IntoView {
    view! {
        <Title text="About"/>
        <main>
            <div class="min-h-screen bg-navy flex flex-col gap-[40px]">
                <TopNav />
                <Headline title="About Me" description="Who Am I?" />
                <div class="flex flex-col md:flex-row md:justify-center md:gap-[20px] mx-[20px] md:mx-[100px] text-light-gray">
                    <div class="max-w-[400px] h-[479px] relative md:basis-1/2">
                        <img src="http://localhost:3001/view/e8d382ad-a79b-423d-a304-7e74c42c9cfc" alt="gallery-pic" class="rounded-[5px] w-[299px] h-[429px] object-cover"/>
                        <img src="http://localhost:3001/view/f069333b-361f-4402-9383-63c3e4c58cf5" alt="gallery-pic" class="rounded-[5px] w-[196px] h-[218px] absolute bottom-0 right-0 object-cover"/>
                    </div>
                    <div class="max-w-[400px] flex flex-col gap-[20px] md:basis-1/2">
                        <div class="flex flex-col gap-[20px]">
                            <h1 class="text-light-gray">"Hello, I am "<span class="text-primary">"Elon Aseneka Idiong'o"</span></h1>
                            <p>"I’m a Rust developer specializing in embedded systems, full-stack WebAssembly apps, and service-oriented architectures using GraphQL and gRPC. I enjoy solving low-level challenges, designing reusable components, and pushing for both safety and usability in software. Oh, I am a big advocate for DevSecOps.  I also have a deep appreciation for lyrical storytelling, rhythm, and cultural substance, whether in hip-hop or product design."</p>
                        </div>

                        <div class="flex flex-col gap-[20px]">
                            <p><strong class="text-primary">"Age: "</strong>30 years</p>
                            <p><strong class="text-primary">"Country of Residence: "</strong>Kenya</p>
                            <p><strong class="text-primary">"Relocation: "</strong>Open to relocation</p>
                        </div>
                    </div>
                </div>
                <SectionTitle title="My Services" />
                <div class="mx-[20px] md:mx-[100px] grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-[20px]">
                    // <FlipCard />
                    // <FlipCard />
                    // <FlipCard />
                </div>
            </div>
        </main>
    }
}
