# User CLI

## Getting User Information
```
Usage: rbxcloud user get [OPTIONS] --user-id <USER_ID> --api-key <API_KEY>

Options:
  -u, --user-id <USER_ID>  User ID
  -p, --pretty             Pretty-print the JSON response
  -a, --api-key <API_KEY>  Roblox Open Cloud API Key [env: RBXCLOUD_API_KEY=]
  -h, --help               Print help
```

### Example
```
$ rbxcloud user get -p -u 12345 -a MY_KEY
```

## Generating User Thumbnail
```
Usage: rbxcloud user thumbnail [OPTIONS] --user-id <USER_ID> --api-key <API_KEY>

Options:
  -u, --user-id <USER_ID>  User ID
  -s, --size <SIZE>        Thumbnail size [possible values: size48x48, size50x50, size60x60, size75x75, size100x100, size110x110, size150x150, size180x180, size352x352, size420x420, size720x720]
  -f, --format <FORMAT>    Thumbnail format [possible values: png, jpeg]
  -S, --shape <SHAPE>      Thumbnail shape [possible values: round, square]
  -p, --pretty             Pretty-print the JSON response
  -a, --api-key <API_KEY>  Roblox Open Cloud API Key [env: RBXCLOUD_API_KEY=]
  -h, --help               Print help
```

### Example
```
$ rbxcloud user thumbnail -p -u 12345 -s size100x100 -f png -S round -a MY_KEY
```
