use crate::components::logo::Logo;
use yew::prelude::*;

#[function_component(TailwindPage)]
pub fn tailwind_page() -> Html {
    html! {
        <div class="w-screen h-screen transition-colors animate-background-change">
            <div
                class="w-full h-full flex justify-center items-center bg-gradient-to-b from-transparent to-green-700 flex-col"
            >
                <Logo />
                <div
                    class="animate-move-up-once transition-all flex flex-col items-center text-center p-4"
                >
                    <h1
                        class="before:content-['Yew_is_set_up_with_tailwind!'] text-black text-5xl font-bold mt-4"
                    />
                    <div class="text-xl flex flex-col items-center gap-2 pt-2 w-4/5 text-center">
                        <div>{ "Get this template at *link*" }</div>
                        <div>
                            { "Check out the Yew docs " }
                            <a
                                class="underline text-blue-950"
                                href="https://yew.rs/docs/getting-started/introduction"
                                target="_blank"
                            >
                                { "here" }
                            </a>
                        </div>
                        <div>
                            { "Check out this guide by Dmitry Slutskii on setting up Yew with Tailwind " }
                            <a
                                class="underline text-blue-950"
                                href="https://lakret.net/blog/2023-03-10-tailwind-with-yew"
                                target="_blank"
                            >
                                { "here" }
                            </a>
                        </div>
                    </div>
                </div>
                <div class="hidden">{ "Yew is not set up with tailwind." }</div>
            </div>
        </div>
    }
}
