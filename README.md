# MITRE Intermediate Log Format (ILF) Open Cybersecurity Schema Format (OCSF) Protobuf

ILF to [OCSF](https://github.com/ocsf) Protobuf translates OCSF json logs to ILF logs using the OCSF protobuf schema. 

*Important Note*: This is designed to operate on one OCSF log file at a time to test the mapping from OCSF events to ILF. This is not designed to be used in production environments.

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

## License

This software is licensed under the Apache 2.0 license.

## Public Release

> [!NOTE]
> Approved for Public Release; Distribution Unlimited. Public Release Case
> Number 24-3961.