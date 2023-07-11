# gopro-cli

cli tools that downloads files from a gopro camera into a specified folder

it's supposed to handle all the edge cases and be a direct replacement for the gopro app and gopro cloud subscription

## synopsis
```
Usage: gopro-cli [OPTIONS] --download-path <PATH>

Options:
  -d, --download-path <PATH>  folder to download files to
  -g, --gopro-host <HOST>     gopro host [default: http://10.5.5.9:8080]
      --dry-run               dry run (do not download files)
  -b, --bluetooth             starts the camera's AP with bluetooth
  -h, --help                  Print help
  -V, --version               Print version
```

## example

```sh
gopro-cli -b --download-path /tmp/gopro
```

## usage

remember to connect to the camera's wifi network the first time you use it
