# Subscription API

## Getting Subscription Info
```
Usage: rbxcloud.exe subscription get [OPTIONS] --universe-id <UNIVERSE_ID> --product <PRODUCT> --subscription <SUBSCRIPTION> --api-key <API_KEY>

Options:
  -u, --universe-id <UNIVERSE_ID>    Universe ID
  -S, --product <PRODUCT>            Subscription product ID
  -s, --subscription <SUBSCRIPTION>  Subscription ID
  -v, --view <VIEW>                  View type [possible values: basic, full]
  -p, --pretty                       Pretty-print the JSON response
  -a, --api-key <API_KEY>            Roblox Open Cloud API Key [env: RBXCLOUD_API_KEY=]
  -h, --help                         Print help
```

### Example
```
$ rbxcloud subscription get -p -u 12345 -S 1234 -s 5678 -a MY_KEY
```
