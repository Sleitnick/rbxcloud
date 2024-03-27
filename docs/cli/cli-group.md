# Group API

## Get Group Info
Get information about a group.
```
Usage: rbxcloud.exe group get [OPTIONS] --group-id <GROUP_ID> --api-key <API_KEY>

Options:
  -a, --api-key <API_KEY>    Roblox Open Cloud API Key [env: RBXCLOUD_API_KEY=]
  -g, --group-id <GROUP_ID>  Group ID
  -p, --pretty               Pretty-print the JSON response
  -h, --help                 Print help
```

### Example
```
$ rbxcloud group get -p -g 12345 -a MY_KEY
```

## Get Group Shout
Get a group's current shout and its metadata.
```
Usage: rbxcloud.exe group shout [OPTIONS] --group-id <GROUP_ID> --api-key <API_KEY>

Options:
  -a, --api-key <API_KEY>    Roblox Open Cloud API Key [env: RBXCLOUD_API_KEY=]
  -g, --group-id <GROUP_ID>  Group ID
  -p, --pretty               Pretty-print the JSON response
  -o, --only-message         Only return the shout message string
  -h, --help                 Print help
```

### Example
Get a group's shout and its metadata:
```
$ rbxcloud group shout -p -g 12345 -a MY_KEY
```

Get a group's shout message only:
```
$ rbxcloud group shout -p -g 12345 -a MY_KEY --only-message
```