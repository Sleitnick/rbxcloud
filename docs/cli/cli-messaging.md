# Messaging API

## Publish Message
Publish a message to a Roblox experience. This takes a message and a topic and publishes the message. Experiences can use the [`MessagingService`](https://create.roblox.com/docs/reference/engine/classes/MessagingService) to subscribe to a topic and listen for messages.
```
USAGE:
    rbxcloud messaging publish --topic <TOPIC> --message <MESSAGE> --universe-id <UNIVERSE_ID> --api-key <API_KEY>

OPTIONS:
    -a, --api-key <API_KEY>            Roblox Open Cloud API Key [env: RBXCLOUD_API_KEY=]
    -h, --help                         Print help information
    -m, --message <MESSAGE>            Message to send
    -t, --topic <TOPIC>                Message topic
```

Example:
```
$ rbxcloud messaging publish -t Hello -m "Hello world!" -p 12345 -u 98765 -a MY_KEY
```
```lua
MessagingService:SubscribeAsync("Hello", function(message)
	print(message)
	--> {message: "Hello world!"}
end)
```