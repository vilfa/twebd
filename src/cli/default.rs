pub fn address() -> std::net::IpAddr {
    use std::net;
    net::IpAddr::V4(net::Ipv4Addr::new(127, 0, 0, 1))
}

pub fn port() -> u16 {
    8080
}

pub fn protocol() -> crate::net::DataProtocol {
    crate::net::DataProtocol::Tcp
}

pub fn directory() -> std::path::PathBuf {
    std::path::PathBuf::from("./public")
}

pub fn loglevel() -> log::LevelFilter {
    log::LevelFilter::Info
}

pub fn threads() -> usize {
    4
}

pub fn threads_max() -> usize {
    10
}

pub fn https() -> bool {
    false
}

pub fn https_cert() -> std::path::PathBuf {
    std::path::PathBuf::from("./ssl/localhost.crt")
}

pub fn https_priv_key() -> std::path::PathBuf {
    std::path::PathBuf::from("./ssl/localhost.key")
}
