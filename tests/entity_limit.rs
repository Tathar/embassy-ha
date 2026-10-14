use embassy_ha::{DeviceConfig, DeviceResources, config::ENTITY_LIMIT, create_sensor};

fn device_config() -> DeviceConfig {
    DeviceConfig {
        device_id: "test",
        device_name: "Test",
        manufacturer: "Test",
        model: "Test",
    }
}

#[test]
fn configured_limit_matches_environment_or_default() {
    let expected = option_env!("EMBASSY_HA_ENTITY_LIMIT")
        .map(|value| value.parse::<usize>().unwrap())
        .unwrap_or(16);
    assert_eq!(ENTITY_LIMIT, expected);
}

#[test]
fn accepts_configured_number_of_entities_without_type_annotations() {
    let mut resources = DeviceResources::default();
    let device = embassy_ha::new(&mut resources, device_config());

    for _ in 0..ENTITY_LIMIT {
        create_sensor(&device, "sensor", Default::default());
    }
}

#[test]
#[should_panic(expected = "device entity limit reached")]
fn exceeding_configured_limit_panics() {
    let mut resources = DeviceResources::default();
    let device = embassy_ha::new(&mut resources, device_config());

    for _ in 0..=ENTITY_LIMIT {
        create_sensor(&device, "sensor", Default::default());
    }
}
