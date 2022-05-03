use yew::prelude::*;
pub enum Message {
    AddOne,
}
pub struct SendButton {
    count: usize,
}
impl Component for SendButton {
    type Message = Message;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self { count: 0 }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html!(
            <button
                onclick= {
                    link.callback(|_x| {Message::AddOne})
                }>
            {format!("Clicked {} times", self.count)}
            </button>
        )
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::AddOne => {
                self.count += 1;
                true
            }
        }
    }
}
