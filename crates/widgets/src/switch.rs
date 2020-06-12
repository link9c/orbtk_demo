use super::behaviors::MouseBehavior;
use crate::prelude::*;

static SWITCH_TRACK: &'static str = "switch_track";
static SWITCH_TOGGLE: &'static str = "switch_toggle";

/// State to handle the position of switch toggle.
#[derive(Default, AsAny)]
pub struct SwitchState {
    selected: bool,
    switch_toggle: Entity,
}

impl SwitchState {
    fn toggle_selection(&mut self) {
        self.selected = !self.selected;
    }
}

impl State for SwitchState {
    fn init(&mut self, _: &mut Registry, ctx: &mut Context) {
        self.switch_toggle = ctx
            .entity_of_child(SWITCH_TOGGLE)
            .expect("SwitchState.init: Switch toggle child could not be found.");
    }

    fn update(&mut self, _: &mut Registry, ctx: &mut Context<'_>) {
        if *ctx.widget().get::<bool>("selected") == self.selected {
            return;
        }

        ctx.widget().set("selected", self.selected);

        let element = ctx.widget().clone::<Selector>("selector").element.unwrap();

        if let Some(parent) = ctx.parent_entity_by_element(&*element) {
            ctx.get_widget(parent).update_theme_by_state(false);
        }

        {
            let mut switch_toggle = ctx.get_widget(self.switch_toggle);

            if self.selected {
                switch_toggle.set("horizontal_alignment", Alignment::from("end"));
                add_selector_to_widget("selected", &mut switch_toggle);
            } else {
                switch_toggle.set("horizontal_alignment", Alignment::from("start"));
                remove_selector_from_widget("selected", &mut switch_toggle);
            }

            switch_toggle.update_theme_by_state(true);
        }

        ctx.push_event_strategy_by_entity(
            ChangedEvent(ctx.entity),
            ctx.entity,
            EventStrategy::Direct,
        );

        ctx.get_widget(self.switch_toggle)
            .update_theme_by_state(false);
    }
}

widget!(
    /// The `Switch` widget can be switch between `on` and `off`.
    ///
    /// **CSS element:** `switch`
    Switch<SwitchState>: MouseHandler {
        /// Sets or shares the background property.
        background: Brush,

        /// Sets or shares the border radius property.
        border_radius: f64,

        /// Sets or shares the border thickness property.
        border_width: Thickness,

        /// Sets or shares the border brush property.
        border_brush: Brush,

        /// Sets or shares the padding property.
        padding: Thickness,

        /// Sets or shares the pressed property.
        pressed: bool,

        /// Sets or shares the selected property.
        selected: bool
    }
);

impl Template for Switch {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("Switch")
            .element("switch")
            .pressed(false)
            .selected(false)
            .width(36.0)
            .height(30.0)
            .border_radius(8.0)
            .border_width(1.0)
            .padding(4.0)
            .child(
                MouseBehavior::create()
                    .pressed(id)
                    .enabled(id)
                    .target(id.0)
                    .on_click(move |states, _| {
                        states.get_mut::<SwitchState>(id).toggle_selection();
                        false
                    })
                    .child(
                        Grid::create()
                            .child(
                                Container::create()
                                    .element(SWITCH_TRACK)
                                    .vertical_alignment("center")
                                    .build(ctx),
                            )
                            .child(
                                Container::create()
                                    .element(SWITCH_TOGGLE)
                                    .id(SWITCH_TOGGLE)
                                    .vertical_alignment("center")
                                    .horizontal_alignment("start")
                                    .width(20.0)
                                    .height(20.0)
                                    .border_radius(10.0)
                                    .build(ctx),
                            )
                            .build(ctx),
                    )
                    .build(ctx),
            )
    }
}
