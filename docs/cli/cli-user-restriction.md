# User Restriction API

## Get
Get user restriction information.
```
Usage: rbxcloud user-restriction get [OPTIONS] --universe-id <UNIVERSE_ID> --user-id <USER_ID> --api-key <API_KEY>

Options:
  -u, --universe-id <UNIVERSE_ID>  Universe ID
  -U, --user-id <USER_ID>          User ID
  -P, --place-id <PLACE_ID>        Place ID
  -p, --pretty                     Pretty-print the JSON response
  -a, --api-key <API_KEY>          Roblox Open Cloud API Key [env: RBXCLOUD_API_KEY=]
  -h, --help                       Print help
```

## Update
Update user restriction information.
```
Usage: rbxcloud user-restriction update [OPTIONS] --universe-id <UNIVERSE_ID> --user-id <USER_ID> --api-key <API_KEY>

Options:
  -u, --universe-id <UNIVERSE_ID>        Universe ID
  -U, --user-id <USER_ID>                User ID
  -P, --place-id <PLACE_ID>              Place ID
  -A, --active <ACTIVE>                  Restriction active [possible values: true, false]
  -d, --duration <DURATION>              Restriction duration (seconds)
  -r, --private-reason <PRIVATE_REASON>  Private reason
  -D, --display-reason <DISPLAY_REASON>  Display reason
  -e, --exclude-alts <EXCLUDE_ALTS>      Exclude alternate accounts [possible values: true, false]
  -p, --pretty                           Pretty-print the JSON response
  -a, --api-key <API_KEY>                Roblox Open Cloud API Key [env: RBXCLOUD_API_KEY=]
  -h, --help                             Print help
```

## List
List user restrictions.
```
Usage: rbxcloud user-restriction list [OPTIONS] --universe-id <UNIVERSE_ID> --api-key <API_KEY>

Options:
  -u, --universe-id <UNIVERSE_ID>  Universe ID
  -P, --place-id <PLACE_ID>        Place ID
  -s, --page-size <PAGE_SIZE>      Max page size
  -t, --token <TOKEN>              Next page token
  -f, --filter <FILTER>            Filter
  -p, --pretty                     Pretty-print the JSON response
  -a, --api-key <API_KEY>          Roblox Open Cloud API Key [env: RBXCLOUD_API_KEY=]
  -h, --help                       Print help
```

## Logs
List user restriction logs.
```
Usage: rbxcloud user-restriction logs [OPTIONS] --universe-id <UNIVERSE_ID> --api-key <API_KEY>

Options:
  -u, --universe-id <UNIVERSE_ID>  Universe ID
  -P, --place-id <PLACE_ID>        Place ID
  -s, --page-size <PAGE_SIZE>      Max page size
  -t, --token <TOKEN>              Next page token
  -f, --filter <FILTER>            Filter
  -p, --pretty                     Pretty-print the JSON response
  -a, --api-key <API_KEY>          Roblox Open Cloud API Key [env: RBXCLOUD_API_KEY=]
  -h, --help                       Print help
```
