use serde_json::Value;
use crate::convert::sql_value_convert::SqlValueConvert;

pub fn to_rdbc_values(arg_array:&mut Vec<Value>)->Vec<rdbc::Value>{
    let mut params =vec![];
    for x in arg_array {
        let item=x.to_sql_value_def();
        params.push(rdbc::Value::String(item));
    }
    return params;
}

pub fn to_string(arg:&Vec<rdbc::Value>)->String{
    let mut s = String::new();
    for x in arg {
        s = s + x.to_string().as_str() + ",";
    }
    if s.len() > 0 {
        s.pop();
    }
    s = "{".to_string()+s.as_str();
    s = s + "}";
    return s;
}