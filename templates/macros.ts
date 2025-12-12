{% macro param_list(func) %}
	{%- for arg in func.arguments() -%}
	    {%- let type_ = arg.as_type() -%}
	    {{ arg.name() | typescript_var_name }}: {{ arg | typescript_type_name }}
		{%- if !loop.last %}, {% endif -%}
	{%- endfor -%}
{%- endmacro %}

{%- macro docstring(defn, indent_level) %}
{%- match defn.docstring() %}
{%- when Some(s) %}
{{ s | typescript_docstring(indent_level) }}
{%- else %}
{%- endmatch %}
{%- endmacro %}

{%- macro function_return_type(func_def) -%}
    {%- if func_def.is_async() -%}Promise<{%- endif -%}
        {%- if let Some(ret_type) = func_def.return_type() -%}
            {{ ret_type | typescript_type_name }}
        {%- else -%}
            void
        {%- endif %}
    {%- if func_def.is_async() -%}>{%- endif -%}
{%- endmacro -%}
