use crate::prelude::*;

widget!(
    /// The `Stack` defines a layout that is used to stack its children vertical or horizontal.
    ///
    /// **CSS element:** `stack`
    Stack {
        /// Sets or shares the orientation property.
        orientation: Orientation,

        /// Margin between widgets in the stack.
        spacing: f64
    }
);

impl Template for Stack {
    fn template(self, _: Entity, _: &mut BuildContext) -> Self {
        self.name("Stack").orientation("vertical").element("stack")
    }

    fn layout(&self) -> Box<dyn Layout> {
        Box::new(StackLayout::new())
    }
}
