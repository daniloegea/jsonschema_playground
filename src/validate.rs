use jsonschema::{error::ValidationErrorKind, JSONSchema};
use serde_json::value::Value;

pub mod schema;

pub fn validate(schema: &JSONSchema, yaml: &str) -> Result<(), String> {
    let data = match serde_yaml::from_str::<Value>(yaml) {
        Ok(v) => v,
        Err(_) => {
            return Err("serde_yaml failed to parse the file".to_string());
        }
    };

    if let Err(err) = schema.validate(&data) {
        for error in err {
            match error.kind {
                ValidationErrorKind::AdditionalProperties { unexpected } => {
                    for u in &unexpected {
                        let e =
                            format!("Unexpected keyword {}/{u}", error.instance_path.to_string());
                        return Err(e);
                    }
                }
                ValidationErrorKind::UniqueItems => {
                    let e = format!(
                        "Duplicate item {}/{}",
                        error.instance_path.to_string(),
                        error.instance.to_string(),
                    );
                    return Err(e);
                }
                _ => {
                    let e = format!(
                        "Unexpected value {}: {}",
                        error.instance_path.to_string(),
                        error.instance.to_string()
                    );
                    return Err(e);
                }
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_net_a_network() {
        let schema = schema::build_schema().unwrap();
        let result = validate(
            &schema,
            r#"
            somethingelse:
              version: 2
        "#,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_network_invalid_property() {
        let schema = schema::build_schema().unwrap();
        let result = validate(
            &schema,
            r#"
            network:
              notavalidproperty: 2
        "#,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_network_valid_version() {
        let schema = schema::build_schema().unwrap();
        let result = validate(
            &schema,
            r#"
            network:
              version: 2
        "#,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_network_invalid_version() {
        let schema = schema::build_schema().unwrap();
        let result = validate(
            &schema,
            r#"
            network:
              version: 3
        "#,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_network_invalid_renderer() {
        let schema = schema::build_schema().unwrap();
        let result = validate(
            &schema,
            r#"
            network:
              renderer: invalid
        "#,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_network_valid_renderer() {
        let schema = schema::build_schema().unwrap();
        let result = validate(
            &schema,
            r#"
            network:
              renderer: networkd
        "#,
        );
        assert!(result.is_ok());

        let result = validate(
            &schema,
            r#"
            network:
              renderer: NetworkManager
        "#,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_ethernet_empty() {
        let schema = schema::build_schema().unwrap();
        let result = validate(
            &schema,
            r#"
            network:
              ethernets:
                eth0: {}
        "#,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_ethernet_invalid_property() {
        let schema = schema::build_schema().unwrap();
        let result = validate(
            &schema,
            r#"
            network:
              ethernets:
                eth0:
                  weird_property: veryweird
        "#,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_ethernet_link() {
        let schema = schema::build_schema().unwrap();
        let result = validate(
            &schema,
            r#"
            network:
              ethernets:
                eth0:
                  link: eth1
        "#,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_ethernet_invalid_link() {
        let schema = schema::build_schema().unwrap();
        let result = validate(
            &schema,
            r#"
            network:
              ethernets:
                eth0:
                  link: 0
        "#,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_ethernet_virtual_function_count() {
        let schema = schema::build_schema().unwrap();
        let result = validate(
            &schema,
            r#"
            network:
              ethernets:
                eth0:
                  virtual-function-count: 1
        "#,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_ethernet_invalid_virtual_function_count() {
        let schema = schema::build_schema().unwrap();
        let result = validate(
            &schema,
            r#"
            network:
              ethernets:
                eth0:
                  virtual-function-count: -1
        "#,
        );
        assert!(result.is_err());

        let result = validate(
            &schema,
            r#"
            network:
              ethernets:
                eth0:
                  virtual-function-count: abc
        "#,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_ethernet_embedded_switch_mode() {
        let schema = schema::build_schema().unwrap();
        let result = validate(
            &schema,
            r#"
            network:
              ethernets:
                eth0:
                  embedded-switch-mode: switchdev
        "#,
        );
        assert!(result.is_ok());

        let result = validate(
            &schema,
            r#"
            network:
              ethernets:
                eth0:
                  embedded-switch-mode: legacy
        "#,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_ethernet_invalid_embedded_switch_mode() {
        let schema = schema::build_schema().unwrap();
        let result = validate(
            &schema,
            r#"
            network:
              ethernets:
                eth0:
                  embedded-switch-mode: 0
        "#,
        );
        assert!(result.is_err());

        let result = validate(
            &schema,
            r#"
            network:
              ethernets:
                eth0:
                  embedded-switch-mode: something
        "#,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_ethernet_delay_virtual_functions_rebind() {
        let schema = schema::build_schema().unwrap();
        let result = validate(
            &schema,
            r#"
            network:
              ethernets:
                eth0:
                  delay-virtual-functions-rebind: false
        "#,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_ethernet_invalid_delay_virtual_functions_rebind() {
        let schema = schema::build_schema().unwrap();
        let result = validate(
            &schema,
            r#"
            network:
              ethernets:
                eth0:
                  delay-virtual-functions-rebind: something
        "#,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_ethernet_infiniband_mode() {
        let schema = schema::build_schema().unwrap();
        let result = validate(
            &schema,
            r#"
            network:
              ethernets:
                eth0:
                  infiniband-mode: datagram
        "#,
        );
        assert!(result.is_ok());

        let result = validate(
            &schema,
            r#"
            network:
              ethernets:
                eth0:
                  infiniband-mode: connected
        "#,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_ethernet_invalid_infiniband_mode() {
        let schema = schema::build_schema().unwrap();
        let result = validate(
            &schema,
            r#"
            network:
              ethernets:
                eth0:
                  infiniband-mode: something
        "#,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_ethernet_with_renderer() {
        let schema = schema::build_schema().unwrap();
        let result = validate(
            &schema,
            r#"
            network:
              ethernets:
                eth0:
                  renderer: networkd
        "#,
        );
        assert!(result.is_ok());

        let result = validate(
            &schema,
            r#"
            network:
              ethernets:
                eth0:
                  renderer: NetworkManager
        "#,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_ethernet_with_invalid_renderer() {
        let schema = schema::build_schema().unwrap();
        let result = validate(
            &schema,
            r#"
            network:
              ethernets:
                eth0:
                  renderer: invalid
        "#,
        );
        assert!(result.is_err());

        let result = validate(
            &schema,
            r#"
            network:
              ethernets:
                eth0:
                  renderer: NetworkManager
        "#,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_ethernet_dhcp() {
        let schema = schema::build_schema().unwrap();
        let result = validate(
            &schema,
            r#"
            network:
              ethernets:
                eth0:
                  dhcp4: true
                  dhcp6: false
        "#,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_ethernet_dhcp_invalid() {
        let schema = schema::build_schema().unwrap();
        let result = validate(
            &schema,
            r#"
            network:
              ethernets:
                eth0:
                  dhcp4: 1
        "#,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_ethernet_ipv6_mtu() {
        let schema = schema::build_schema().unwrap();
        let result = validate(
            &schema,
            r#"
            network:
              ethernets:
                eth0:
                  ipv6-mtu: 1500
        "#,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_ethernet_invalid_ipv6_mtu() {
        let schema = schema::build_schema().unwrap();
        let result = validate(
            &schema,
            r#"
            network:
              ethernets:
                eth0:
                  ipv6-mtu: something
        "#,
        );
        assert!(result.is_err());

        let result = validate(
            &schema,
            r#"
            network:
              ethernets:
                eth0:
                  ipv6-mtu: -1
        "#,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_ethernet_ipv6_privacy() {
        let schema = schema::build_schema().unwrap();
        let result = validate(
            &schema,
            r#"
            network:
              ethernets:
                eth0:
                  ipv6-privacy: true
        "#,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_ethernet_invalid_ipv6_privacy() {
        let schema = schema::build_schema().unwrap();
        let result = validate(
            &schema,
            r#"
            network:
              ethernets:
                eth0:
                  ipv6-privacy: something
        "#,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_ethernet_link_local() {
        let schema = schema::build_schema().unwrap();
        let result = validate(
            &schema,
            r#"
            network:
              ethernets:
                eth0:
                  link-local:
                    - ipv4
                    - ipv6
        "#,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_ethernet_invalid_link_local() {
        let schema = schema::build_schema().unwrap();
        let result = validate(
            &schema,
            r#"
            network:
              ethernets:
                eth0:
                  link-local:
                    - ipv4
                    - ipv6
                    - somethingelse
        "#,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_ethernet_ignore_carrier() {
        let schema = schema::build_schema().unwrap();
        let result = validate(
            &schema,
            r#"
            network:
              ethernets:
                eth0:
                  ignore-carrier: true
        "#,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_ethernet_invalid_ignore_carrier() {
        let schema = schema::build_schema().unwrap();
        let result = validate(
            &schema,
            r#"
            network:
              ethernets:
                eth0:
                  ignore-carrier: something
        "#,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_ethernet_critical() {
        let schema = schema::build_schema().unwrap();
        let result = validate(
            &schema,
            r#"
            network:
              ethernets:
                eth0:
                  critical: false
        "#,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_ethernet_invalid_critical() {
        let schema = schema::build_schema().unwrap();
        let result = validate(
            &schema,
            r#"
            network:
              ethernets:
                eth0:
                  critical: something
        "#,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_ethernet_dhcp_identifier() {
        let schema = schema::build_schema().unwrap();
        let result = validate(
            &schema,
            r#"
            network:
              ethernets:
                eth0:
                  dhcp-identifier: duid
        "#,
        );
        assert!(result.is_ok());

        let result = validate(
            &schema,
            r#"
            network:
              ethernets:
                eth0:
                  dhcp-identifier: mac
        "#,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_ethernet_dhcp4_override() {
        let schema = schema::build_schema().unwrap();
        let result = validate(
            &schema,
            r#"
            network:
              ethernets:
                eth0:
                  dhcp4-overrides:
                    use-dns: false
                    use-ntp: false
                    send-hostname: false
                    use-hostname: false
                    use-mtu: false
                    hostname: something
                    use-routes: false
                    route-metric: 123
                    use-domains: false
        "#,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_ethernet_invalid_dhcp4_override() {
        let schema = schema::build_schema().unwrap();
        let result = validate(
            &schema,
            r#"
            network:
              ethernets:
                eth0:
                  dhcp4-overrides:
                    invalid-property: abc
        "#,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_ethernet_dhcp6_override() {
        let schema = schema::build_schema().unwrap();
        let result = validate(
            &schema,
            r#"
            network:
              ethernets:
                eth0:
                  dhcp6-overrides:
                    use-dns: false
                    use-ntp: false
                    send-hostname: false
                    use-hostname: false
                    use-mtu: false
                    hostname: something
                    use-routes: false
                    route-metric: 123
                    use-domains: false
        "#,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_ethernet_invalid_dhcp6_override() {
        let schema = schema::build_schema().unwrap();
        let result = validate(
            &schema,
            r#"
            network:
              ethernets:
                eth0:
                  dhcp6-overrides:
                    invalid-property: abc
        "#,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_ethernet_accept_ra() {
        let schema = schema::build_schema().unwrap();
        let result = validate(
            &schema,
            r#"
            network:
              ethernets:
                eth0:
                  accept-ra: false
        "#,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_ethernet_invalid_accept_ra() {
        let schema = schema::build_schema().unwrap();
        let result = validate(
            &schema,
            r#"
            network:
              ethernets:
                eth0:
                  accept-ra: something
        "#,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_ethernet_addresses() {
        let schema = schema::build_schema().unwrap();
        let result = validate(
            &schema,
            r#"
            network:
              ethernets:
                eth0:
                  addresses:
                    - 10.1.2.3/24
                    - 10.1.2.4/24
                    - 192.168.0.1/24:
                        lifetime: forever
                        label: alabel
        "#,
        );
        assert!(result.is_ok());

        let result = validate(
            &schema,
            r#"
            network:
              ethernets:
                eth0:
                  addresses:
                    - 192.168.0.1/24:
                        lifetime: forever
                        label: alabel
                    - 10.1.2.3/24
                    - 10.1.2.4/24
        "#,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_ethernet_non_unique_addresses() {
        let schema = schema::build_schema().unwrap();
        let result = validate(
            &schema,
            r#"
            network:
              ethernets:
                eth0:
                  addresses:
                    - 10.1.2.3/24
                    - 10.1.2.3/24
                    - 192.168.0.1/24:
                        lifetime: forever
                        label: alabel
        "#,
        );
        assert!(result.is_err());
        assert!(result.err().unwrap().starts_with("Duplicate item"));

        let result = validate(
            &schema,
            r#"
            network:
              ethernets:
                eth0:
                  addresses:
                    - 192.168.0.1/24:
                        lifetime: forever
                        label: alabel
                    - 192.168.0.1/24:
                        lifetime: forever
                        label: alabel
                    - 10.1.2.3/24
                    - 10.1.2.4/24
        "#,
        );
        assert!(result.is_err());

        assert!(result.err().unwrap().starts_with("Duplicate item"));
    }

    #[test]
    fn test_ethernet_ipv6_address_generation() {
        let schema = schema::build_schema().unwrap();
        let result = validate(
            &schema,
            r#"
            network:
              ethernets:
                eth0:
                  ipv6-address-generation: eui64
        "#,
        );
        assert!(result.is_ok());

        let result = validate(
            &schema,
            r#"
            network:
              ethernets:
                eth0:
                  ipv6-address-generation: stable-privacy
        "#,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_ethernet_invalid_ipv6_address_generation() {
        let schema = schema::build_schema().unwrap();
        let result = validate(
            &schema,
            r#"
            network:
              ethernets:
                eth0:
                  ipv6-address-generation: something
        "#,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_ethernet_gateway4() {
        let schema = schema::build_schema().unwrap();
        let result = validate(
            &schema,
            r#"
            network:
              ethernets:
                eth0:
                  gateway4: 10.1.2.3
        "#,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_ethernet_gateway6() {
        let schema = schema::build_schema().unwrap();
        let result = validate(
            &schema,
            r#"
            network:
              ethernets:
                eth0:
                  gateway6: abcd::1
        "#,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_ethernet_invalid_gateway4() {
        let schema = schema::build_schema().unwrap();
        let result = validate(
            &schema,
            r#"
            network:
              ethernets:
                eth0:
                  gateway4: 10.1.2.3.1
        "#,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_ethernet_invalid_gateway6() {
        let schema = schema::build_schema().unwrap();
        let result = validate(
            &schema,
            r#"
            network:
              ethernets:
                eth0:
                  gateway6: abcd:::1
        "#,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_ethernet_nameservers() {
        let schema = schema::build_schema().unwrap();
        let result = validate(
            &schema,
            r#"
            network:
              ethernets:
                eth0:
                  nameservers:
                    addresses:
                      - 8.8.8.8
                      - 8.8.4.4
                    search:
                      - home.com
                      - home.local
                    
        "#,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_ethernet_invalid_nameservers() {
        let schema = schema::build_schema().unwrap();
        let result = validate(
            &schema,
            r#"
            network:
              ethernets:
                eth0:
                  nameservers:
                    addresses:
                      - 8.8.8.8
                      - 8.8.4.4
                    search:
                      - home.com
                      - home.local
                    something: else
                    
        "#,
        );
        assert!(result.is_err());

        let result = validate(
            &schema,
            r#"
            network:
              ethernets:
                eth0:
                  nameservers:
                    addresses: false
                    
        "#,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_ethernet_macaddress() {
        let schema = schema::build_schema().unwrap();
        let result = validate(
            &schema,
            r#"
            network:
              ethernets:
                eth0:
                  macaddress: 00:11:22:33:aa:bb:cc
        "#,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_ethernet_invalid_macaddress() {
        let schema = schema::build_schema().unwrap();
        let result = validate(
            &schema,
            r#"
            network:
              ethernets:
                eth0:
                  macaddress: 00:33:aa:bb:cc
        "#,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_ethernet_mtu() {
        let schema = schema::build_schema().unwrap();
        let result = validate(
            &schema,
            r#"
            network:
              ethernets:
                eth0:
                  mtu: 1500
        "#,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_ethernet_invalid_mtu() {
        let schema = schema::build_schema().unwrap();
        let result = validate(
            &schema,
            r#"
            network:
              ethernets:
                eth0:
                  mtu: -1
        "#,
        );
        assert!(result.is_err());

        let result = validate(
            &schema,
            r#"
            network:
              ethernets:
                eth0:
                  mtu: something
        "#,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_ethernet_optional() {
        let schema = schema::build_schema().unwrap();
        let result = validate(
            &schema,
            r#"
            network:
              ethernets:
                eth0:
                  optional: true
        "#,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_ethernet_invalid_optional() {
        let schema = schema::build_schema().unwrap();
        let result = validate(
            &schema,
            r#"
            network:
              ethernets:
                eth0:
                  optional: something
        "#,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_ethernet_optional_addresses() {
        let schema = schema::build_schema().unwrap();
        let result = validate(
            &schema,
            r#"
            network:
              ethernets:
                eth0:
                  optional-addresses:
                    - ipv4-ll
                    - ipv6-ra
                    - dhcp4
                    - dhcp6
                    - static
        "#,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_ethernet_invalid_optional_addresses() {
        let schema = schema::build_schema().unwrap();
        let result = validate(
            &schema,
            r#"
            network:
              ethernets:
                eth0:
                  optional-addresses:
                    - abc
        "#,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_ethernet_activation_mode() {
        let schema = schema::build_schema().unwrap();
        let result = validate(
            &schema,
            r#"
            network:
              ethernets:
                eth0:
                  activation-mode: manual
        "#,
        );
        assert!(result.is_ok());

        let result = validate(
            &schema,
            r#"
            network:
              ethernets:
                eth0:
                  activation-mode: off
        "#,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_ethernet_invalid_activation_mode() {
        let schema = schema::build_schema().unwrap();
        let result = validate(
            &schema,
            r#"
            network:
              ethernets:
                eth0:
                  activation-mode: something
        "#,
        );
        assert!(result.is_err());
    }
}
