# Roblox Open Cloud CLI
:warning: Under Development :warning:

The `rbxcloud` CLI lets developers easily communicate with the Roblox Open Cloud APIs.

Common use-cases:
- CD pipelines
- Debugging DataStores
- _TODO: More use-cases_

## Install
### Foreman
Add `rbxcloud` under the `[tools]` section of your `foreman.toml` file:
```toml
rbxcloud = { github = "Sleitnick/rbx-cloud-cli", version = "0.1.0-alpha.2" }
```

## Experience

### Publish
```
USAGE:
    rbxcloud experience publish --filename <FILENAME> --place-id <PLACE_ID> --universe-id <UNIVERSE_ID> --version-type <VERSION_TYPE> --api-key <API_KEY>

OPTIONS:
    -a, --api-key <API_KEY>              Roblox Open Cloud API Key
    -f, --filename <FILENAME>            Filename (full or relative) of the RBXL file
    -h, --help                           Print help information
    -p, --place-id <PLACE_ID>            Place ID of the experience
    -u, --universe-id <UNIVERSE_ID>      Universe ID of the experience
    -v, --version-type <VERSION_TYPE>    Version type [possible values: saved, published]
```

## Messaging

### Publish
```
USAGE:
    rbxcloud messaging publish --topic <TOPIC> --message <MESSAGE> --universe-id <UNIVERSE_ID> --api-key <API_KEY>

OPTIONS:
    -a, --api-key <API_KEY>            Roblox Open Cloud API Key
    -h, --help                         Print help information
    -m, --message <MESSAGE>            Message to send
    -t, --topic <TOPIC>                Message topic
```

## DataStore
:warning: Under Development :warning:

### List Stores
```
USAGE:
    rbxcloud datastore list-stores [OPTIONS] --universe-id <UNIVERSE_ID> --api-key <API_KEY>

OPTIONS:
    -a, --api-key <API_KEY>            Roblox Open Cloud API Key
    -c, --cursor <CURSOR>              Cursor for the next set of data
    -h, --help                         Print help information
    -l, --limit <LIMIT>                Maximum number of items to return
    -p, --prefix <PREFIX>              Return only DataStores with this prefix
    -u, --universe-id <UNIVERSE_ID>    Universe ID of the experience
```

### List Keys
```
USAGE:
    rbxcloud datastore list [OPTIONS] --datastore-name <DATASTORE_NAME> --universe-id <UNIVERSE_ID> --api-key <API_KEY>

OPTIONS:
    -a, --api-key <API_KEY>                  Roblox Open Cloud API Key
    -c, --cursor <CURSOR>                    Cursor for the next set of data
    -d, --datastore-name <DATASTORE_NAME>    DataStore name
    -h, --help                               Print help information
    -l, --limit <LIMIT>                      Maximum number of items to return
    -o, --all-scopes                         If true, return keys from all scopes
    -p, --prefix <PREFIX>                    Return only DataStores with this prefix
    -s, --scope <SCOPE>                      DataStore scope
    -u, --universe-id <UNIVERSE_ID>          Universe ID of the experience
```

### List Versions of Key
```
USAGE:
    rbxcloud datastore list-versions [OPTIONS] --datastore-name <DATASTORE_NAME> --key <KEY> --sort-order <SORT_ORDER> --limit <LIMIT> --universe-id <UNIVERSE_ID> --api-key <API_KEY>

OPTIONS:
    -a, --api-key <API_KEY>                  Roblox Open Cloud API Key
    -c, --cursor <CURSOR>                    Cursor for the next set of data
    -d, --datastore-name <DATASTORE_NAME>    DataStore name
    -e, --end-time <END_TIME>                End time constraint (ISO UTC Datetime)
    -h, --help                               Print help information
    -k, --key <KEY>                          The key of the entry
    -l, --limit <LIMIT>                      Maximum number of items to return
    -o, --sort-order <SORT_ORDER>            Sort order [possible values: ascending, descending]
    -s, --scope <SCOPE>                      DataStore scope
    -t, --start-time <START_TIME>            Start time constraint (ISO UTC Datetime)
    -u, --universe-id <UNIVERSE_ID>          Universe ID of the experience
```

### Get Key
```
USAGE:
    rbxcloud datastore get [OPTIONS] --datastore-name <DATASTORE_NAME> --key <KEY> --universe-id <UNIVERSE_ID> --api-key <API_KEY>

OPTIONS:
    -a, --api-key <API_KEY>                  Roblox Open Cloud API Key
    -d, --datastore-name <DATASTORE_NAME>    DataStore name
    -h, --help                               Print help information
    -k, --key <KEY>                          The key of the entry
    -s, --scope <SCOPE>                      DataStore scope
    -u, --universe-id <UNIVERSE_ID>          Universe ID of the experience
```

### Set Key
```
USAGE:
    rbxcloud datastore set [OPTIONS] --datastore-name <DATASTORE_NAME> --key <KEY> --data <DATA> --universe-id <UNIVERSE_ID> --api-key <API_KEY>

OPTIONS:
    -a, --api-key <API_KEY>                  Roblox Open Cloud API Key
    -d, --datastore-name <DATASTORE_NAME>    DataStore name
    -D, --data <DATA>                        JSON-stringified data (up to 4MB)
    -e, --exclusive-create                   Only create the entry if it does not exist
    -h, --help                               Print help information
    -i, --match-version <MATCH_VERSION>      Only update if the current version matches this
    -k, --key <KEY>                          The key of the entry
    -s, --scope <SCOPE>                      DataStore scope
    -t, --attributes <ATTRIBUTES>            JSON-stringified attributes data
    -u, --universe-id <UNIVERSE_ID>          Universe ID of the experience
    -U, --user-ids <USER_IDS>                Comma-separated list of Roblox user IDs
```

## Increment Key
```
USAGE:
    rbxcloud datastore increment [OPTIONS] --datastore-name <DATASTORE_NAME> --key <KEY> --increment-by <INCREMENT_BY> --universe-id <UNIVERSE_ID> --api-key <API_KEY>

OPTIONS:
    -a, --api-key <API_KEY>                  Roblox Open Cloud API Key
    -d, --datastore-name <DATASTORE_NAME>    DataStore name
    -h, --help                               Print help information
    -i, --increment-by <INCREMENT_BY>        The amount by which the entry should be incremented
    -k, --key <KEY>                          The key of the entry
    -s, --scope <SCOPE>                      DataStore scope
    -t, --attributes <ATTRIBUTES>            JSON-stringified attributes data
    -u, --universe-id <UNIVERSE_ID>          Universe ID of the experience
    -U, --user-ids <USER_IDS>                Comma-separated list of Roblox user IDs
```

## Delete Key
```
USAGE:
    rbxcloud datastore delete [OPTIONS] --datastore-name <DATASTORE_NAME> --key <KEY> --universe-id <UNIVERSE_ID> --api-key <API_KEY>

OPTIONS:
    -a, --api-key <API_KEY>                  Roblox Open Cloud API Key
    -d, --datastore-name <DATASTORE_NAME>    DataStore name
    -h, --help                               Print help information
    -k, --key <KEY>                          The key of the entry
    -s, --scope <SCOPE>                      DataStore scope
    -u, --universe-id <UNIVERSE_ID>          Universe ID of the experience
```

## Get Version of Key
```
USAGE:
    rbxcloud datastore get-version [OPTIONS] --datastore-name <DATASTORE_NAME> --key <KEY> --version-id <VERSION_ID> --universe-id <UNIVERSE_ID> --api-key <API_KEY>

OPTIONS:
    -a, --api-key <API_KEY>                  Roblox Open Cloud API Key
    -d, --datastore-name <DATASTORE_NAME>    DataStore name
    -h, --help                               Print help information
    -i, --version-id <VERSION_ID>            The version of the key
    -k, --key <KEY>                          The key of the entry
    -s, --scope <SCOPE>                      DataStore scope
    -u, --universe-id <UNIVERSE_ID>          Universe ID of the experience
```
