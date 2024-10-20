use yew::prelude::*;

#[function_component(Logo)]
pub fn logo() -> Html {
    html! {
        <div class="animate-move-down-once">
            <div
                class="relative flex justify-center items-center w-56 h-56 bg-white rounded-full border-2 border-black animate-spin-slow"
            >
                <div class="absolute z-">{ "ue77" }</div>
                <div
                    class="absolute w-4 h-24 z-10 bg-logo-gray rounded-full transform origin-bottom rotate-0 -mt-24"
                />
                <div
                    class="absolute w-4 h-24 z-10 bg-logo-gray rounded-full transform origin-bottom rotate-[120deg] -mt-24"
                />
                <div
                    class="absolute w-4 h-24 z-10 bg-logo-gray rounded-full transform origin-bottom rotate-[240deg] -mt-24"
                />
                <div class="absolute w-32 h-32 bg-logo-light-green rounded-full" />
                <div class="absolute w-16 h-16 bg-logo-gray rounded-full" />
                <div class="absolute z-10 w-10 h-10 bg-logo-dark-green rounded-full" />
            </div>
        </div>
    }
}
