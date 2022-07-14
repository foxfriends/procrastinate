use crate::app_components::Link;
use crate::routes::Route;
use yew::prelude::*;

#[function_component(HomePage)]
pub(crate) fn home_page() -> Html {
    html! {
        <div class="grow flex flex-col items-center justify-center">
            <h1 class="font-display text-2xl w-[800px] max-w-[calc(100%-32px)] mb-2">
                {"This app needs a real name."}
            </h1>
            <div class="flex w-screen">
                <div class="grow h-[8px] bg-yellow" />
                <div class="relative flex justify-end w-[800px] max-w-[calc(100%-32px)]">
                    // Yellow top line --|
                    <div class="absolute top-0 inset-x-0 h-[8px] bg-yellow shrink-0" />
                    <div class="absolute right-0 w-[8px] h-[8px] shrink-0">
                        <div class="absolute left-0 at-y-center h-[64px] w-[8px] bg-yellow rounded-full" />
                    </div>

                    // Cyan bottom line --o
                    <img class="relative shrink-0" src="/public/rail-fork.svg" />
                    <div class="mt-auto basis-[200px] h-[8px] bg-cyan" />
                    <div class="relative h-[8px] w-[48px] mt-auto shrink-0 translate-x-[-4px]">
                        <div class="absolute at-center w-[8px] h-[8px] shrink-0 bg-cyan rounded-full" />
                        <div class="absolute at-center h-[48px] w-[48px] border-[8px] border-cyan rounded-full" />
                    </div>
                    <div class="flex items-center mt-auto mb-[4px] ml-4 mr-auto shrink-0  whitespace-nowrap translate-y-1/2">
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
