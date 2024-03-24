use common::prelude::EnvironmentType;

#[test]
fn test_default() {
    let protocol = EnvironmentType::default();
    assert_eq!(protocol, EnvironmentType::UnknownEnv);
}

#[test]
fn test_debug() {
    let e1 = EnvironmentType::UnknownEnv;
    assert_eq!(format!("{:?}", e1), "UnknownEnv");

    let e2 = EnvironmentType::LOCAL;
    assert_eq!(format!("{:?}", e2), "LOCAL");

    let e3 = EnvironmentType::CLUSTER;
    assert_eq!(format!("{:?}", e3), "CLUSTER");

    let e4 = EnvironmentType::CI;
    assert_eq!(format!("{:?}", e4), "CI");
}

#[test]
fn test_display() {
    let e1 = EnvironmentType::UnknownEnv;
    assert_eq!(format!("{}", e1), "UnknownEnv");

    let e2 = EnvironmentType::LOCAL;
    assert_eq!(format!("{}", e2), "LOCAL");

    let e3 = EnvironmentType::CLUSTER;
    assert_eq!(format!("{}", e3), "CLUSTER");

    let e4 = EnvironmentType::CI;
    assert_eq!(format!("{}", e4), "CI");
}
