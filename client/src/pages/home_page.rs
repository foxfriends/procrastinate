use crate::app_components::Link;
use crate::routes::Route;
use yew::prelude::*;

#[function_component(HomePage)]
pub(crate) fn home_page() -> Html {
    html! {
        <div class="grow flex flex-col items-center justify-center">
            <h1 class="font-display text-2xl w-[800px] mb-2">
                {"This app needs a real name."}
            </h1>
            <div class="flex w-screen">
                <div class="grow h-[8px] bg-yellow" />
                <div class="relative flex w-[800px]">
                    // Yellow top line --|
                    <div class="absolute top-0 inset-x-0 h-[8px] bg-yellow" />
                    <div class="absolute right-0 w-[8px] h-[8px]">
                        <div class="absolute left-0 at-y-center h-[64px] w-[8px] bg-yellow rounded-full" />
                    </div>

                    // Cyan bottom line --o
                    <img class="relative" src="/public/rail-fork.svg" />
                    <div class="mt-auto w-[200px] h-[8px] bg-cyan" />
                    <div class="relative mt-auto h-[8px] w-[48px] translate-x-[-4px]">
                        <div class="absolute w-[8px] h-[8px] bg-cyan at-center rounded-full" />
                        <div class="absolute border-[8px] border-cyan rounded-full h-[48px] w-[48px] at-center" />
                    </div>
                    <div class="mt-auto mb-[4px] translate-y-1/2 flex items-center ml-4">
                        <Link class="font-display text-xl" to={Route::App}>
                            {"Let's go!"}
                        </Link>
                    </div>
                </div>
                <div class="grow" />
            </div>
        </div>
    }
}
