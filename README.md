# Roblox Open Cloud CLI
:warning: Under development :warning:

The `rbxcloud` CLI lets developers easily communicate with the Roblox Open Cloud APIs.

Common use-cases:
- CD pipelines
- Debugging DataStores
- _TODO: More use-cases_

## Install
### Foreman
Add `rbxcloud` under the `[tools]` section of your `foreman.toml` file:
```toml
rbxcloud = { source = "Sleitnick/rbx-cloud-cli", version = "0.1.0-alpha.2" }
```

## Experience

### Publish
```sh
# Long
$ rbxcloud experience publish --filename <FILENAME> --place-id <PLACE_ID> --universe-id <UNIVERSE_ID> --version-type <VERSION_TYPE> --api-key <API_KEY>

# Short
$ rbxcloud experience publish -f <FILENAME> -p <PLACE_ID> -u <UNIVERSE_ID> -v <VERSION_TYPE> -a <API_KEY>

# Example
$ rbxcloud experience publish -f myplace.rbxl -p 123456789 -u 987654321 -v published -a ABCDEFG
```

## Messaging

### Publish
```sh
# Long
$ rbxcloud messaging publish --topic <TOPIC> --message <MESSAGE> --universe-id <UNIVERSE_ID> --api-key <API_KEY>

# Short
$ rbxcloud messaging publish -t <TOPIC> -m <MESSAGE> -u <UNIVERSE_ID> -a <API_KEY>

# Example
$ rbxcloud messaging publish -t MyTopic -m "Hello world!" -u 987654321 -a ABCDEFG
```
