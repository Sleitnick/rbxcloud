# Luau Execution API

# Execute Task
Executes Luau code on Roblox.
```
Usage: rbxcloud luau execute [OPTIONS] --universe-id <UNIVERSE_ID> --place-id <PLACE_ID> --api-key <API_KEY>

Options:
  -u, --universe-id <UNIVERSE_ID>  Universe ID of the experience
  -i, --place-id <PLACE_ID>        Place ID of the experience
  -r, --version-id <VERSION_ID>    Version ID of the experience
  -s, --script <SCRIPT>            Script source code
  -f, --filepath <FILEPATH>        Script source code file
  -t, --timeout <TIMEOUT>          Execution timeout
  -p, --pretty                     Pretty-print the JSON response
  -a, --api-key <API_KEY>          Roblox Open Cloud API Key [env: RBXCLOUD_API_KEY=]
  -h, --help                       Print help
```

## Get Task Information
Gets information on a previously executed task.

```
Usage: rbxcloud luau get-task [OPTIONS] --universe-id <UNIVERSE_ID> --place-id <PLACE_ID> --session-id <SESSION_ID> --task-id <TASK_ID> --api-key <API_KEY>

Options:
  -u, --universe-id <UNIVERSE_ID>  Universe ID of the experience
  -i, --place-id <PLACE_ID>        Place ID of the experience
  -r, --version-id <VERSION_ID>    Version ID of the experience
  -s, --session-id <SESSION_ID>    Luau execution session ID
  -t, --task-id <TASK_ID>          Luau execution task ID
  -p, --pretty                     Pretty-print the JSON response
  -a, --api-key <API_KEY>          Roblox Open Cloud API Key [env: RBXCLOUD_API_KEY=]
  -h, --help                       Print help
```

## Get Task Logs
Retrieves logs on a previously executed task.
```
Usage: rbxcloud luau get-logs [OPTIONS] --universe-id <UNIVERSE_ID> --place-id <PLACE_ID> --session-id <SESSION_ID> --task-id <TASK_ID> --api-key <API_KEY>

Options:
  -u, --universe-id <UNIVERSE_ID>      Universe ID of the experience
  -i, --place-id <PLACE_ID>            Place ID of the experience
  -r, --version-id <VERSION_ID>        Version ID of the experience
  -s, --session-id <SESSION_ID>        Luau execution session ID
  -t, --task-id <TASK_ID>              Luau execution task ID
  -m, --max-page-size <MAX_PAGE_SIZE>  Max page size
  -n, --page-token <PAGE_TOKEN>        Next page token
  -w, --view <VIEW>                    Log view type [possible values: flat, structured]
  -p, --pretty                         Pretty-print the JSON response
  -a, --api-key <API_KEY>              Roblox Open Cloud API Key [env: RBXCLOUD_API_KEY=]
  -h, --help                           Print help
```
