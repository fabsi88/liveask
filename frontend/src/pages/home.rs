use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::Route;

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct HomeProps;

pub struct Home {}
pub enum Msg {
    Example,
    CreateEvent,
    Privacy,
}
impl Component for Home {
    type Message = Msg;
    type Properties = HomeProps;

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::CreateEvent => {
                ctx.link().history().unwrap().push(Route::NewEvent);
                false
            }
            Msg::Example => {
                ctx.link().history().unwrap().push(Route::Event {
                    id: "eventexample".into(),
                });
                false
            }
            Msg::Privacy => {
                ctx.link().history().unwrap().push(Route::Privacy);
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let twitter_svg = include_str!("../../inline-assets/twitter.svg");
        let div = gloo_utils::document().create_element("div").unwrap();
        div.set_inner_html(twitter_svg);
        div.set_id("twitter");
        let twitter_svg = Html::VRef(div.into());

        html! {
            <div id="home">
                <div class="feature-dark">
                    <h1 id="firstheader">
                        {"Real-Time questions from your audience"}
                    </h1>
                    <p>
                        {"Have you ever organized a meetup, conference, or moderated a panel discussion and wanted an easy way to receive real-time
                        questions from your audience? Welcome to Live-Ask."}
                    </p>
                    <button class="button-red" onclick={ctx.link().callback(|_| Msg::CreateEvent)}>
                        {"Create your Event"}
                    </button>
                    <button class="button-dark" onclick={ctx.link().callback(|_| Msg::Example)}>
                        {"View Example"}
                    </button>
                </div>

                <div class="feature-bright">
                    <h1>
                        {"Incognito"}
                    </h1>
                    <img class="img-simple" src="assets/main-incognito.png" />
                    <p>
                        {"No registration necessary - everyone can ask questions and vote. Participant anonymity ensures freedom of speech and a smooth
                        user experience."}
                    </p>
                </div>

                <div class="feature-dark">
                    <h1>
                        {"Effortless"}
                    </h1>
                    <img class="img-simple" src="assets/main-effortless.png" />
                    <p>
                        {"Set up your event in seconds! Share the link with your audience and let them decide what’s hot."}
                    </p>
                </div>

                <div class="feature-bright">
                    <h1>
                        {"Real-Time"}
                    </h1>
                    <img class="img-simple" id="img-realtime" src="assets/main-realtime.png" />
                    <p>
                        {" Designed for live events. Questions can be asked and voted on in real time. This way, you can interact with everyone seamlessly."}
                    </p>
                </div>

                <div class="feature-dark">
                    <h1>
                        {"Cross Platform"}
                    </h1>
                    <img class="img-simple" id="img-crossplatform" src="assets/main-crossplatform.png" />
                    <p>
                        {"Use Live-Ask on your mobile phone, tablet, laptop or desktop computer. Go crazy and cast it to your smart TV, too!"}
                    </p>
                </div>

                <div class="feature-bright">
                    <h1>
                        {"Social"}
                    </h1>
                    <img class="img-simple" src="assets/main-social.png" />
                    <p>
                        {"We want to make sharing as effortless as possible. Have you organized an awesome event? Live-Ask makes it easy to share it
                        with others. You bring the great content, we’ll help you spread the word."}
                    </p>
                </div>

                <div class="feature-dark">
                    <h1>
                        {"It’s free, try it now!"}
                    </h1>
                    <button class="button-red" onclick={ctx.link().callback(|_| Msg::CreateEvent)}>
                        {"Create your Event"}
                    </button>

                    <div class="copyright">
                        {"© 2022 Live-Ask. All right reserved"}
                    </div>

                    <a href="https://twitter.com/liveask1">
                        {twitter_svg}
                    </a>

                    <a class="about" onclick={ctx.link().callback(|_| Msg::Privacy)}>
                        {"Privacy Policy"}
                    </a>

                    <a class="about" href="http://blog.extrawurst.org/general/webdev/2018/04/02/liveask.html">
                        {"About"}
                    </a>

                    <div class="version">
                        {"v.TODO"}
                    </div>
                </div>
            </div>
        }
    }
}
