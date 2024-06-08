use practice::*;

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 1,
                height: 1,
                options: vec![
                    String::from("s1"),
                    String::from("s2"),
                    String::from("s3"),
                ],
            }),
            Box::new(Button {
                width: 1,
                height: 1,
                label: String::from("butooon"),
            }),
        ]
    };

    screen.run();
}
