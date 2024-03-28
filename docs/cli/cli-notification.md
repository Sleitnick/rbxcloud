# Notification API

## Send Notification
Send a notification to a user.
```
Usage: rbxcloud notification send [OPTIONS] --universe-id <UNIVERSE_ID> --user-id <USER_ID> --payload <PAYLOAD> --api-key <API_KEY>

Options:
  -u, --universe-id <UNIVERSE_ID>  Universe ID
  -U, --user-id <USER_ID>          User ID
  -P, --payload <PAYLOAD>          Payload
  -p, --pretty                     Pretty-print the JSON response
  -a, --api-key <API_KEY>          Roblox Open Cloud API Key [env: RBXCLOUD_API_KEY=]
  -h, --help                       Print help
```

### Example
For an example payload, see the [Roblox documentation](https://create.roblox.com/docs/cloud/reference/UserNotification#Create-User-Notification).
```
$ rbxcloud notification send -p -u 12345 -U 6789 -P PAYLOAD_JSON -a MY_KEY
```
