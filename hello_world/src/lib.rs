

use tari_template_macros::template;

#[template]
mod hello_world {
    struct HelloWorld {}

    impl HelloWorld {
        pub fn greet() -> String {
            "Hello World!".to_string()
        }
    }
}
