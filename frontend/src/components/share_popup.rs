use crate::{
    agents::{EventAgent, GlobalEvent},
    components::Popup,
};
use yew::prelude::*;
use yew_agent::{Bridge, Bridged};

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct ShareProps {
    pub url: String,
}

#[derive(Debug)]
pub enum ShareLink {
    Twitter,
    Whatsapp,
    Sms,
    Mail,
}

pub enum Msg {
    GlobalEvent(GlobalEvent),
    Close,
    Copy,
    Share(ShareLink),
}

pub struct SharePopup {
    show: bool,
    copied_to_clipboard: bool,
    url: String,
    #[allow(dead_code)]
    events: Box<dyn Bridge<EventAgent>>,
    qr_image: String,
}

impl Component for SharePopup {
    type Message = Msg;
    type Properties = ShareProps;

    fn create(ctx: &Context<Self>) -> Self {
        let events = EventAgent::bridge(ctx.link().callback(Msg::GlobalEvent));

        use qrcode::{render::svg, EcLevel, QrCode, Version};
        let code =
            QrCode::with_version(ctx.props().url.clone(), Version::Normal(6), EcLevel::M).unwrap();
        let qr_image = code
            .render()
            .min_dimensions(100, 100)
            .dark_color(svg::Color("#000000"))
            .light_color(svg::Color("#ffffff"))
            .build();

        Self {
            qr_image,
            url: ctx.props().url.clone(),
            show: false,
            copied_to_clipboard: false,
            events,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::GlobalEvent(e) => {
                if matches!(e, GlobalEvent::OpenSharePopup) {
                    self.show = true;
                    return true;
                }
                false
            }
            Msg::Close => {
                self.show = false;
                true
            }
            Msg::Copy => {
                self.copied_to_clipboard = true;
                gloo_utils::window()
                    .navigator()
                    .clipboard()
                    .map(|c| c.write_text(&self.url));
                true
            }
            Msg::Share(share) => {
                match share {
                    ShareLink::Mail => location_href(format!("mailto:?&body={}", self.url)),
                    ShareLink::Twitter => {
                        gloo_utils::window()
                            .open_with_url(
                                format!(
                                    "https://twitter.com/intent/tweet?via=liveask1&text={}",
                                    self.url
                                )
                                .as_str(),
                            )
                            .unwrap();
                    }
                    ShareLink::Whatsapp => {
                        location_href(format!("whatsapp://send?text={}", self.url))
                    }
                    ShareLink::Sms => location_href(format!("sms:?&body={}", self.url)),
                }
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if self.show {
            let div = gloo_utils::document().create_element("div").unwrap();
            div.set_inner_html(&self.qr_image);
            let qr_svg = Html::VRef(div.into());

            let on_close = ctx.link().callback(|_| Msg::Close);
            let on_click_copy = ctx.link().callback(|_| Msg::Copy);
            let on_click_share_twitter = ctx.link().callback(|_| Msg::Share(ShareLink::Twitter));
            let on_click_share_mail = ctx.link().callback(|_| Msg::Share(ShareLink::Mail));
            let on_click_share_whatsapp = ctx.link().callback(|_| Msg::Share(ShareLink::Whatsapp));
            let on_click_share_sms = ctx.link().callback(|_| Msg::Share(ShareLink::Sms));

            html! {
                <Popup class="share-popup" {on_close}>
                    <div class="title">
                        {
                            "Share this Link"
                        }
                    </div>

                    <div class="link-box">
                        <div class="link">
                            {
                                self.url.clone()
                            }
                        </div>
                        <div class="copy" onclick={on_click_copy}>
                            {
                                if self.copied_to_clipboard {"Copied"} else {"Copy"}
                            }
                        </div>
                    </div>

                    <div class="sharebuttons">
                        <div onclick={on_click_share_twitter}>
                            <img src="/assets/share/share-twitter.svg" />
                        </div>
                        <div onclick={on_click_share_mail}>
                            <img src="/assets/share/share-email.svg" />
                        </div>
                        <div onclick={on_click_share_sms}>
                            <img src="/assets/share/share-sms.svg" />
                        </div>
                        <div onclick={on_click_share_whatsapp}>
                            <img src="/assets/share/share-whatsapp.svg" />
                        </div>
                    </div>

                    <div class="qr">
                        {qr_svg}
                    </div>

                    <div class="print">
                        {"Show print version"}
                    </div>
                </Popup>
            }
        } else {
            html! {}
        }
    }
}

fn location_href(url: String) {
    gloo_utils::document()
        .location()
        .unwrap()
        .set_href(url.as_str())
        .unwrap();
}
