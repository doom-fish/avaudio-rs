use core::ffi::c_char;
use std::ffi::CStr;

use doom_fish_utils::json::{self, FromJson};

use crate::error::AVAudioError;
use crate::ffi;
use crate::JsonValue;

macro_rules! impl_from_json {
    ($type:ty { $($field:ident: $field_type:ty = $key:literal),+ $(,)? }) => {
        impl doom_fish_utils::json::FromJson for $type {
            fn from_json(
                value: &doom_fish_utils::json::JsonValue,
            ) -> Result<Self, doom_fish_utils::json::JsonError> {
                Ok(Self {
                    $($field: value.field::<$field_type>($key)?),+
                })
            }
        }
    };
}

pub(super) use impl_from_json;

pub fn from_json<T: FromJson>(text: &str) -> Result<T, AVAudioError> {
    json::parse(text)
        .and_then(|value| T::from_json(&value))
        .map_err(|error| {
            AVAudioError::OperationFailed(format!("failed to decode bridge JSON: {error}"))
        })
}

pub fn parse_json_and_free<T: FromJson>(json_ptr: *mut c_char) -> Result<T, AVAudioError> {
    let json = unsafe { CStr::from_ptr(json_ptr) }
        .to_string_lossy()
        .into_owned();
    unsafe { ffi::ava_string_free(json_ptr) };
    from_json(&json)
}

pub trait ToJson {
    fn to_json(&self) -> JsonValue;
}

macro_rules! integer_to_json {
    ($($type:ty),+ $(,)?) => {
        $(
            impl ToJson for $type {
                fn to_json(&self) -> JsonValue {
                    JsonValue::Integer(i64::from(*self))
                }
            }
        )+
    };
}

integer_to_json!(u8, u32, i64);

impl ToJson for f64 {
    fn to_json(&self) -> JsonValue {
        JsonValue::Float(*self)
    }
}

impl ToJson for f32 {
    fn to_json(&self) -> JsonValue {
        if !self.is_finite() {
            return JsonValue::Null;
        }
        JsonValue::Float(
            self.to_string()
                .parse()
                .expect("a finite f32 has a valid f64 representation"),
        )
    }
}

impl ToJson for String {
    fn to_json(&self) -> JsonValue {
        JsonValue::String(self.clone())
    }
}

impl<T: ToJson> ToJson for Option<T> {
    fn to_json(&self) -> JsonValue {
        self.as_ref().map_or(JsonValue::Null, ToJson::to_json)
    }
}

impl<T: ToJson> ToJson for Vec<T> {
    fn to_json(&self) -> JsonValue {
        JsonValue::Array(self.iter().map(ToJson::to_json).collect())
    }
}
