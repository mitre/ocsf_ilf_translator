== Open Questions ==
====================
# eks.ocsf
6003, EOF when parsing datetime?
These fields exist in the protobuf but not the Schema. Could be upset that the time is a string, not a struct?

# waf.ocsf
4002, EOF when parsing datetime?
These fields exist in the protobuf but not the Schema. Could be upset that the time is a string, not a struct?

### TODO:
Looks like i've gotta fix/parse the timestamps myself:https://github.com/protocolbuffers/protobuf/blob/main/src/google/protobuf/timestamp.proto

# waf.ocsf (has other error)
# ocsf_eks.json
# ocsf_authentication.json (v1.1.0 and 1.0.0)
# ocsf_api_activity.json (v1.1.0 and 1.0.0)
# ocsf_account_change.json (v1.1.0 and 1.0.0)
All have null fields where a string literal is expected

== JSON doesn't match Spec == 
=============================

# parse_4673_0
# parse_4661
# parse_4663_0
All have class_uid of 1010, which is not in my list of uids. 

# 4689_0.json:
# 4688_0.json:
Type 1007, ProcessActivity
actor.user.account_type -> "Windows Account"
Present in JSON, Not present in protobuf or Documentation
Though there is a similar optional field, "type"
https://schema.ocsf.io/1.3.0/objects/user

# 4624_0.json:
# 4625_0.json:
Type 3002, Authentication
actor.user.account_type -> "Windows Account"
Present in JSON, Not present in protobuf or Documentation
Though there is a similar optional field, "type"
https://schema.ocsf.io/1.3.0/objects/user

# parse_malware_infection_detected_sample [all]
Type 2001, Security Finding
"confidence" field is a number instead of a string
String in the Protobuf and Spec, not the JSON.

# parse_sample_coff_fccx
Type 4001, Network Activity
Error: "duration": 0.006,
Duration should be a Long (i32 in protobuf) not a Float

# OCSF_dns_activity (v1.0.0)
"answers" is a single DNS Answer object, not a list

# dns_log.ocsf
4003, Expecting string literal at 1:63: 's\":[3,4],\"'"
answers.0.flags is an int array, it should be a string array.
There is a similar field flag_ids which is an int array

# ssl_log.ocsf
4001, unknown field 'certificate'
In JSON, Not in spec.
Looks like there's multiple extra keys

# conn_log.ocsf
4001 Unknown field name: 'bytes_in'
In JSON, Not in spec.
Looks like there's multiple extra keys

# falco.ocsf
2001 Unknown field name 'classname', should be "class_name"

# Security Hub.ocsf
2001, 'account_uid'
No field in the spec, should be an Account object

# offense.ocsf
2001, "metadata"."original_time" should be a string

== Misc issues ==
=================

# 1_0_0_account_change.ocsf
# 1_0_0_authentication.ocsf
# 1_0_0_generic_api_activity.ocsf 
# 1_0_0_vpcflowlog.ocsf
# 1_0_0_route53.ocsf
These fail, but there's a v1.1.0 version that passes.

# OCSF_dns_activity (v1.1.0)
File is empty