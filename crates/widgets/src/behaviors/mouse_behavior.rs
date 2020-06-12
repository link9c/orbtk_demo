use std::cell::Cell;

use crate::prelude::*;

#[derive(Debug, Copy, Clone)]
enum Action {
    Press(Mouse),
    Release(Mouse),
    Scroll(Point),
}

/// The `MouseBehaviorState` handles the `MouseBehavior` widget.
#[derive(Default, AsAny)]
pub struct MouseBehaviorState {
    action: Cell<Option<Action>>,
    has_delta: Cell<bool>,
}

impl MouseBehaviorState {
    fn action(&self, action: Action) {
        self.action.set(Some(action));
    }
}

impl State for MouseBehaviorState {
    fn update(&mut self, _: &mut Registry, ctx: &mut Context<'_>) {
        if !ctx.widget().get::<bool>("enabled") {
            return;
        }

        if let Some(action) = self.action.get() {
            match action {
                Action::Press(_) => {
                    ctx.widget().set("pressed", true);
                }
                Action::Release(p) => {
                    let pressed: bool = *ctx.widget().get("pressed");
                    ctx.widget().set("pressed", false);

                    if check_mouse_condition(Point::new(p.x, p.y), &ctx.widget()) && pressed {
                        let parent = ctx.entity_of_parent().unwrap();
                        ctx.push_event_by_entity(
                            ClickEvent {
                                position: Point::new(p.x, p.y),
                            },
                            parent,
                        )
                    }
                }
                Action::Scroll(p) => {
                    ctx.widget().set("position", p);
                    self.has_delta.set(true);
                }
            };

            let target: Entity = (*ctx.widget().get::<u32>("target")).into();
            ctx.get_widget(target).update_theme_by_state(false);

            self.action.set(None);
        }
    }

    fn update_post_layout(&mut self, _: &mut Registry, ctx: &mut Context<'_>) {
        if self.has_delta.get() {
            ctx.widget().set("delta", Point::new(0.0, 0.0));
            self.has_delta.set(false);
        }
    }
}

widget!(
    /// The `MouseBehavior` widget is used to handle internal the pressed behavior of a widget.
    ///
    /// **CSS element:** `check-box`
    MouseBehavior<MouseBehaviorState>: MouseHandler {
        /// Sets or shares the target of the behavior.
        target: u32,

        /// Sets or shares the pressed property.
        pressed: bool,

        /// Sets or shares the (wheel, scroll) delta property.
        delta: Point
    }
);

impl Template for MouseBehavior {
    fn template(self, id: Entity, _: &mut BuildContext) -> Self {
        self.name("MouseBehavior")
            .delta(0.0)
            .pressed(false)
            .on_mouse_down(move |states, m| {
                states
                    .get::<MouseBehaviorState>(id)
                    .action(Action::Press(m));
                false
            })
            .on_mouse_up(move |states, m| {
                states
                    .get::<MouseBehaviorState>(id)
                    .action(Action::Release(m));
                false
            })
            .on_scroll(move |states, p| {
                states
                    .get::<MouseBehaviorState>(id)
                    .action(Action::Scroll(p));
                false
            })
    }
}
