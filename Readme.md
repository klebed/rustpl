

# rustpl

Template compiler written in 🦀🦀🦀 Rust, so it's blazingly🔥🔥🔥 fast 🚀🚀🚀 and memory safe!  It may use args, env and json files as input sets of keys/values.

It may be used as generator for: 
1) Declarative/conditional config files
2) Static pages
3) Data reports MD/TXT/etc.

It uses Tera template engine, which allows to use nesting/inheritance of templates for better complex workloads.

Example usage: 
```bash
  rustpl --template samples/main.tpl --template samples/header.tpl\
   --set key=value --render main=/path/filename.txt
```

```jinja2
{# main.tpl #}
{% include "header" %}
You should see value of **key** here: {{ key }}
```

```jinja2
{# header.tpl #}
Hello, rustacean!
This is rustpl output!

```

This would produce following output in /path/filename.txt :
```
Hello, rustacean!
This is rustpl output!

You should see value of **key** here: value    
```


For more advanced guidance on template format, read Tera documentation here: https://keats.github.io/tera/docs/

Full CLI arguments description:
```
#rustpl --help
Usage: rustpl [OPTIONS] --render <TEMPLATE=OUTPUT> --template <TEMPLATE_FILE>

Options:
  -r, --render <TEMPLATE=OUTPUT>  Sets the name of the template to render
  -t, --template <TEMPLATE_FILE>  Specifies the template file to use
  -d, --template-dir <DIR>        Specifies the directory containing templates
  -s, --set <KEY=VALUE>           Sets a variable for the document. You should escape quotes and doublequotes unless you're passing just a string
  -j, --values <FILE>             Path to a JSON file containing values
  -E, --import-env [<PREFIX>]     Import environment variables (optionally filtered by PREFIX, e.g., 'APP_'). WARNING: May expose secrets!
  -v, --verbose                   Enable verbose logging (debug level)
  -h, --help                      Print help
  -V, --version                   Print version
```



## Changelog

### 0.0.4
+ feat: Explicit environment variables import (-E [PREFIX]/ --import-env [PREFIX] enables ENV variable import, while PREFIX filtering out all variables that don't comply => less exposure, more security)
+ feat: Tracing debug logging and better error handling (-v / --verbose acts as RUST_LOG=debug)
+ 

### 0.0.3 

- +fix: glibc --> musl

- +fix: static build 

### 0.0.2

- +fix: Handlebars --> Tera ( <https://keats.github.io/tera/docs> )

### 0.0.1

first try

---

Cli tool for template-based file generation

## ToDo: add helpers

<https://github.com/davidB/handlebars_misc_helpers>

<https://github.com/tephrocactus/gtmpl-rust>
