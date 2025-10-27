{%- import "macros.rs" as macros -%}

#![deny(clippy::all)]

use napi_derive::napi;


{#- ========== #}
{#- Record definitions: #}
{#- ========== #}

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

{#- ========== #}
{#- Enum definitions: #}
{#- ========== #}

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

{#- ========== #}
{#- Object definitions: #}
{#- ========== #}


{#- ========== #}
{#- Function definitions: #}
{#- ========== #}

{% for func_def in ci.function_definitions() %}
{%- if let Some(docstring) = func_def.docstring() -%}
    {%- for line in docstring.split("\n") -%}
/// {{line}}
{% endfor -%}
{%- endif -%}
#[napi]
pub {% if func_def.is_async() %}async {% endif %}fn {{ func_def.name() | rust_fn_name }}({% call macros::rust_param_list(func_def) %}){%- if let Some(ret_type) = func_def.return_type() %} -> {{ ret_type | rust_type_name }} {%- endif %} {
    {{func_def.ffi_func().name()}}({% call macros::rust_ffi_arg_list(func_def.ffi_func()) %})
}

{% endfor %}
