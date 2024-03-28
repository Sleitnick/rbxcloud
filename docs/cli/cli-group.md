# Group API

## Get Group Info
Get information about a group.
```
Usage: rbxcloud group get [OPTIONS] --group-id <GROUP_ID> --api-key <API_KEY>

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
Usage: rbxcloud group shout [OPTIONS] --group-id <GROUP_ID> --api-key <API_KEY>

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

## List Group Roles
List the roles of a given group.
```
Usage: rbxcloud group roles [OPTIONS] --group-id <GROUP_ID> --api-key <API_KEY>

Options:
  -a, --api-key <API_KEY>                  Roblox Open Cloud API Key [env: RBXCLOUD_API_KEY=]
  -g, --group-id <GROUP_ID>                Group ID
  -p, --pretty                             Pretty-print the JSON response
  -m, --max-page-size <MAX_PAGE_SIZE>      Max items returned per page
  -n, --next-page-token <NEXT_PAGE_TOKEN>  Next page token
  -h, --help                               Print help
```

### Example
```
$ rbxcloud group roles -p -g 12345 -a MY_KEY
```

## List Group Memberships
List members of the group. For more info on the `--filter` option, see [Roblox's documentation](https://create.roblox.com/docs/cloud/reference/patterns#list-group-memberships).
```
Usage: rbxcloud.exe group memberships [OPTIONS] --group-id <GROUP_ID> --api-key <API_KEY>

Options:
  -a, --api-key <API_KEY>                  Roblox Open Cloud API Key [env: RBXCLOUD_API_KEY=]
  -g, --group-id <GROUP_ID>                Group ID
  -p, --pretty                             Pretty-print the JSON response
  -m, --max-page-size <MAX_PAGE_SIZE>      Max items returned per page
  -f, --filter <FILTER>                    Filter
  -n, --next-page-token <NEXT_PAGE_TOKEN>  Next page token
  -h, --help                               Print help
```

### Example
```
$ rbxcloud group memberships -p -g 12345 -a MY_KEY
```
