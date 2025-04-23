use gui_lib::{Button, Draw, Screen};

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // draw select box
    }
}

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 100,
                height: 50,
                options: vec!["yes".to_string(), "no".to_string(), "maybe".to_string()],
            }),
            Box::new(Button {
                width: 100,
                height: 100,
                label: "ok".to_string(),
            }),
        ],
    };

    screen.run();
}
