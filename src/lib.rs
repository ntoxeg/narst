pub mod memory;
pub mod nal;

#[cfg(test)]
mod tests {
    use crate::memory::Memory;
    use crate::nal::TruthValue;
    use serde_json;

    #[test]
    fn truth_value() {
        let tv = TruthValue::new(0.8, 0.9);
        assert_eq!(
            serde_json::to_string(&tv).unwrap(),
            r#"{"ev":{"s":0.8,"c":0.9}}"#
        );
    }

    #[test]
    fn memory() {
        let mut mem = Memory::new();
        let tv = TruthValue::new(0.8, 0.9);
        mem.add("rA9.", tv, None);
        assert_eq!(
            serde_json::to_string(&mem).unwrap(),
            r#"{"items":[{"id":0,"timestamp":0,"term":"rA9.","tv":{"ev":{"s":0.8,"c":0.9}},"usage_count":0,"embed_id":null}],"last_id":1,"current_timestamp":1}"#
        );
    }
}
