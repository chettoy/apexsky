mod apex_offsets;

use apex1_common::global::{MSG, RecOffsets, Store};
use apex1_common::offsets::CustomOffsets;
use obfstr::obfstr as s;
use ohosky_api::common::rpc::ISharedRpcService;
use ohosky_api::common::share::{ISharableValue, RkyvValue};
use ohosky_api::ohosky::rpc::SharedRpcService;

fn real_main() {
    let service: SharedRpcService<RkyvValue<String>, RkyvValue<CustomOffsets>> =
        SharedRpcService::register(&MSG.request_parse_offset);
    let input = service.recv_blocking().unwrap();
    let offsets_str = input.access().unwrap();
    let offsets_obj = apex_offsets::export_offsets_from_str(&offsets_str).unwrap();
    let output = RkyvValue::from_value(offsets_obj).unwrap();
    service.reply(output.clone()).unwrap();
    RecOffsets::set(output).unwrap();
}

fn main() {
    println!("{}", s!("Hello, apexsky dream pro!"));
}

ohosky_api::ohosky_main!(real_main);
