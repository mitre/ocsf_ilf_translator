# ILF OCSF Protobuf

ILF OCSF Protobuf translates OCSF json logs to ILF logs using the OCSF protobuf schema. 

This is designed to operate on one OCSF log file at a time, limiting its capability for use in production.

## Version
0.2.1

## Usage

Run `cargo build` to build the binary file.

```
USAGE:
    json_to_ilf --ocsf-file-path <ocsf-file-path>
```

Pass the path to an ILF file as input. The tool will print errors if it fails to parse the OCSF json

```
json_to_ilf --ilf-file /path/to/file.ilf
```

## Updating OCSF Schema

To update the OCSF schema, replace the `ocsf.proto` file located at `src/protos`