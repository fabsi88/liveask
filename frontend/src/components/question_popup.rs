use crate::{
    agents::{EventAgent, GlobalEvent},
    components::Popup,
    fetch,
    local_cache::LocalCache,
};
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;
use yew_agent::{Bridge, Bridged};

pub enum Msg {
    GlobalEvent(GlobalEvent),
    Send,
    Sent,
    Close,
    InputChanged(InputEvent),
}

pub struct QuestionPopup {
    show: bool,
    text: String,
    error: Option<String>,
    #[allow(dead_code)]
    events: Box<dyn Bridge<EventAgent>>,
}

const MAX_WORD_LENGTH: usize = 30;
const MIN_LENGTH: usize = 10;

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct AddQuestionProps {
    pub event: String,
}

impl Component for QuestionPopup {
    type Message = Msg;
    type Properties = AddQuestionProps;

    fn create(ctx: &Context<Self>) -> Self {
        let events = EventAgent::bridge(ctx.link().callback(Msg::GlobalEvent));

        Self {
            show: false,
            events,
            error: None,
            text: String::new(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::GlobalEvent(e) => {
                if matches!(e, GlobalEvent::OpenQuestionPopup) {
                    self.show = true;
                    return true;
                }
                false
            }
            Msg::Close => {
                self.show = false;
                true
            }
            Msg::Send => {
                self.show = false;
                let event_id = ctx.props().event.clone();
                let text = self.text.clone();
                ctx.link().send_future(async move {
                    if let Ok(item) = fetch::add_question(event_id.clone(), text).await {
                        LocalCache::set_like_state(&event_id, item.id, true);
                    }
                    Msg::Sent
                });
                true
            }
            Msg::Sent => false,
            Msg::InputChanged(ev) => {
                let target: HtmlTextAreaElement = ev.target_dyn_into().unwrap();

                self.text = target.value();

                self.error = None;
                if self.text.is_empty() {
                    self.error = Some(String::from("Question cannot be empty"));
                } else if self.text.trim().len() < MIN_LENGTH {
                    self.error = Some(format!(
                        "Question must be at least {} characters long.",
                        MIN_LENGTH
                    ));
                } else if self
                    .text
                    .split_ascii_whitespace()
                    .any(|word| word.len() > MAX_WORD_LENGTH)
                {
                    self.error = Some(String::from(
                        "Question contains a word exceeding max length.",
                    ));
                }

                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if self.show {
            let on_close = ctx.link().callback(|_| Msg::Close);
            let on_click_ask = ctx.link().callback(|_| Msg::Send);

            html! {
            <Popup class="share-popup" {on_close}>
                <div class="newquestion">
                <div class="add-question">
                    <textarea
                        id="questiontext"
                        name="questiontext"
                        maxlength="200"
                        value={self.text.clone()}
                        placeholder="What’s your question?"
                        required=true
                        oninput={ctx.link().callback(Msg::InputChanged)}
                        //TODO:
                        // [maxHeight]="390"
                        // autosize=true
                        >
                    </textarea>

                    <div class="more-info">
                        <div class="chars-info">
                            <code>
                                {format!("{}",200 - self.text.len())}
                            </code>
                        </div>
                        {
                            if let Some(e) = &self.error {
                                html!{<div class="invalid">
                                    <div>
                                        {e.clone()}
                                    </div>
                                </div>}
                            } else {html!{}}
                        }
                    </div>
                </div>
                <button class="dlg-button"
                    onclick={on_click_ask}
                    disabled={!self.is_valid()}
                    >
                    {"Ask!"}
                </button>
                </div>
            </Popup>
            }
        } else {
            html! {}
        }
    }
}

impl QuestionPopup {
    fn is_valid(&self) -> bool {
        self.error.is_none() && !self.text.is_empty()
    }
}
