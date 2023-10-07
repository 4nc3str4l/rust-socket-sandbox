use regex::Regex;

pub fn is_valid_websocket_ip(ip: &str) -> bool {
    let re = Regex::new(
        r"^(ws|wss)://([a-zA-Z0-9\.-]+|[0-9]{1,3}(\.[0-9]{1,3}){3})(:[0-9]{1,5})?(/[\w/-]*)*$",
    )
    .unwrap();
    re.is_match(ip)
}
