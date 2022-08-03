use std::sync::Arc;
use std::sync::atomic::AtomicU32;
use tari_common_types::types::FixedHash;
use tari_crypto::ristretto::RistrettoSecretKey;
use tari_dan_engine::crypto::create_key_pair;
use tari_dan_engine::instruction::{Instruction, InstructionBuilder, InstructionProcessor};
use tari_dan_engine::models::{Component, ComponentId};
use tari_dan_engine::packager::Package;
use tari_dan_engine::runtime::{RuntimeError, RuntimeInterface};
use tari_dan_engine::wasm::compile::build_wasm_module_from_source;
use tari_template_abi::borsh::BorshDeserialize;
use tari_template_abi::{encode_with_len, LogLevel};
use tari_dan_common_types::Hash;

#[test]
fn test_hello_world() {
    let template_test = TemplateTest::new("HelloWorld".to_string(), ".".to_string());
    let result: String = template_test.call_function("greet".to_string(), vec![]);

    // FIXME: without the "encode_with_len" calls, the strings are different because of added padding characters
    assert_eq!(encode_with_len(&result), encode_with_len(&"Hello World!"));
}


struct TemplateTest {
    template_name: String,
    package_id: FixedHash,
    processor: InstructionProcessor<MockRuntimeInterface>,
    secret_key: RistrettoSecretKey,
}

impl TemplateTest {
    pub fn new(template_name: String, template_path: String) -> Self {
        let mut processor = InstructionProcessor::new(MockRuntimeInterface::new());
        let (secret_key, _pk) = create_key_pair();

        let wasm = build_wasm_module_from_source(template_path).unwrap();
        let package = Package::builder().add_wasm_module(wasm).build().unwrap();
        let package_id = package.id();
        processor.load(package);

        Self {
            template_name,
            package_id,
            processor,
            secret_key,
        }
    }

    pub fn call_function<T>(&self, func_name: String, args: Vec<Vec<u8>>) -> T
        where T: BorshDeserialize {
        let instruction = InstructionBuilder::new()
            .add_instruction(Instruction::CallFunction {
                package_id: self.package_id,
                template: self.template_name.clone(),
                function: func_name,
                args,
            })
            .sign(&self.secret_key)
            .build();
        let result = self.processor.execute(instruction).unwrap();

        result[0].decode::<T>().unwrap()
    }

    pub fn call_method<T>(&self, component_id: String, method_name: String, args: Vec<Vec<u8>>) -> T
        where T: BorshDeserialize {
        let instruction = InstructionBuilder::new()
            .add_instruction(Instruction::CallMethod {
                package_id: self.package_id,
                component_id,
                method: method_name,
                args,
            })
            .sign(&self.secret_key)
            .build();
        let result = self.processor.execute(instruction).unwrap();

        result[0].decode::<T>().unwrap()
    }
}


#[derive(Debug, Clone, Default)]
pub struct MockRuntimeInterface {
    ids: Arc<AtomicU32>,
}

impl MockRuntimeInterface {
    pub fn new() -> Self {
        Self {
            ids: Arc::new(AtomicU32::new(0)),
        }
    }

    pub fn next_id(&self) -> u32 {
        self.ids.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
    }
}

impl RuntimeInterface for MockRuntimeInterface {
    fn emit_log(&self, level: LogLevel, message: &str) {
        let level = match level {
            LogLevel::Error => log::Level::Error,
            LogLevel::Warn => log::Level::Warn,
            LogLevel::Info => log::Level::Info,
            LogLevel::Debug => log::Level::Debug,
        };
        eprintln!("[{:?}] {}", level, message);
        log::log!(target: "tari::dan::engine::runtime", level, "{}", message);
    }

    fn create_component(&self, _new_component: Component) -> Result<ComponentId, RuntimeError> {
        Ok((Hash::default(), self.next_id()))
    }
}
