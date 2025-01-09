use crate::ocsf_types::OCSFEvent;
use serde::ser::{Serialize, SerializeMap, SerializeSeq, Serializer};
use serde_json;

#[derive(serde::Deserialize)]
struct SkipNulls(serde_json::Value);

impl Serialize for SkipNulls {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self.0.clone() {
            serde_json::Value::Null => serializer.serialize_unit(),
            serde_json::Value::Array(vec) => {
                let mut seq = serializer.serialize_seq(Some(vec.len()))?;
                for e in vec.into_iter() {
                    if let serde_json::Value::Null = e {
                        continue;
                    } else {
                        seq.serialize_element(&SkipNulls(e))?;
                    }
                }
                seq.end()
            }
            serde_json::Value::Object(map) => {
                let mut new_map = serializer.serialize_map(Some(map.len()))?;
                for (k, v) in map {
                    if let serde_json::Value::Null = v {
                        continue;
                    } else {
                        new_map.serialize_entry(&k, &SkipNulls(v))?;
                    }
                }
                new_map.end()
            }
            val => val.serialize(serializer),
        }
    }
}

impl TryFrom<&str> for OCSFEvent {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parsed_json = serde_json::from_str::<serde_json::Map<String, serde_json::Value>>(value)
            .map_err(|e| format!("Could not parse JSON to Map: {e}"))?;

        let class_uid = match parsed_json.get("class_uid") {
            Some(serde_json::Value::Number(num)) if num.is_u64() => num.as_u64().unwrap() as usize,
            Some(_) => {
                return Err(String::from("class_uid field is not an unsigned int"));
            }
            None => {
                return Err(String::from("No class_uid field"));
            }
        };

        let stringified = serde_json::to_string(&SkipNulls(serde_json::Value::Object(parsed_json)))
            .map_err(|e| format!("Could not serialize parsed JSON: {e}"))?;

        OCSFEvent::parse_from_id(class_uid, &stringified).map_err(|e| {
            if e.is::<protobuf_json_mapping::ParseError>() {
                let err = e.downcast::<protobuf_json_mapping::ParseError>().unwrap();

                // Print a helpful snippet of context around where the JSON validation failed
                if let Some((err, loc)) = err.to_string().split_once(" at ") {
                    if let Some((line, char)) = loc.split_once(':') {
                        if let Ok(line_num) = line.parse::<usize>() {
                            if let Ok(char_num) = char.parse::<usize>() {
                                let lines = stringified.lines().collect::<Vec<_>>();
                                if line_num-1 < lines.len() && char_num-1 < lines[line_num-1].len() {
                                    // Get line
                                    let line_s = lines[line_num - 1];
                                    // Get start/end indexes for context, being aware of index oob
                                    let context_start = char_num.saturating_sub(5);
                                    let context_end = (char_num + 5).min(line_s.len());
                                    // get ref to snippet
                                    let snippet = &line_s[context_start..context_end];
                                    return format!(
                                        "Could not parse OCSF event from JSON: {err} at {line}:{char}: '{snippet}'"
                                    );
                                }
                            }
                        }
                    }
                }
                format!("Could not parse OCSF event from JSON: {err}")
            } else {
                format!("Could not parse OCSF event from JSON: {e}")
            }
        })
    }
}
