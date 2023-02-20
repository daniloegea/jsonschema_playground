use jsonschema::{Draft, JSONSchema};
use serde_json::value::Value;

// Apparently, inheritance has some limitation in jsonschema.
// When trying to reuse blocks of definitions and expanding the properties
// of objects while keeping the "additionalProperties: false", I realized it doesn't work
// as I expected. The $reference of definitions work like a charm, but it will allow the user
// to insert additional properties in the YAML and we can't allow that.
//
// My workaround is dirty: it will load the base schema, get a pointer to the properties we
// want to expand and inject the common properties used by all the interface types there.

pub fn build_schema() -> Result<JSONSchema, String> {
    let mut schema_data: Value = serde_yaml::from_str(SCHEMA).unwrap();

    let common_properties_patch: Value = serde_yaml::from_str(COMMON_PROPERTIES).unwrap();

    let ethernet_properties = schema_data
        .pointer_mut("/properties/network/properties/ethernets/patternProperties/.*$/properties");
    if let Some(props) = ethernet_properties {
        let properties = props.as_object_mut().unwrap();
        for (k, v) in common_properties_patch.as_object().unwrap() {
            properties.insert(k.clone(), v.clone());
        }
    }

    let vlan_properties = schema_data
        .pointer_mut("/properties/network/properties/vlans/patternProperties/.*$/properties");

    if let Some(props) = vlan_properties {
        let properties = props.as_object_mut().unwrap();
        for (k, v) in common_properties_patch.as_object().unwrap() {
            properties.insert(k.clone(), v.clone());
        }
    }

    let bridge_properties = schema_data
        .pointer_mut("/properties/network/properties/bridges/patternProperties/.*$/properties");

    if let Some(props) = bridge_properties {
        let properties = props.as_object_mut().unwrap();
        for (k, v) in common_properties_patch.as_object().unwrap() {
            properties.insert(k.clone(), v.clone());
        }
    }

    let wifis_properties = schema_data
        .pointer_mut("/properties/network/properties/wifis/patternProperties/.*$/properties");

    if let Some(props) = wifis_properties {
        let properties = props.as_object_mut().unwrap();
        for (k, v) in common_properties_patch.as_object().unwrap() {
            properties.insert(k.clone(), v.clone());
        }
    }

    let bonds_properties = schema_data
        .pointer_mut("/properties/network/properties/bonds/patternProperties/.*$/properties");

    if let Some(props) = bonds_properties {
        let properties = props.as_object_mut().unwrap();
        for (k, v) in common_properties_patch.as_object().unwrap() {
            properties.insert(k.clone(), v.clone());
        }
    }

    let tunnels_properties = schema_data
        .pointer_mut("/properties/network/properties/tunnels/patternProperties/.*$/properties");

    if let Some(props) = tunnels_properties {
        let properties = props.as_object_mut().unwrap();
        for (k, v) in common_properties_patch.as_object().unwrap() {
            properties.insert(k.clone(), v.clone());
        }
    }

    let vrfs_properties = schema_data
        .pointer_mut("/properties/network/properties/vrfs/patternProperties/.*$/properties");

    if let Some(props) = vrfs_properties {
        let properties = props.as_object_mut().unwrap();
        for (k, v) in common_properties_patch.as_object().unwrap() {
            properties.insert(k.clone(), v.clone());
        }
    }

    let schema_result = JSONSchema::options()
        .with_draft(Draft::Draft7)
        .compile(&schema_data);

    if let Err(ref err) = schema_result {
        let error = format!("{}, {:?}, {}", err.schema_path, err.kind, err.instance_path);
        return Err(error);
    }

    return Ok(schema_result.unwrap());
}

pub const SCHEMA: &str = r#"
$schema: "https://json-schema.org/draft-7"
title: Netplan Network Definition
description: "Representation of a Netplan network definition"
type: object
required:
  - network
additionalProperties: false
properties:
  network:
    type: object
    additionalProperties: false
    properties:
      # network.version
      version:
        type: integer
        maximum: 2
        minimum: 2

      # network.renderer
      renderer:
        $ref: /schemas/renderer

      # network.ethernets
      ethernets:
        type: object
        properties:
          renderer:
            $ref: /schemas/renderer

        patternProperties:
          # network.ethernets.<interface>
          ".*$":
            additionalProperties: false
            properties:
              link:
                type: string
              virtual-function-count:
                type: integer
                minimum: 0
              embedded-switch-mode:
                type: string
                enum: [switchdev, legacy]
              delay-virtual-functions-rebind:
                type: boolean
              infiniband-mode:
                type: string
                enum: [datagram, connected]

      vlans:
        type: object
        properties:
          renderer:
            $ref: /schemas/renderer

        patternProperties:
          # network.vlans.<interface>
          ".*$":
            additionalProperties: false
            properties:
              id:
                type: integer
              link:
                type: string

      bridges:
        type: object
        properties:
          renderer:
            $ref: /schemas/renderer

        patternProperties:
          # network.bridges.<interface>
          ".*$":
            additionalProperties: false
            properties:
              interfaces:
                type: array
                uniqueItems: true
                items:
                  type: string
              parameters:
                type: object
                additionalProperties: false
                properties:
                  ageing-time:
                    type: string
                  aging-time:
                    type: string
                  priority:
                    type: integer
                    minimum: 0
                    maximum: 65535
                  port-priority:
                    type: integer
                    minimum: 0
                    maximum: 63
                  forward-delay:
                    type: string
                  hello-time:
                    type: string
                  max-age:
                    type: string
                  path-cost:
                    type: integer
                  stp:
                    type: boolean
      modems:
        type: object
        properties:
          renderer:
            $ref: /schemas/renderer

        patternProperties:
          # network.modems.<interface>
          ".*$":
            additionalProperties: false
            properties: {}

      bonds:
        type: object
        properties:
          renderer:
            $ref: /schemas/renderer

        patternProperties:
          # network.bonds.<interface>
          ".*$":
            additionalProperties: false
            properties: {}

      tunnels:
        type: object
        properties:
          renderer:
            $ref: /schemas/renderer

        patternProperties:
          # network.tunnels.<interface>
          ".*$":
            additionalProperties: false
            properties: {}

      vrfs:
        type: object
        properties:
          renderer:
            $ref: /schemas/renderer

        patternProperties:
          # network.tunnels.<interface>
          ".*$":
            additionalProperties: false
            properties: {}

      wifis:
        type: object
        properties:
          renderer:
            $ref: /schemas/renderer

        patternProperties:
          # network.wifis.<interface>
          ".*$":
            type: object
            additionalProperties: false
            properties: 
              access-points:
                type: object
                patternProperties:
                  ".*$":
                    type: object
                    additionalProperties: false
                    properties:
                      password:
                        type: string
                      mode:
                        type: string
                        enum: [infrastructure, ap, adhoc]
                      bssid:
                        type: string
                      band:
                        type: string
                        enum: [5GHz, 2.4GHz]
                      channel:
                        type: integer
                      hidden:
                        type: boolean
                      auth:
                        type: object
                        additionalProperties: false
                        properties:
                          key-management:
                            type: string
                            enum: [none, psk, eap]
                          password:
                            type: string
                          method:
                            type: string
                            enum: [tls, peap, ttls]
                          identity:
                            type: string
                          anonymous-identity:
                            type: string
                          ca-certificate:
                            type: string
                          client-certificate:
                            type: string
                          client-key:
                            type: string
                          client-key-password:
                            type: string
                          phase2-auth:
                            type: string


      nm-devices:
        type: object
        properties:
          renderer:
            $ref: /schemas/renderer

        patternProperties:
          # network.tunnels.<interface>
          ".*$":
            additionalProperties: false
            properties: {}
           

$defs:
  renderer:
    $id: /schemas/renderer
    "$schema": "http://json-schema.org/draft-07/schema#"
    type: string
    enum: [networkd, NetworkManager, sriov]
"#;

pub const COMMON_PROPERTIES: &str = r#"
renderer:
  $ref: /schemas/renderer

# network.ethernets.<interface>.dhcp4
dhcp4:
  type: boolean

dhcp6:
  type: boolean

ipv6-mtu:
  type: integer
  minimum: 0

ipv6-privacy:
  type: boolean

link-local:
  type: array
  uniqueItems: true
  items:
    type: string
    enum: [ ipv4, ipv6]

ignore-carrier:
  type: boolean

critical:
  type: boolean

dhcp-identifier:
  type: string
  enum: [duid, mac]

dhcp4-overrides:
  type: object
  additionalProperties: false
  properties:
    use-dns:
      type: boolean
    use-ntp:
      type: boolean
    send-hostname:
      type: boolean
    use-hostname:
      type: boolean
    use-mtu:
      type: boolean
    hostname:
      type: string
    use-routes:
      type: boolean
    route-metric:
      type: integer
    use-domains:
      type: boolean
  
dhcp6-overrides:
  type: object
  additionalProperties: false
  properties:
    use-dns:
      type: boolean
    use-ntp:
      type: boolean
    send-hostname:
      type: boolean
    use-hostname:
      type: boolean
    use-mtu:
      type: boolean
    hostname:
      type: string
    use-routes:
      type: boolean
    route-metric:
      type: integer
    use-domains:
      type: boolean

accept-ra:
  type: boolean

addresses:
  type: array
  uniqueItems: true
  items:
    anyOf:
      - type: object
        patternProperties:
          ".*$": # TODO: regex to match ipv4 and ipv6 with prefix
            type: object
            additionalProperties: false
            properties:
              lifetime:
                type: string
                enum: [forever, 0]
              label:
                type: string
                maxLength: 15
      - type: string
        pattern: .*$ # TODO: regex to match ipv4 and ipv6 with prefix

ipv6-address-generation:
  type: string
  enum: [eui64, stable-privacy]
ipv6-address-token: # TODO can't be used with the field above
  type: string
    
gateway4:
  type: string
  format: ipv4

gateway6:
  type: string
  format: ipv6

nameservers:
  type: object
  additionalProperties: false
  properties:
    search:
      type: array
      items:
        type: string
    addresses:
      type: array
      items:
        type: string

macaddress:
  type: string
  pattern: ([0-9a-f]{2}):([0-9a-f]{2}):([0-9a-f]{2}):([0-9a-f]{2}):([0-9a-f]{2}):([0-9a-f]{2})

mtu:
  type: integer
  minimum: 0

optional:
  type: boolean

optional-addresses:
  type: array
  items:
    type: string
    enum: [ipv4-ll, ipv6-ra, dhcp4, dhcp6, static]

activation-mode:
  type: string
  enum: [manual, off]

routes:
  type: array
  items:
    type: object
    additionalProperties: false
    properties:
      from:
        type: string
      to:
        type: string
      via:
        type: string
      on-link:
        type: boolean
      metric:
        type: integer
        minimum: 0
      type:
        type: string
        enum: [unicast, anycast, blackhole, broadcast, local, multicast, nat, prohibit, throw, unreachable, xresolve]
      scope:
        type: string
        enum: [global, link, host]
      table:
        type: integer
        minimum: 0
      mtu:
        type: integer
        minimum: 0
      congestion-window:
        type: integer
        minimum: 0
      advertised-receive-window:
        type: integer
        minimum: 0

routing-policy:
  type: object
  additionalProperties: false
  properties:
    from:
      type: string
    to:
      type: string
    table:
      type: integer
      minimum: 0
    priority:
      type: integer
    mark:
      type: integer
      minimum: 1
    type-of-service:
      type: integer

neigh-suppress:
  type: boolean

match:
  type: object
  additionalProperties: false
  properties:
    name:
      type: string
    driver:
      type: string
    macaddress:
      type: string
      pattern: ([0-9a-f]{2}):([0-9a-f]{2}):([0-9a-f]{2}):([0-9a-f]{2}):([0-9a-f]{2}):([0-9a-f]{2})
    

"#;
