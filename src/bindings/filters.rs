// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

use askama::Result;
use heck::{ToLowerCamelCase, ToPascalCase, ToUpperCamelCase};
use uniffi_bindgen::interface::{AsType, Type};

pub fn typescript_type_name(
    typ: &impl AsType,
    askama_values: &dyn askama::Values,
) -> Result<String> {
    Ok(match typ.as_type() {
        Type::Int8 => "number /*i8*/".into(),
        Type::Int16 => "number /*i16*/".into(),
        Type::Int32 => "number /*i32*/".into(),
        Type::Int64 => "bigint /*i64*/".into(),
        Type::UInt8 => "number /*u8*/".into(),
        Type::UInt16 => "number /*u16*/".into(),
        Type::UInt32 => "number /*u32*/".into(),
        Type::UInt64 => "bigint /*u64*/".into(),
        Type::Float32 => "number /*f32*/".into(),
        Type::Float64 => "number /*f64*/".into(), // FIXME: is this right for f64? I am not sure `number` is big enough?
        Type::Boolean => "boolean".into(),
        Type::String => "string".into(),
        Type::Bytes => "ArrayBuffer".into(),
        Type::Timestamp => "Date".into(),
        Type::Duration => "number /* in milliseconds */".into(), // ref: https://github.com/jhugman/uniffi-bindgen-react-native/blob/b9301797ef697331d29edb9d2402ea35c218571e/crates/ubrn_bindgen/src/bindings/gen_typescript/miscellany.rs#L31
        Type::Enum { name, .. } | Type::Record { name, .. } => name.to_pascal_case(),
        Type::Object { name, .. } => typescript_class_name(&name, askama_values)?,
        Type::CallbackInterface { name, .. } => name.to_lower_camel_case(),
        Type::Optional { inner_type } => {
            format!(
                "{} | undefined",
                typescript_type_name(&inner_type, askama_values)?
            )
        }
        Type::Sequence { inner_type } => format!(
            "Array<{}>",
            typescript_type_name(&inner_type, askama_values)?
        ),
        Type::Map {
            key_type,
            value_type,
        } => format!(
            "Map<{}, {}>",
            typescript_type_name(&key_type, askama_values)?,
            typescript_type_name(&value_type, askama_values)?,
        ),
        Type::Custom { name, .. } => name.to_pascal_case(),
    })
}

pub fn typescript_ffi_converter_name(
    typ: &impl AsType,
    askama_values: &dyn askama::Values,
) -> Result<String> {
    Ok(match typ.as_type() {
        Type::Int8 => "FfiConverterInt8".into(),
        Type::Int16 => "FfiConverterInt16".into(),
        Type::Int32 => "FfiConverterInt32".into(),
        Type::Int64 => "FfiConverterInt64".into(),
        Type::UInt8 => "FfiConverterUInt8".into(),
        Type::UInt16 => "FfiConverterUInt16".into(),
        Type::UInt32 => "FfiConverterUInt32".into(),
        Type::UInt64 => "FfiConverterUInt64".into(),
        Type::Float32 => "FfiConverterFloat32".into(),
        Type::Float64 => "FfiConverterFloat64".into(),
        Type::Boolean => "FfiConverterBool".into(),
        Type::String => "FfiConverterString".into(),
        Type::Bytes => "FfiConverterBytes".into(),
        Type::Timestamp => "FfiConverterTimestamp".into(),
        Type::Duration => "FfiConverterDuration".into(),
        Type::Enum { name, .. } | Type::Record { name, .. } | Type::Object { name, .. } => {
            typescript_ffi_converter_struct_enum_object_name(&name, askama_values)?
        }
        Type::CallbackInterface { name, .. } => name.to_lower_camel_case(),
        Type::Optional { inner_type } => {
            format!(
                "(new FfiConverterOptional({}))",
                typescript_ffi_converter_name(&inner_type, askama_values)?
            )
        }
        Type::Sequence { inner_type } => format!(
            "(new FfiConverterArray({}))",
            typescript_ffi_converter_name(&inner_type, askama_values)?
        ),
        Type::Map {
            key_type,
            value_type,
        } => format!(
            "(new FfiConverterMap({}, {}))",
            typescript_ffi_converter_name(&key_type, askama_values)?,
            typescript_ffi_converter_name(&value_type, askama_values)?,
        ),
        Type::Custom { name, .. } => format!("/* custom? */ {}", name.to_pascal_case()), // FIXME: what should this be?
    })
}

pub fn typescript_fn_name(raw_name: &str, _: &dyn askama::Values) -> Result<String> {
    Ok(raw_name.to_lower_camel_case())
}

pub fn typescript_var_name(raw_name: &str, _: &dyn askama::Values) -> Result<String> {
    Ok(raw_name.to_lower_camel_case())
}

pub fn typescript_class_name(raw_name: &str, _: &dyn askama::Values) -> Result<String> {
    Ok(raw_name.to_pascal_case())
}

pub fn typescript_ffi_converter_struct_enum_object_name(
    struct_name: &str,
    _: &dyn askama::Values,
) -> Result<String> {
    Ok(format!(
        "FfiConverterType{}",
        struct_name.to_upper_camel_case()
    ))
}

pub fn typescript_docstring(s: &str, _: &dyn askama::Values, level: &i32) -> Result<String> {
    let contents = textwrap::indent(&textwrap::dedent(s), " * ");
    let comment = format!("/**\n{contents}\n */");
    Ok(textwrap::indent(&comment, &" ".repeat(*level as usize)))
}
