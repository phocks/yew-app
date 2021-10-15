use yew::prelude::*;

enum Msg {
  AddOne,
}

struct Model {
  // `ComponentLink` is like a reference to a component.
  // It can be used to send messages to the component
  link: ComponentLink<Self>,
  value: i64,
}

impl Component for Model {
  type Message = Msg;
  type Properties = ();

  fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
    Self { link, value: 0 }
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match msg {
      Msg::AddOne => {
        self.value += 1;

        loop {
          self.value += 1;
          if self.value == 3 {
            println!("three");

            // Skip the rest of this iteration
            continue;
          }
          println!("{}", self.value);
          if self.value > 250000 {
            println!("OK, that's enough");

            // Exit this loop
            break;
          }
        }

        // the value has changed so we need to
        // re-render for it to appear on the page
        true
      }
    }
  }

  fn change(&mut self, _props: Self::Properties) -> ShouldRender {
    // Should only return "true" if new properties are different to
    // previously received properties.
    // This component has no properties so we will always return "false".
    false
  }

  fn view(&self) -> Html {
    html! {
        <p>
            <button onclick=self.link.callback(|_| Msg::AddOne)>{ "Plus" }</button>
            <p>{ self.value }</p>
        </p>
    }
  }
}

fn main() {
  yew::start_app::<Model>();
}
