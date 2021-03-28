use cursive::*;

enum State {
    Default,
}
struct StateContainer {
    state: State,
}

impl StateContainer {
    fn new() -> StateContainer {
        StateContainer {
            state: State::Default,
        }
    }
}

struct REditor<'a> {
    siv: CursiveRunnable,
    state: StateContainer,
    canvas: views::Canvas<&'a StateContainer>,
}
impl REditor<'_> {
    fn new() -> REditor<'static> {
        let s = StateContainer::new();
        REditor {
            siv: default(),
            state: s,
            canvas: views::Canvas::new(&s),
        }
    }
    fn init(&mut self) {
        self.canvas.set_required_size(|_, constraint| constraint);
        self.canvas.set_draw(|state, printer| match state.state {
            State::Default => {
                printer.print((printer.size.x / 2, printer.size.y / 2), "test");
            }
        });
        self.siv.add_fullscreen_layer(self.canvas);
        self.siv.run();
    }
}

fn main() {
    let mut editor = REditor::new();
    editor.init();
}
