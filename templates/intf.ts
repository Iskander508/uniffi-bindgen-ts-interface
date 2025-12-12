{%- import "macros.ts" as ts %}

// ==========
// Record definitions:
// ==========
{%- for record_def in ci.record_definitions() %}
{% call ts::docstring(record_def, 0) %}
export interface {{ record_def.name() | typescript_class_name }} {
  {%- for field_def in record_def.fields() -%}
  {% call ts::docstring(field_def, 2) %}
  {%- let type_ = field_def.as_type() %}
  {{field_def.name() | typescript_var_name}}: {{field_def | typescript_type_name}};
  {%- endfor %}
}
{%- endfor -%}

// ==========
// Enum definitions:
// ==========
{% for enum_def in ci.enum_definitions() %}
{%- include "Enum.ts" %}
{% endfor %}

// ==========
// Object definitions:
// ==========
{%- for object_def in ci.object_definitions() %}
{% call ts::docstring(object_def, 0) %}
export interface {{ object_def.name() | typescript_class_name }} {
  {%- for method_def in object_def.methods() -%}
  {% call ts::docstring(method_def, 2) %}
  {{ method_def.name() | typescript_fn_name }}(
    {%- call ts::param_list(method_def) -%}
    ): {% call ts::function_return_type(method_def) %};
  {%- endfor %}
}
{%- endfor -%}

