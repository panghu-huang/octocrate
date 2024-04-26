use crate::schemas::schema::{SchemaType, SchemaTypeDefinition};

#[derive(Debug, Clone, Default)]
pub struct SchemaTypes {
  pub types: Vec<SchemaType>,
  pub is_null: bool,
  pub is_array: bool,
  pub is_object: bool,
  pub is_string: bool,
  pub is_integer: bool,
  pub is_number: bool,
  pub is_boolean: bool,
}

impl SchemaTypes {
  pub fn new(types: Vec<SchemaType>) -> Self {
    let is_null = types.contains(&SchemaType::Null);
    let is_array = types.contains(&SchemaType::Array);
    let is_object = types.contains(&SchemaType::Object);
    let is_string = types.contains(&SchemaType::String);
    let is_integer = types.contains(&SchemaType::Integer);
    let is_number = types.contains(&SchemaType::Number);
    let is_boolean = types.contains(&SchemaType::Boolean);

    SchemaTypes {
      types,
      is_null,
      is_array,
      is_object,
      is_string,
      is_integer,
      is_boolean,
      is_number,
    }
  }

  pub fn to_full_type(&self) -> String {
    match (
      self.is_array,
      self.is_string,
      self.is_integer,
      self.is_number,
      self.is_null,
      self.is_boolean,
      self.is_object,
    ) {
      // [string]
      (false, true, false, false, false, false, false) => "String".into(),
      // [string, null]
      (false, true, false, false, true, false, false) => "Option<String>".into(),
      // [integer]
      (false, false, true, false, false, false, false) => "i64".into(),
      // [number]
      (false, false, false, true, false, false, false) => "f64".into(),
      // [integer, null]
      (false, false, true, false, true, false, false) => "Option<i64>".into(),
      // [number, null]
      (false, false, false, true, true, false, false) => "Option<f64>".into(),
      // [array, string]
      (true, true, false, false, false, false, false) => "Vec<String>".into(),
      // [array]
      (true, false, false, false, false, false, false) => "Vec<serde_json::Value>".into(),
      // [boolean]
      (false, false, false, false, false, true, false) => "bool".into(),
      // [boolean, null]
      (false, false, false, false, true, true, false) => "Option<bool>".into(),
      // [boolean, string]
      // TODO: BooleanOrString
      (false, true, false, false, false, true, false) => "StringOrBool".into(),
      // [string, integer]
      (false, true, true, false, false, false, false) => "StringOrInteger".into(),
      // [string, number]
      (false, true, false, true, false, false, false) => "StringOrNumber".into(),
      // [string, integer, null]
      (false, true, true, false, true, false, false) => "Option<StringOrInteger>".into(),
      // [string, number, null]
      (false, true, false, true, true, false, false) => "Option<StringOrNumber>".into(),
      // [null]
      (false, false, false, false, true, false, false) => "serde_json::Value".into(),
      // []
      // TODO: Why is this here?
      (false, false, false, false, false, false, false) => "serde_json::Value".into(),
      // [object, null]
      (false, false, false, false, true, false, true) => "Option<serde_json::Value>".into(),
      // [object, *]
      // (_, _, _, _, _, _) => "serde_json::Value".into(),
      _ => {
        "serde_json::Value".into()
        // unreachable!(
        //   "Invalid schema types: array: {}, string: {}, integer: {}, null: {}, boolean: {}, object: {}",
        //   self.is_array, self.is_string, self.is_integer, self.is_null, self.is_boolean, self.is_object
        // )
      }
    }
  }

  pub fn to_full_type_with_object(&self, object_name: &String) -> String {
    let integer_or_number = self.is_integer || self.is_number;

    match (
      self.is_object,
      self.is_array,
      self.is_string,
      integer_or_number,
      self.is_boolean,
      self.is_null,
    ) {
      // object
      // [object]
      (true, false, false, false, false, false) => object_name.clone(),
      // [object, null]
      (true, false, false, false, false, true) => format!("Option<{}>", object_name),
      // [object, array]
      // TODO: ObjectOrArray
      (true, true, false, false, false, false) => format!("ObjectOrArray<{}>", object_name),
      // [object, array, null]
      (true, true, false, false, false, true) => format!("Option<ObjectOrArray<{}>>", object_name),
      // [object, string]
      // TODO: ObjectOrString
      (true, false, true, false, false, false) => format!("ObjectOrString<{}>", object_name),
      // [object, string, null]
      (true, false, true, false, false, true) => format!("Option<ObjectOrString<{}>>", object_name),
      // Object may be a string or number
      // [array]
      (false, true, false, false, false, false) => format!("Vec<{}>", object_name),
      // [array, null]
      (false, true, false, false, false, true) => format!("Option<Vec<{}>>", object_name),
      // null
      (false, false, false, false, false, true) => format!("Option<{}>", object_name),
      // All false
      // TODO: should return the object name?
      (false, false, false, false, false, false) => "serde_json::Value".into(),
      // [string]
      // TODO: call to_full_type()
      (false, false, true, false, false, false) => format!("ObjectOrString<{}>", object_name),
      // (true, false, true, true, true, false) => "serde_json::Value".into(),
      // (false, true, true, false, false, false) => "serde_json::Value".into(),
      _ => {
        "serde_json::Value".into()
        // unreachable!(
        //   "Invalid schema types: object: {}, array: {}, string: {}, integer: {}, boolean: {}, null: {}",
        //   self.is_object, self.is_array, self.is_string, self.is_integer, self.is_boolean, self.is_null
        // )
      }
    }
  }

  pub fn merge(&mut self, other: &SchemaTypes) {
    self.types.extend(other.types.iter().cloned());
    self.is_null = self.types.contains(&SchemaType::Null);
    self.is_array = self.types.contains(&SchemaType::Array);
    self.is_object = self.types.contains(&SchemaType::Object);
    self.is_string = self.types.contains(&SchemaType::String);
    self.is_integer = self.types.contains(&SchemaType::Integer);
    self.is_number = self.types.contains(&SchemaType::Number);
    self.is_boolean = self.types.contains(&SchemaType::Boolean);
  }
}

impl From<&SchemaTypeDefinition> for SchemaTypes {
  fn from(value: &SchemaTypeDefinition) -> Self {
    let types = match value {
      SchemaTypeDefinition::String(type_) => {
        vec![
          SchemaType::try_from(type_).unwrap_or_else(|_| panic!("Unknown schema type: {}", type_))
        ]
      }
      SchemaTypeDefinition::Array(types) => types
        .iter()
        .map(|type_| {
          SchemaType::try_from(type_).unwrap_or_else(|_| panic!("Unknown schema type: {}", type_))
        })
        .collect(),
    };

    SchemaTypes::new(types)
  }
}
