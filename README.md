### How to build

```
cargo build
```

### How to run the tests
```
cargo test
```

### How to parse a yaml file

```
$ cargo -q run /etc/netplan/02-bridge.yaml
Parsing /etc/netplan/02-bridge.yaml
File /etc/netplan/02-bridge.yaml is valid
```

### Examples of errors it can catch

Property not supported

```
$ cargo -q run tests/bad_ethernet.yaml
Parsing tests/bad_ethernet.yaml
Validation failed for file tests/bad_ethernet.yaml
Error: Unexpected keyword /network/ethernets/eth0/weird-property
```
```
$ cargo -q run tests/bad_vlan.yaml
Parsing tests/bad_vlan.yaml
Validation failed for file tests/bad_vlan.yaml
Error: Unexpected keyword /network/vlans/vlan200/lynk
```

Duplications
```
$ cargo -q run tests/duplicate_addresses.yaml
Parsing tests/duplicate_addresses.yaml
Validation failed for file tests/duplicate_addresses.yaml
Error: Duplicate item /network/ethernets/eth0/addresses/["192.168.0.1/24","192.168.0.1/24",{"10.0.0.1/24":{"label":"aaaaaaaaaaaaaaa"}}]
```

Value is not in the list of supported values
```
$ cargo -q run tests/invalid_value.yaml
Parsing tests/invalid_value.yaml
Validation failed for file tests/invalid_value.yaml
Error: Unexpected value /network/renderer: "AssistantToTheRegionalNetworkManager"
```

Invalid values
```
$ cargo -q run tests/bad_boolean.yaml
Parsing tests/bad_boolean.yaml
Validation failed for file tests/bad_boolean.yaml
Error: Unexpected value /network/vlans/vlan200/dhcp4: "nothanks"
```

Value out of range

```
$ cargo -q run tests/out_of_range.yaml
Parsing tests/out_of_range.yaml
Validation failed for file tests/out_of_range.yaml
Error: Unexpected value /network/version: 42
```

Doesn't match the pattern

```
$ cargo -q run tests/bad_mac_address.yaml
Parsing tests/bad_mac_address.yaml
Validation failed for file tests/bad_mac_address.yaml
Error: Unexpected value /network/ethernets/eth0/match/macaddress: "a0:4b:xy:1d:ee:0a"
```