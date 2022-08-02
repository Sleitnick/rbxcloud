# Experience API

## Publish Experience
Publish an experience to Roblox. This takes an `*.rbxl` file and publishes it to a specific Place ID.
```
USAGE:
    rbxcloud experience publish --filename <FILENAME> --place-id <PLACE_ID> --universe-id <UNIVERSE_ID> --version-type <VERSION_TYPE> --api-key <API_KEY>

OPTIONS:
    -a, --api-key <API_KEY>              Roblox Open Cloud API Key
    -f, --filename <FILENAME>            Filename (full or relative) of the RBXL file
    -h, --help                           Print help information
    -p, --place-id <PLACE_ID>            Place ID of the experience
    -u, --universe-id <UNIVERSE_ID>      Universe ID of the experience
    -t, --version-type <VERSION_TYPE>    Version type [possible values: saved, published]
```

Example:
```
$ rbxcloud experience publish -f myplace.rbxl -v published -p 12345 -u 98765 -a MY_KEY
```
