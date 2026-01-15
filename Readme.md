

# rustpl

Template compiler written in 🦀🦀🦀 Rust, so it's blazingly🔥🔥🔥 fast 🚀🚀🚀 and memory safe!  It may use args, env and json files as input sets of keys/values.

It may be used as generator for: 
1) Declarative/conditional config files
2) Static pages
3) Data reports MD/TXT/etc.

It uses Tera template engine, which allows to use nesting/inheritance of templates for better complex workloads.

## Installation

Normally you may want to use this utility within a docker container, or distribute it as single binary to your machines. If that's the case, refer to [Building](#building) section.

In case if you still want to use utility in your system, then you may choose either to install it with your derfault target, or build statically linked single binary.

Normal:
```bash
cargo install rustpl
```

MUSL statically linked:
```bash
RUSTFLAGS='-C target-feature=+crt-static' cargo install --target x86_64-unknown-linux-musl rustpl
```


## Building

For most anticipated build, as it intended to be, you'll build utility with release script, shipped with the project:
```bash
./release.sh
```
> this would require **x86_64-unknown-linux-musl** rust target to be installed.

After you built binary, you may want to build the docker image. We use this container as additional layer to include it to other containers, so we start with 'scratch' image.

Since you may want to use your own registry, we specifically decoupled repo name and expect it to be found in text file in the parent folder. So you may want to create it first:
```bash
echo "my-internal-registry.domain.local:port/" > ../repository.txt
```
> If no file found, there would be no registry set in container name: rustpl:x.x.x

When registry is set, you just call:
```bash
./docker_build.sh
```


## Example usage: 
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

