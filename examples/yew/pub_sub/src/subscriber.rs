use super::event_bus::EventBus;
use yew::{html, Component, Context, Html};
use yew_agent::{Bridge, Bridged};

pub enum Msg {
    NewMessage(String),
}

pub struct Subscriber {
    message: String,
    _producer: Box<dyn Bridge<EventBus>>,
}

impl Component for Subscriber {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            message: "No message yet.".to_owned(),
            _producer: EventBus::bridge(ctx.link().callback(Msg::NewMessage)),
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::NewMessage(s) => {
                self.message = s;
                true
            }
        }
    }

    fn view(&self, _: &Context<Self>) -> Html {
        html! {
            <h1>{ &self.message }</h1>
        }
    }
}
