use gloo_utils::format::JsValueSerdeExt;
use std::{
    error::Error,
    fmt::{self, Debug, Display, Formatter},
};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

use shared::{AddEvent, AddQuestion, EditLike, EventData, EventInfo, Item};

/// Something wrong has occurred while fetching an external resource.
#[derive(Debug)]
pub enum FetchError {
    JsonError(JsValue),
    SerdeError(serde_json::error::Error),
}
impl Display for FetchError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::JsonError(e) => Debug::fmt(e, f),
            Self::SerdeError(e) => Debug::fmt(e, f),
        }
    }
}
impl Error for FetchError {}

impl From<JsValue> for FetchError {
    fn from(v: JsValue) -> Self {
        Self::JsonError(v)
    }
}
impl From<serde_json::error::Error> for FetchError {
    fn from(v: serde_json::error::Error) -> Self {
        Self::SerdeError(v)
    }
}

const BASE: &str = "https://api.www.live-ask.com";
// const BASE: &str = "http://localhost:8090";

pub async fn fetch_event(id: String, secret: Option<String>) -> Result<EventInfo, FetchError> {
    let url = if let Some(secret) = secret {
        format!("{}/api/mod/event/{}/{}", BASE, id, secret)
    } else {
        format!("{}/api/event/{}", BASE, id)
    };

    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(&url, &opts)?;

    let window = gloo_utils::window();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into()?;

    let json = JsFuture::from(resp.json()?).await?;
    let res = JsValueSerdeExt::into_serde::<EventInfo>(&json)?;

    Ok(res)
}

pub async fn like_question(
    event_id: String,
    question_id: i64,
    like: bool,
) -> Result<Item, FetchError> {
    let body = EditLike { question_id, like };
    let body = serde_json::to_string(&body)?;
    let body = JsValue::from_str(&body);

    let url = format!("{}/api/event/editlike/{}", BASE, event_id);

    let mut opts = RequestInit::new();
    opts.method("POST");
    opts.body(Some(&body));

    let request = Request::new_with_str_and_init(&url, &opts)?;
    request.headers().set("content-type", "application/json")?;

    let window = gloo_utils::window();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into()?;

    let json = JsFuture::from(resp.json()?).await?;
    let res = JsValueSerdeExt::into_serde::<Item>(&json)?;

    Ok(res)
}

pub async fn add_question(event_id: String, text: String) -> Result<Item, FetchError> {
    let body = AddQuestion { text };
    let body = serde_json::to_string(&body)?;
    let body = JsValue::from_str(&body);

    let url = format!("{}/api/event/addquestion/{}", BASE, event_id);

    let mut opts = RequestInit::new();
    opts.method("POST");
    opts.body(Some(&body));

    let request = Request::new_with_str_and_init(&url, &opts)?;
    request.headers().set("content-type", "application/json")?;

    let window = gloo_utils::window();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into()?;

    let json = JsFuture::from(resp.json()?).await?;
    let res = JsValueSerdeExt::into_serde::<Item>(&json)?;

    Ok(res)
}

pub async fn create_event(
    name: String,
    desc: String,
    email: String,
) -> Result<EventInfo, FetchError> {
    let body = AddEvent {
        data: EventData {
            name,
            description: desc,
            max_likes: 0,
            long_url: None,
            short_url: String::new(),
        },
        moderator_email: email,
    };
    let body = serde_json::to_string(&body)?;
    let body = JsValue::from_str(&body);

    let mut opts = RequestInit::new();
    opts.method("POST");
    opts.body(Some(&body));

    let request = Request::new_with_str_and_init(&format!("{}/api/addevent", BASE), &opts)?;
    request.headers().set("content-type", "application/json")?;

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into()?;

    let json = JsFuture::from(resp.json()?).await?;
    let res = JsValueSerdeExt::into_serde::<EventInfo>(&json)?;

    Ok(res)
}
