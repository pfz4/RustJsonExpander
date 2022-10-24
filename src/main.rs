use std::fs;

use serde_json::{Value, Map};

fn main() {
    let mut parsed = serde_json::from_str(&fs::read_to_string("demo.json").unwrap()).unwrap();
        

    expand_json(&mut parsed);
}

fn expand_json(data: &mut Value) {
    match data {
        Value::Object(object_attrs) => {
            let keys = object_attrs.keys().cloned().collect::<Vec<String>>();
            for key in keys {
                if key.contains(".") {
                    //key contains . so we need to create a new object

                    //pop value from map
                    let mut value = object_attrs.remove(&key).unwrap();

                    //recursive call to expand nested
                    expand_json(&mut value);

                    //split key
                    let seperator_pos = key.find(".").unwrap();
                    let key_first = key.chars().take(seperator_pos).collect::<String>();
                    let key_second = key.chars().skip(seperator_pos+1).collect::<String>();

                    //check is nested object already exists
                    if let Some(nested_object) = object_attrs.get_mut(&key_first){
                        //append value to existing nested object
                        match nested_object {
                            Value::Object(ref mut nested_object) => {nested_object.insert(key_second, value);},
                            _ => panic!("Name collision")
                        }
                    }else{
                        //create new object with value
                        let mut new_map: Map<String, Value> = Map::new();
                        new_map.insert(key_second, value);

                        object_attrs.insert(key_first, Value::Object(new_map));
                    }
                }else{
                    //no . in key, so we just need to call the method on the value to expand nested values
                    let value = object_attrs.get_mut(&key).unwrap();
                    expand_json(value);
                }
            }
        },
        Value::Array(vals) => vals.iter_mut().for_each(|val|expand_json(val)),
        _ => {}
    }
}