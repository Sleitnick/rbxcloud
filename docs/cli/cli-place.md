# Universe API

## Getting Place Info
```
Usage: rbxcloud place get [OPTIONS] --universe-id <UNIVERSE_ID> --place-id <PLACE_ID> --api-key <API_KEY>

Options:
  -u, --universe-id <UNIVERSE_ID>  Universe ID
  -p, --place-id <PLACE_ID>        Place ID
      --pretty                     Pretty-print the JSON response
  -a, --api-key <API_KEY>          Roblox Open Cloud API Key [env: RBXCLOUD_API_KEY=]
  -h, --help                       Print help
```

### Example
```
$ rbxcloud place get -u 12345 -p 67890 -a MY_KEY
```

## Updating Name
```
Usage: rbxcloud place update-name [OPTIONS] --universe-id <UNIVERSE_ID> --place-id <PLACE_ID> --name <NAME> --api-key <API_KEY>

Options:
  -u, --universe-id <UNIVERSE_ID>  Universe ID
  -p, --place-id <PLACE_ID>        Place ID
  -n, --name <NAME>                New Place name
      --pretty                     Pretty-print the JSON response
  -a, --api-key <API_KEY>          Roblox Open Cloud API Key [env: RBXCLOUD_API_KEY=]
  -h, --help                       Print help
```

### Example
```
$ rbxcloud place update-name -n "Place Name" -u 12345 -p 67890 -a MY_KEY
```

## Updating Description
```
Usage: rbxcloud place update-description [OPTIONS] --universe-id <UNIVERSE_ID> --place-id <PLACE_ID> --description <DESCRIPTION> --api-key <API_KEY>

Options:
  -u, --universe-id <UNIVERSE_ID>  Universe ID
  -p, --place-id <PLACE_ID>        Place ID
  -d, --description <DESCRIPTION>  New Place description
      --pretty                     Pretty-print the JSON response
  -a, --api-key <API_KEY>          Roblox Open Cloud API Key [env: RBXCLOUD_API_KEY=]
  -h, --help                       Print help
```

### Example
```
$ rbxcloud universe update-description -n "Place description here." -u 12345 -p 67890 -a MY_KEY
```
