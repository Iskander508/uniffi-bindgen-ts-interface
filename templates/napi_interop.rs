{%- import "macros.rs" as macros -%}

#![deny(clippy::all)]

use napi_derive::napi;
use uniffi::{RustBuffer, RustCallStatus, RustCallStatusCode, UniffiForeignPointerCell};

use {{ci.crate_name()}};


{% macro docstring(optional_docstring) %}
{%- if let Some(docstring) = optional_docstring -%}
    {%- for line in docstring.split("\n") -%}
/// {{line}}{%- if !loop.last %}
{% endif -%}
{%- endfor -%}
{%- endif -%}
{% endmacro %}


{#- ========== #}
{#- Record definitions: #}
{#- ========== #}

{#
{% for record_def in ci.record_definitions() %}
{%- if let Some(docstring) = record_def.docstring() -%}
    {%- for line in docstring.split("\n") -%}
/// {{line}}
{% endfor -%}
{%- endif -%}
#[napi(object)]
pub struct {{ record_def.name() | rust_fn_name }} {
{%- for field_def in record_def.fields() -%}
    {%- if let Some(docstring) = field_def.docstring() -%}
        {%- for line in docstring.split("\n") -%}
    /// {{line}}
    {% endfor -%}
    {%- endif -%}

    {%- let type_ = field_def.as_type() %}
    pub {{field_def.name() | rust_var_name}}: {{field_def | rust_type_name}}
    {%- if !loop.last %}, {% endif -%}
{%- endfor %}
}

{% endfor %}
#}

{#- ========== #}
{#- Enum definitions: #}
{#- ========== #}

{#
{% for enum_def in ci.enum_definitions() %}
{%- if let Some(docstring) = enum_def.docstring() -%}
    {%- for line in docstring.split("\n") -%}
/// {{line}}
{% endfor -%}
{%- endif -%}
#[napi]
pub enum {{ enum_def.name() | rust_fn_name }} {
{%- for variant in enum_def.variants() %}
    {% if let Some(docstring) = variant.docstring() -%}
        {%- for line in docstring.split("\n") -%}
    /// {{line}}
    {% endfor -%}
    {%- endif -%}

    {{variant.name() | rust_enum_variant_name-}}
    {%- if !variant.fields().is_empty() -%}
        { {%- for field_def in variant.fields() -%}
            {%- let type_ = field_def.as_type() %}
            pub {{field_def.name() | rust_var_name}}: {{field_def | rust_type_name}}
            {%- if !loop.last %}, {% endif -%}
        {%- endfor %} }
    {%- endif -%}
    {%- if !loop.last %}, {% endif -%}
{%- endfor %}
}

{% endfor %}
#}

{#- ========== #}
{#- Object definitions: #}
{#- ========== #}


{% for func_def in ci.function_definitions() %}
{% call docstring(func_def.docstring()) %}
#[napi]
pub {% if func_def.is_async() %}async {% endif %}fn {{ func_def.name() | rust_fn_name }}({% call macros::rust_param_list(func_def) %}){%- if let Some(ret_type) = func_def.return_type() %} -> {{ ret_type | rust_type_name }} {%- endif %} {
    {{ci.crate_name()}}_ffi_sys::{{func_def.ffi_func().name() | rust_fn_name}}({% call macros::rust_ffi_arg_list(func_def.ffi_func()) %})
}
{% endfor %}


{#- ========== #}
{#- FFI definitions: #}
{#- ========== #}

mod {{ci.crate_name()}}_ffi_sys {
    {%- for definition in ci.ffi_definitions() %}
        {%- match definition %}
        {%- when FfiDefinition::Struct(ffi_struct) %}
        #[repr(C)]
        pub struct {{ffi_struct.name() | rust_ffi_struct_name}} {
            {% for field in ffi_struct.fields() %}
                {{ field.name() }}: {{ field.type_().borrow() | rust_ffi_type_name }}
            {% endfor %}
        }

        {%- else %}
        {%- endmatch %}

    {%- endfor %}


    #[link(name = "/Users/ryan/w/livekit/rust-sdks/target/release/liblivekit_uniffi.dylib")]
    extern "C" {
    {%- for definition in ci.ffi_definitions() %}
        {%- match definition %}

        {%- when FfiDefinition::CallbackFunction(callback) %}
        pub unsafe fn {{ callback.name() | rust_ffi_callback_name }}(
        {%-   for arg in callback.arguments() %}
            {{ arg.name() }}: {{ arg.type_().borrow() | rust_ffi_type_name }}{% if !loop.last %}, {% endif %}
        {%-   endfor %}
        )
        {%-   if callback.has_rust_call_status_arg() -%}
        {%      if callback.arguments().len() > 0 %}, {% endif %}rust_call_status: &mut RustCallStatus
        {%-   endif %}
        {%-   match callback.return_type() %}
        {%-     when Some(return_type) %} -> {{ return_type | rust_ffi_type_name }}
        {%-     when None %}
        {%-   endmatch %};

        {%- when FfiDefinition::Function(func) %}
        pub unsafe fn {{ func.name() }}(
            {%- for arg in func.arguments() %}
            {{ arg.name() }}: {{ arg.type_().borrow() | rust_ffi_type_name }}
            {%-   if !loop.last %}, {# space #}
            {%-   endif %}
            {%- endfor %}
            {%- if func.has_rust_call_status_arg() %}
            {%-   if !func.arguments().is_empty() %}, {# space #}
            {%   endif %}uniffi_out_err: RustCallStatus
            {%- endif %}
        )
        {%- if let Some(return_type) = func.return_type() %}
            -> {{ return_type.borrow() | rust_ffi_type_name }}
        {%- endif %};

        {%- else %}
        {%- endmatch %}

    {%- endfor %}
    }
}
