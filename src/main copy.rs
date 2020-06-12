use orbtk::prelude::*;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Copy, Clone, PartialEq)]
enum Action {
    Do,
    DisplayAdminChoice,
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
    background: Brush,
    count: u32,
    text: String16,
    list: List,
    selected_indices: SelectedIndices,
    counter: usize,
    list_count: usize
    }
);

impl State for MainViewState {
    fn update(&mut self, _: &mut Registry, ctx: &mut Context<'_>) {
        if let Some(action) = self.action {
            match action {
                Action::Do => {
                    // load label from settings file.
                    // ctx.widget().set("info_text", String16::from("ok"));
                    // let widget = ctx.child("kk");
                    let mut text = String::from("");
                    // ctx.child("kk").set("text", String16::from("ok"));
                    if let Ok(lines) = self.read_lines("D:\\GUI\\orbtk\\orb-hello\\src\\hosts") {
                        // 使用迭代器，返回一个（可选）字符串
                        for line in lines {
                            if let Ok(ip) = line {
                                println!("{}", ip);
                                text.push_str(&ip);
                                text.push_str("\n");
                            } else {
                                println!("Some wrong");
                            }
                        }
                        // ctx.child("input1").set("text", String16::from(text));
                        ctx.widget().set("text", String16::from(text));
                    } else {
                        println!("Some wrong");
                    }
                    // let url = *ctx.child("background").get::<String16>("image");
                    // String16::from("res/orbtk-space.png");
                    // let url = widget.get::<String16>;

                    // println!("{:?}", widget)
                }
                Action::DisplayAdminChoice => {
                    ctx.widget().get_mut::<List>("list").push(format!("Item"));
                    // ctx.child("items").set("count", len + 1);
                    println!("in in in")
                }
            }

            self.action = None;
        }
    }
}

// widget!(
//     MainView<MainViewState> {
//         selected_indices: SelectedIndices,
//         counter: usize,
//         list_count: usize,
//         combo_box_list_count: usize,
//         list: List,
//         selection_list: List,
//         combo_box_list: List,
//         selection_list_count: usize,

//         text_two: String16,
//         result: String16
//     }
// );

impl Template for MainView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        let container = Container::create()
            .width(420.0)
            .height(210.0)
            .child(
                Stack::create()
                    .spacing(8.0)
                    .orientation("vertical")
                    .horizontal_alignment("start")
                    .child(
                        Button::create()
                            // .class("single_content")
                            .text("admin")
                            .horizontal_alignment("start")
                            .on_click(move |states, _| {
                                state(id, states).action(Action::DisplayAdminChoice);
                                true
                            })
                            .build(ctx),
                    )
                    .build(ctx),
            )
            .child(
                ItemsWidget::create()
                    .element("items-widget")
                    .id("items")
                    .padding((4.0, 4.0, 4.0, 2.0))
                    .margin((20.0, 0.0, 0.0, 8.0))
                    .items_builder(move |bc, index| {
                        let text = bc.get_widget(id).get::<Vec<String>>("list")[index].clone();

                        Button::create()
                            .margin((0.0, 0.0, 0.0, 2.0))
                            .text(text)
                            .build(bc)
                    })
                    .count(("list_count", id))
                    .build(ctx),
            )
            .build(ctx);

        ctx.append_child_to_overlay(container).unwrap();
        self.name("MainView")
            .selected_indices(HashSet::new())
            .count(0)
            .list(vec!["Item 1".to_string()])
            .list_count(1)
            .child(
                Container::create()
                    // .child(
                    //     ImageWidget::create()
                    //         .image("res/orbtk-space.png")
                    //         .build(ctx),
                    // )
                    .build(ctx),
            )
    }
}

mod sqlite;
use sqlite::create_database;
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
