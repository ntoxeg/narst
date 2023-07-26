use narst::memory;
use narst::memory::Memory;
use narst::nal::TruthValue;
use serde_json;

pub fn main() {
    let mut mem = Memory::new();
    let tv = TruthValue::new(0.8, 0.9);
    mem.add("rA9.", tv, None);
    println!("{}", serde_json::to_string_pretty(&mem).unwrap());
    let _storeok = memory::store("testmem.json", &mem);
}
