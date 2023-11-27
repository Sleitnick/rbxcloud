# Experience API

## Publish Place
Publish a place to a Roblox experience. This takes an `*.rbxl` file and publishes it to a specific Place ID.
```
USAGE:
    rbxcloud experience publish --filename <FILENAME> --place-id <PLACE_ID> --universe-id <UNIVERSE_ID> --version-type <VERSION_TYPE> --api-key <API_KEY>

OPTIONS:
    -a, --api-key <API_KEY>              Roblox Open Cloud API Key [env: RBXCLOUD_API_KEY=]
    -f, --filename <FILENAME>            Filename (full or relative) of the RBXL file
    -h, --help                           Print help information
    -p, --place-id <PLACE_ID>            Place ID of the experience
    -u, --universe-id <UNIVERSE_ID>      Universe ID of the experience
    -t, --version-type <VERSION_TYPE>    Version type [possible values: saved, published]
```

### Example
```
$ rbxcloud experience publish -f myplace.rbxl -t published -p 12345 -u 98765 -a MY_KEY
```

### GitHub Action
A common practice is to create a CD pipeline to deploy a place automatically. This can be done with GitHub actions. Below is an example GitHub Action workflow. The example assumes that Aftman is being used and that `rbxcloud` is listed within the `aftman.toml` file.

```yaml
# Deploy any time code is pushed to the 'main' branch
name: Deploy
on:
  push:
    branches:
      - main

jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3

      - name: Install Aftman
        uses: ok-nick/setup-aftman@v0

      - name: Publish
        shell: bash
        env:
          UID: 123456789              # Universe ID
          PID: 123456789              # Place ID
          API_KEY: ${{ secrets.key }} # API Key (keep this in your GitHub Repository Secrets)
          FILE: my_place.rbxl         # Roblox place file (e.g. might have a step before this to build the file with Rojo)

        run: rbxcloud experience publish -a "$API_KEY" -u "$UID" -p "$PID" -t published -f "$FILE"
```
