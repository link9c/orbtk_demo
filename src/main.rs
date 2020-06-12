#![allow(dead_code, unused_variables, unused_imports)]
use orbtk::prelude::*;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

mod sqlite;
use sqlite::{check_auth, create_database};
#[derive(Copy, Clone, PartialEq)]
enum Action {
    Ensure,
}
#[derive(Default, AsAny)]
pub struct MainViewState {
    action: Option<Action>,
}

impl MainViewState {
    fn action(&mut self, action: impl Into<Option<Action>>) {
        self.action = action.into();
    }
    fn read_lines<P>(&mut self, filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where
        P: AsRef<Path>,
    {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }
}

type List = Vec<String>;
widget!(
    MainView<MainViewState>
    {
        info_text: String16,
    combo_box_list: List,
    combo_box_list_count: usize

    }
);

impl State for MainViewState {
    fn update(&mut self, _: &mut Registry, ctx: &mut Context<'_>) {
        if let Some(action) = self.action {
            match action {
                Action::Ensure => {
                    let name = ctx.child("name").get::<String16>("text").to_string();
                    let password = ctx.child("password").get::<String16>("text").to_string();
                    match *ctx.child("cb").get::<i32>("selected_index") {
                        0_i32 => {
                            println!("is admin");
                            if check_auth(name, password) {
                                println!("验证成功");
                                ctx.widget().set("info_text", String16::from("验证成功"));
                            } else {
                                println!("验证失败");
                                ctx.widget().set("info_text", String16::from("ooooo"));
                                // ctx.child("result").set("text", String16::from("验证失败"));
                            }
                        }

                        1_i32 => println!("is normal"),
                        _ => {}
                    }
                }
            }

            self.action = None;
        }
    }
}

impl Template for MainView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("MainView")
            .combo_box_list(vec!["Admin".to_string(), "Normal".to_string()])
            .combo_box_list_count(2)
            .child(
                Stack::create()
                    .child(
                        ComboBox::create()
                            .id("cb")
                            // .selected_indices(id)
                            .items_builder(move |bc, index| {
                                let text = bc.get_widget(id).get::<Vec<String>>("combo_box_list")
                                    [index]
                                    .clone();
                                TextBlock::create()
                                    .margin((0.0, 0.0, 0.0, 2.0))
                                    .vertical_alignment("center")
                                    .text(text)
                                    .build(bc)
                            })
                            .selected_index(0)
                            .class("single_content")
                            .horizontal_alignment("start")
                            .count(("combo_box_list_count", id))
                            .build(ctx),
                    )
                    .child(
                        TextBox::create()
                            .id("name")
                            .water_mark("name...")
                            .margin((0.0, 8.0, 0.0, 0.0))
                            .class("single_content")
                            .horizontal_alignment("start")
                            .build(ctx),
                    )
                    .child(
                        TextBox::create()
                            .id("password")
                            .water_mark("password...")
                            .margin((0.0, 8.0, 0.0, 0.0))
                            .class("single_content")
                            .horizontal_alignment("start")
                            .build(ctx),
                    )
                    .child(
                        Button::create()
                            .text("ok")
                            .element("button")
                            .class("primary")
                            .margin((0.0, 8.0, 0.0, 0.0))
                            .icon(material_font_icons::CHECK_FONT_ICON)
                            .horizontal_alignment("start")
                            .on_click(move |states, _| {
                                state(id, states).action(Action::Ensure);
                                true
                            })
                            .build(ctx),
                    )
                    .child(
                        TextBlock::create()
                            .id("result")
                            .margin((0.0, 8.0, 0.0, 0.0))
                            .horizontal_alignment("start")
                            .text(("info_text", id))
                            .build(ctx),
                    )
                    .build(ctx),
            )
    }
}

fn main() {
    // match create_database() {
    //     Ok(_) => println!("ok"),
    //     _ => println!("failed"),
    // };
    Application::new()
        .window(|ctx| {
            Window::create()
                .title("OrbTk - image example")
                .position((100.0, 100.0))
                .size(800.0, 420.0)
                .resizeable(true)
                .child(MainView::create().build(ctx))
                .build(ctx)
        })
        .run();
}

fn state<'a>(id: Entity, states: &'a mut StatesContext) -> &'a mut MainViewState {
    states.get_mut(id)
}
