use leptos::prelude::*;

#[component]
pub fn FlipCard(
    #[prop(into)] title: String,
    #[prop(into)] image_url: String,
    #[prop(into)] description: String,
    #[prop(optional)] actions: Option<ViewFn>,
) -> impl IntoView {
    view! {
        <div class="relative w-full h-[247px] preserve-3d group transition-all duration-700">
            /* Front Face */
            <div class="absolute inset-0 backface-hidden rounded-[5px] overflow-hidden shadow-lg bg-contrast-white flip-front">
                /* Image – 80% */
                <div class="h-4/5 relative">
                    <img
                        src={image_url}
                        alt="card-image"
                        class="w-full h-full object-cover"
                    />
                </div>

                /* Footer – 20% */
                <div class="h-1/5 p-4 flex items-center justify-center bg-primary absolute bottom-0 left-0 right-0">
                    <h5 class="line-clamp-1">{title.clone()}</h5>
                </div>
            </div>

            /* Back Face */
            <div class="absolute inset-0 backface-hidden rounded-[5px] overflow-hidden shadow-lg bg-gradient-to-br from-secondary to-primary text-contrast-white p-6 flex flex-col justify-center rotate-y-180 flip-back">
                <h5 class="mb-3 ">{title.clone()}</h5>
                <p class="text-sm mb-4 leading-relaxed">
                    {description}
                </p>
                <div class="text-xs">
                    {actions.map(|action| action.run())}
                </div>
            </div>
        </div>
    }
}
