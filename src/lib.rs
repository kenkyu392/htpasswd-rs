use std::collections::HashMap;

/// Parse the htpasswd string and create a HashMap of user/password pairs.
///
/// # Examples
///
/// ```
/// let input = "
/// johndoe:$apr1$hdqQY4oe$6PtEz0XH6ORg.GPKCTpG31
/// janedoe
/// # comment
/// janedoe:$apr1$D7qCR.yD$vfKO/2urv89Okpxl8VGpb/
/// ";
/// let mut output = HashMap::new();
/// output.insert("johndoe", "$apr1$hdqQY4oe$6PtEz0XH6ORg.GPKCTpG31");
/// output.insert("janedoe", "$apr1$D7qCR.yD$vfKO/2urv89Okpxl8VGpb/");
/// assert_eq!(output, parse(input));
/// ```
pub fn parse(text: &str) -> HashMap<&str, &str> {
    let mut map = HashMap::new();
    for line in text.trim().lines() {
        let parts: Vec<&str> = line.trim().split(":").collect();
        if parts.len() != 2 {
            continue;
        }
        if parts[0].starts_with("#") {
            continue;
        }
        map.insert(parts[0].trim(), parts[1].trim());
    }
    return map;
}

#[cfg(test)]
mod tests {
    use super::parse;
    use std::collections::HashMap;

    #[test]
    fn it_parse() {
        let input = "
        johndoe:$apr1$hdqQY4oe$6PtEz0XH6ORg.GPKCTpG31
        janedoe
        # comment
        janedoe:$apr1$D7qCR.yD$vfKO/2urv89Okpxl8VGpb/
        ";
        let mut output = HashMap::new();
        output.insert("johndoe", "$apr1$hdqQY4oe$6PtEz0XH6ORg.GPKCTpG31");
        output.insert("janedoe", "$apr1$D7qCR.yD$vfKO/2urv89Okpxl8VGpb/");
        assert_eq!(output, parse(input));
    }
}
