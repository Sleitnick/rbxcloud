# Assets API

## Get
Get information on a given asset.
```
Usage: rbxcloud.exe assets get [OPTIONS] --asset-id <ASSET_ID> --api-key <API_KEY>

Options:
  -i, --asset-id <ASSET_ID>    Asset ID
  -m, --read-mask <READ_MASK>
  -a, --api-key <API_KEY>      Roblox Open Cloud API Key [env: RBXCLOUD_API_KEY=]
  -h, --help                   Print help
```

## Create
Create a new asset.
```
Usage: rbxcloud.exe assets create [OPTIONS] --display-name <DISPLAY_NAME> --description <DESCRIPTION> --creator-id <CREATOR_ID> --creator-type <CREATOR_TYPE> --filepath <FILEPATH> --api-key <API_KEY>

Options:
  -t, --asset-type <ASSET_TYPE>
          Asset type [possible values: audio-mp3, audio-ogg, audio-flac, audio-wav, decal-png, decal-jpeg, decal-bmp, decal-tga, model-fbx]
  -n, --display-name <DISPLAY_NAME>
          Display name
  -d, --description <DESCRIPTION>
          Description
  -e, --expected-price <EXPECTED_PRICE>
          Expected Robux price
  -i, --creator-id <CREATOR_ID>
          Creator ID
  -c, --creator-type <CREATOR_TYPE>
          Creator type [possible values: user, group]
  -f, --filepath <FILEPATH>
          File (full or relative path)
  -a, --api-key <API_KEY>
          Roblox Open Cloud API Key [env: RBXCLOUD_API_KEY=]
  -h, --help
          Print help
```

## Update
Update an asset.
```
Usage: rbxcloud.exe assets update [OPTIONS] --asset-id <ASSET_ID> --filepath <FILEPATH> --api-key <API_KEY>

Options:
  -t, --asset-type <ASSET_TYPE>  Asset type [possible values: audio-mp3, audio-ogg, audio-flac, audio-wav, decal-png, decal-jpeg, decal-bmp, decal-tga, model-fbx]
  -i, --asset-id <ASSET_ID>      Asset ID
  -f, --filepath <FILEPATH>      File (full or relative path)
  -a, --api-key <API_KEY>        Roblox Open Cloud API Key [env: RBXCLOUD_API_KEY=]
  -h, --help                     Print help
```

## Archive
Archive an asset.
```
Usage: rbxcloud.exe assets archive --asset-id <ASSET_ID> --api-key <API_KEY>

Options:
  -i, --asset-id <ASSET_ID>  Asset ID
  -a, --api-key <API_KEY>    Roblox Open Cloud API Key [env: RBXCLOUD_API_KEY=]
  -h, --help                 Print help
```

## Restore
Restore an archived asset.
```
Usage: rbxcloud.exe assets restore --asset-id <ASSET_ID> --api-key <API_KEY>

Options:
  -i, --asset-id <ASSET_ID>  Asset ID
  -a, --api-key <API_KEY>    Roblox Open Cloud API Key [env: RBXCLOUD_API_KEY=]
  -h, --help                 Print help
```

## Get Operation
Get information on an asset operation.
```
Usage: rbxcloud.exe assets get-operation --operation-id <OPERATION_ID> --api-key <API_KEY>

Options:
  -i, --operation-id <OPERATION_ID>  Operation ID
  -a, --api-key <API_KEY>            Roblox Open Cloud API Key [env: RBXCLOUD_API_KEY=]
  -h, --help                         Print help

```
