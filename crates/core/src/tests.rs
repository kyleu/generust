use crate::field_type::FieldType;

fn test_ft(ft: FieldType) {
  let s = serde_json::to_string(&ft).unwrap();
  let d: FieldType = serde_json::from_str(&s).unwrap();
  assert_eq!(d, ft);
}

#[test]
fn field_type_serialization() {
  test_ft(FieldType::String);
  test_ft(FieldType::List {
    t: Box::new(FieldType::String)
  });
  test_ft(FieldType::Map {
    k: Box::new(FieldType::String),
    v: Box::new(FieldType::Boolean)
  });
}
