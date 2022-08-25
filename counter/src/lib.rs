use tari_template_macros::template;

#[template]
mod counter {
    struct Counter {
        value: u32,
    }

    impl Counter {
        pub fn new() -> Self {
            Self { value: 0 }
        }

        pub fn value(&self) -> u32 {
            self.value
        }

        pub fn increase(&mut self) {
            self.value += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    
    #[template_stub(module="Counter")]
    struct Counter;

    #[template_test]
    fn increase_works() {
        // initialize the component
        let mut counter = Counter::new();
        assert_eq!(counter.value(), Ok(0_u32));

        // increase the value a couple of times
        counter.increase().unwrap();
        assert_eq!(counter.value(), Ok(1_u32));
        counter.increase().unwrap();
        assert_eq!(counter.value(), Ok(2_u32));
    }
}
