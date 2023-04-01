# OrderedDataStore API

## List Entries
```
USAGE:
    rbxcloud ordered-datastore list [OPTIONS] --datastore-name <DATASTORE_NAME> --universe-id <UNIVERSE_ID> --api-key <API_KEY>

OPTIONS:
    -a, --api-key <API_KEY>                  Roblox Open Cloud API Key [env: RBXCLOUD_API_KEY=]
        --datastore-name <DATASTORE_NAME>    DataStore name
    -f, --filter <FILTER>                    A range of qualifying values of entries to return
    -h, --help                               Print help information
    -m, --max-page-size <MAX_PAGE_SIZE>      Maximum number of items to return per page
    -o, --order-by <ORDER_BY>                The enumeration direction (Use 'desc' for descending)
    -p, --page-token <PAGE_TOKEN>            Cursor for the next set of data
    -s, --scope <SCOPE>                      DataStore scope
    -u, --universe-id <UNIVERSE_ID>          Universe ID of the experience
```

## Get Entry
```
USAGE:
    rbxcloud ordered-datastore get [OPTIONS] --datastore-name <DATASTORE_NAME> --id <ID> --universe-id <UNIVERSE_ID> --api-key <API_KEY>

OPTIONS:
    -a, --api-key <API_KEY>                  Roblox Open Cloud API Key [env: RBXCLOUD_API_KEY=]
        --datastore-name <DATASTORE_NAME>    DataStore name
    -h, --help                               Print help information
    -i, --id <ID>                            The ID of the entry
    -s, --scope <SCOPE>                      DataStore scope
    -u, --universe-id <UNIVERSE_ID>          Universe ID of the experience
```

## Create Entry
```
USAGE:
    rbxcloud ordered-datastore create [OPTIONS] --datastore-name <DATASTORE_NAME> --id <ID> --value <VALUE> --universe-id <UNIVERSE_ID> --api-key <API_KEY>

OPTIONS:
    -a, --api-key <API_KEY>                  Roblox Open Cloud API Key [env: RBXCLOUD_API_KEY=]
        --datastore-name <DATASTORE_NAME>    DataStore name
    -h, --help                               Print help information
    -i, --id <ID>                            The ID of the entry
    -s, --scope <SCOPE>                      DataStore scope
    -u, --universe-id <UNIVERSE_ID>          Universe ID of the experience
    -v, --value <VALUE>                      The value of the entry
```

## Update Entry
```
USAGE:
    rbxcloud ordered-datastore update [OPTIONS] --datastore-name <DATASTORE_NAME> --id <ID> --value <VALUE> --universe-id <UNIVERSE_ID> --api-key <API_KEY>

OPTIONS:
    -a, --api-key <API_KEY>                  Roblox Open Cloud API Key [env: RBXCLOUD_API_KEY=]
        --datastore-name <DATASTORE_NAME>    DataStore name
    -h, --help                               Print help information
    -i, --id <ID>                            The ID of the entry
    -m, --allow-missing <ALLOW_MISSING>      Create if missing [possible values: true, false]
    -s, --scope <SCOPE>                      DataStore scope
    -u, --universe-id <UNIVERSE_ID>          Universe ID of the experience
    -v, --value <VALUE>                      The value of the entry
```

## Increment Entry
```
USAGE:
    rbxcloud ordered-datastore increment [OPTIONS] --datastore-name <DATASTORE_NAME> --id <ID> --increment <INCREMENT> --universe-id <UNIVERSE_ID> --api-key <API_KEY>

OPTIONS:
    -a, --api-key <API_KEY>                  Roblox Open Cloud API Key [env: RBXCLOUD_API_KEY=]
        --datastore-name <DATASTORE_NAME>    DataStore name
    -h, --help                               Print help information
    -i, --id <ID>                            The ID of the entry
    -n, --increment <INCREMENT>              The incremented value of the entry
    -s, --scope <SCOPE>                      DataStore scope
    -u, --universe-id <UNIVERSE_ID>          Universe ID of the experience
```

## Delete Entry
```
USAGE:
    rbxcloud ordered-datastore delete [OPTIONS] --datastore-name <DATASTORE_NAME> --id <ID> --universe-id <UNIVERSE_ID> --api-key <API_KEY>

OPTIONS:
    -a, --api-key <API_KEY>                  Roblox Open Cloud API Key [env: RBXCLOUD_API_KEY=]
        --datastore-name <DATASTORE_NAME>    DataStore name
    -h, --help                               Print help information
    -i, --id <ID>                            The ID of the entry
    -s, --scope <SCOPE>                      DataStore scope
    -u, --universe-id <UNIVERSE_ID>          Universe ID of the experience
```