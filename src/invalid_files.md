
# examples/mappings/markdown/Microsoft/Windows Events/4673/4673_0.json
- class_uid is set to 1010, which doesn't seem to be a valid uid

# examples/mappings/markdown/Microsoft/Windows Events/4663/4663_0.json
- class_uid is set to 1010, which doesn't seem to be a valid uid

# examples/mappings/markdown/Microsoft/Windows Events/4661/4661.json
- class_uid is set to 1010, which doesn't seem to be a valid uid

# examples/mappings/markdown/Microsoft/Windows Events/4624/4624_0.json
- There's an unknown field "account_type" in the at actor.user struct set to "Windows Account", which is not present in the protobuf or spec.
- It might be meant to be the similar field "type"

# examples/mappings/markdown/Microsoft/Windows Events/4625/4625_0.json
- There's an unknown field "account_type" in the at actor.user struct set to "Windows Account", which is not present in the protobuf or spec.
- It might be meant to be the similar field "type"

# examples/mappings/markdown/Microsoft/Windows Events/4689/4689_0.json
- There's an unknown field "account_type" in the at actor.user struct set to "Windows Account", which is not present in the protobuf or spec.
- It might be meant to be the similar field "type"

# examples/mappings/markdown/Microsoft/Windows Events/4688/4688_0.json
- There's an unknown field "account_type" in the at actor.user struct set to "Windows Account", which is not present in the protobuf or spec.
- It might be meant to be the similar field "type"

# examples/mappings/markdown/SSC/samples/malware_infection_detected_sample.json
- There's four JSON-encoded Messages in this file, each with the same issue
- "confidence" field is set to a number, instead of a String as specified in the spec

# examples/mappings/markdown/CERT-NetSA at CMU-SEI/SiLK Network Flow Data/samples/SampleOCSF_FCCX.json
- The "Duration" field is set to a Float, instead of a Long as specified in the spec

# examples/mappings/markdown/Zeek/dns_log/samples/dns_log.ocsf
- The "flags" field in the first entry of the "answers" list is an int array, instead of it should be a String array as specified in the spec.
- It might be meant to be the the similar field "flag_ids" which is an int array.

# examples/mappings/markdown/Falco/samples/falco.ocsf
- There's an unknown field "classname". This field is not present in the protobuf or spec, but there's a similar field "class_name"

# examples/mappings/markdown/IBM/QRadar SIEM/samples/offense.ocsf
- the field "original_time" in the metadata struct is set to a Long, instead of a String as specified in the spec

# examples/mappings/bloblang/AWS/route53/v1.1.0/samples/OCSF_dns_activity.json
- File is empty

# examples/mappings/markdown/Zeek/ssl_log/samples/ssl_log.ocsf
- uid is 4003, so it should be a "DNS Activity" message, but there's a bunch of unrecognized fields.
    - for example, "certificate", "issuer", and "cipher" are defined but not present in the spec


# examples/mappings/markdown/Zeek/conn_log/samples/conn_log.ocsf
- uid is 4001, so it should be a "Network Activity" message, but there's a bunch of unrecognized fields.
    - for example, "bytes_in", "packets_in", and "resp_bytes" are defined but not present in the spec
