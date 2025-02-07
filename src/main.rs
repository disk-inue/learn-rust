/* mod calculator;
mod common;
mod dice;
mod todo; */
use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");
static CSS: Asset = asset!("/assets/main.css");

fn main() {
    /* println!("select module");
    println!("1. calulator");
    println!("2. todo");
    println!("3. dice");
    println!("q is end module");
    let select_number: u8 = match common::input("module > ").parse() {
        Ok(num) => num,
        Err(_) => {
            println!("end module");
            return;
        }
    };
    match select_number {
        1 => calculator::exec(),
        2 => todo::exec(),
        3 => dice::exec(),
        _ => {
            println!("end module");
            return; */
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
      document::Stylesheet {href: CSS}
      div {id: "title",
        h1 {"HotDog!"}
      }
      div { id: "dogview",
        img { src:"https://images.dog.ceo/breeds/pitbull/dog-3981540_1280.jpg" }
      }
      div { id: "buttons",
        button {id: "skip", "skip"}
        button {id: "save", "save!"}
      }
    }
}

#[component]
pub fn Hero() -> Element {
    rsx! {
        div {
            id: "hero",
            img { src: HEADER_SVG, id: "header" }
            div { id: "links",
                a { href: "https://dioxuslabs.com/learn/0.6/", "📚 Learn Dioxus" }
                a { href: "https://dioxuslabs.com/awesome", "🚀 Awesome Dioxus" }
                a { href: "https://github.com/dioxus-community/", "📡 Community Libraries" }
                a { href: "https://github.com/DioxusLabs/sdk", "⚙️ Dioxus Development Kit" }
                a { href: "https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus", "💫 VSCode Extension" }
                a { href: "https://discord.gg/XgGxMSkvUM", "👋 Community Discord" }
            }
        }
    }
}
