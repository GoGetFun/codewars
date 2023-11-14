fn int32_to_ip(int: u32) -> String {
    let octets = [
        (int >> 24) & 255,
        (int >> 16) & 255,
        (int >> 8) & 255,
        int & 255,
    ];
    format!("{}.{}.{}.{}", octets[0], octets[1], octets[2], octets[3])
}