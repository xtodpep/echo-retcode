## A commandline app for returning pre-defined retcodes
```sh
Usage: echo-retcode.exe --which <WHICH>

Options:
  -w, --which <WHICH>  [possible values: banh-mi, blt, tuna-salad]
  -h, --help           Print help
```

## Examples
```sh
> echo-retcode --which banh-mi
```
returns `81`

```sh
> echo-retcode -w blt
```
returns `23`

```sh
> echo-retcode --which tuna-salad
```
returns `42`
