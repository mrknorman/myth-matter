use myth_matter::{MaterialBindingStateRefusalV1, MaterialBindingStateV1};

#[test]
fn mat_5_binding_state_has_stable_tags_and_no_unknown_fallback() {
    assert_eq!(MaterialBindingStateV1::Intact.to_byte(), 0);
    assert_eq!(MaterialBindingStateV1::Loose.to_byte(), 1);
    for tag in 0..=u8::MAX {
        let decoded = MaterialBindingStateV1::from_byte(tag);
        match tag {
            0 => assert_eq!(decoded, Ok(MaterialBindingStateV1::Intact)),
            1 => assert_eq!(decoded, Ok(MaterialBindingStateV1::Loose)),
            _ => assert_eq!(decoded, Err(MaterialBindingStateRefusalV1::UnknownTag(tag))),
        }
    }
}
