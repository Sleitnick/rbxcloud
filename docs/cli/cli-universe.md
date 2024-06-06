# Universe API

## Getting Universe Info
```
Usage: rbxcloud universe get [OPTIONS] --universe-id <UNIVERSE_ID> --api-key <API_KEY>

Options:
  -u, --universe-id <UNIVERSE_ID>  Universe ID
  -p, --pretty                     Pretty-print the JSON response
  -a, --api-key <API_KEY>          Roblox Open Cloud API Key [env: RBXCLOUD_API_KEY=]
  -h, --help                       Print help
```

### Example
```
$ rbxcloud universe get -p -u 12345 -a MY_KEY
```

## Updating Name
```
Usage: rbxcloud universe update-name [OPTIONS] --universe-id <UNIVERSE_ID> --name <NAME> --api-key <API_KEY>

Options:
  -u, --universe-id <UNIVERSE_ID>  Universe ID
  -n, --name <NAME>                New Universe name
  -p, --pretty                     Pretty-print the JSON response
  -a, --api-key <API_KEY>          Roblox Open Cloud API Key [env: RBXCLOUD_API_KEY=]
  -h, --help                       Print help
```

### Example
```
$ rbxcloud universe update-name -n "Experience Name" -p -u 12345 -a MY_KEY
```

## Updating Description
```
Usage: rbxcloud universe update-description [OPTIONS] --universe-id <UNIVERSE_ID> --description <DESCRIPTION> --api-key <API_KEY>

Options:
  -u, --universe-id <UNIVERSE_ID>  Universe ID
  -d, --description <DESCRIPTION>  New Universe description
  -p, --pretty                     Pretty-print the JSON response
  -a, --api-key <API_KEY>          Roblox Open Cloud API Key [env: RBXCLOUD_API_KEY=]
  -h, --help                       Print help
```

### Example
```
$ rbxcloud universe update-description -n "Experience description here." -p -u 12345 -a MY_KEY
```

## Restarting Servers
```
Usage: rbxcloud universe restart --universe-id <UNIVERSE_ID> --api-key <API_KEY>

Options:
  -u, --universe-id <UNIVERSE_ID>  Universe ID
  -a, --api-key <API_KEY>          Roblox Open Cloud API Key [env: RBXCLOUD_API_KEY=]
  -h, --help                       Print help
```

### Example
```
$ rbxcloud universe restart -u 12345 -a MY_KEY
```
