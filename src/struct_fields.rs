/**
 * Copyright 2025 The MITRE Corporation

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
 */

use chrono::TimeZone;
use seal_lib::Value;

/// Defines into_ilf_attributes method, which allows allows for all fields in a struct to be serialized into a flat ILF-appropriate list of keys and values.
pub trait IntoILFAttributes {
    fn into_ilf_attributes<'b>(&self, path: String, store: &'b mut Vec<(String, Value)>);
}

impl<T> IntoILFAttributes for Vec<T>
where
    T: IntoILFAttributes,
{
    fn into_ilf_attributes<'b>(&self, path: String, store: &'b mut Vec<(String, Value)>) {
        for (index, item) in self.iter().enumerate() {
            item.into_ilf_attributes(path.clone() + "." + index.to_string().as_str(), store);
        }
    }
}
impl<T> IntoILFAttributes for protobuf::MessageField<T>
where
    T: IntoILFAttributes,
{
    fn into_ilf_attributes<'b>(&self, path: String, store: &'b mut Vec<(String, Value)>) {
        match &self.0 {
            Some(obj) => obj.into_ilf_attributes(path, store),
            None => store.push((path, Value::VNone)),
        }
    }
}
impl<T> IntoILFAttributes for protobuf::EnumOrUnknown<T>
where
    T: protobuf::Enum, // T: Deref<Target = i32>,
{
    fn into_ilf_attributes<'b>(&self, path: String, store: &'b mut Vec<(String, Value)>) {
        store.push((path, Value::VNum(seal_lib::Numeric::Int(self.value()))));
    }
}

impl<T> IntoILFAttributes for Option<T>
where
    T: IntoILFAttributes,
{
    fn into_ilf_attributes<'b>(&self, path: String, store: &'b mut Vec<(String, Value)>) {
        match self {
            Some(a) => a.into_ilf_attributes(path, store),
            None => store.push((path, Value::VNone)),
        }
    }
}

// Basic Types

impl IntoILFAttributes for String {
    fn into_ilf_attributes<'b>(&self, path: String, store: &'b mut Vec<(String, Value)>) {
        store.push((path, Value::VString((*self).clone())));
    }
}

impl IntoILFAttributes for i32 {
    fn into_ilf_attributes<'b>(&self, path: String, store: &'b mut Vec<(String, Value)>) {
        store.push((path, Value::VNum((*self).into())));
    }
}

impl IntoILFAttributes for i64 {
    fn into_ilf_attributes<'b>(&self, path: String, store: &'b mut Vec<(String, Value)>) {
        store.push((path, Value::VNum((*self).into())));
    }
}

impl IntoILFAttributes for f64 {
    fn into_ilf_attributes<'b>(&self, path: String, store: &'b mut Vec<(String, Value)>) {
        store.push((path, Value::VNum((*self).into())));
    }
}

impl IntoILFAttributes for bool {
    fn into_ilf_attributes<'b>(&self, path: String, store: &'b mut Vec<(String, Value)>) {
        store.push((path, Value::VBoolean(self.clone())));
    }
}

// Special Cases

impl IntoILFAttributes for protobuf::well_known_types::timestamp::Timestamp {
    fn into_ilf_attributes<'b>(&self, path: String, store: &'b mut Vec<(String, Value)>) {
        // Conversion cannot fail, unwrap is safe.
        let time = chrono::Utc
            .timestamp_opt(self.seconds, self.nanos as u32)
            .unwrap();

        store.push((path, Value::VString(time.to_rfc3339())));
    }
}

impl IntoILFAttributes for protobuf::well_known_types::struct_::Value {
    fn into_ilf_attributes<'b>(&self, path: String, store: &'b mut Vec<(String, Value)>) {
        // Ignores special fields
        use protobuf::well_known_types::struct_::value::Kind;
        match &self.kind {
            Some(Kind::ListValue(val)) => val.values.into_ilf_attributes(path, store),
            Some(Kind::NullValue(val)) => val.into_ilf_attributes(path, store),
            Some(Kind::StructValue(val)) => val.into_ilf_attributes(path, store),
            Some(Kind::NumberValue(val)) => store.push((path, Value::VNum((*val).into()))),
            Some(Kind::StringValue(val)) => store.push((path, Value::VString(val.clone()))),
            Some(Kind::BoolValue(val)) => store.push((path, Value::VBoolean(*val))),
            Some(other) => println!("Cannot serialize unknown variant of 'Kind': {:?}", other),
            None => store.push((path, Value::VNone)),
        }
    }
}

impl IntoILFAttributes for protobuf::well_known_types::struct_::Struct {
    fn into_ilf_attributes<'b>(&self, path: String, store: &'b mut Vec<(String, Value)>) {
        // Ignores special fields
        for (key, value) in self.fields.iter() {
            value.into_ilf_attributes(path.clone() + "." + key, store);
        }
    }
}

// Ignore Special Fields
impl IntoILFAttributes for protobuf::SpecialFields {
    fn into_ilf_attributes<'b>(&self, path: String, store: &'b mut Vec<(String, Value)>) {
        let _ = path;
        let _ = store;
        ()
    }
}
