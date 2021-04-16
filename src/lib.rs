use indexmap::IndexMap;
/// Parse the htpasswd string and create a IndexMap of user/password pairs.
///
/// # Examples
///
/// ```
/// use indexmap::IndexMap;
/// use htpasswd::parse;
/// let input = "
/// johndoe:$apr1$hdqQY4oe$6PtEz0XH6ORg.GPKCTpG31
/// janedoe
/// # comment
/// janedoe:$apr1$D7qCR.yD$vfKO/2urv89Okpxl8VGpb/
/// ";
/// let mut output = IndexMap::new();
/// output.insert("johndoe", "$apr1$hdqQY4oe$6PtEz0XH6ORg.GPKCTpG31");
/// output.insert("janedoe", "$apr1$D7qCR.yD$vfKO/2urv89Okpxl8VGpb/");
/// assert_eq!(output, parse(input));
/// ```
pub fn parse(text: &str) -> IndexMap<&str, &str> {
    let mut map = IndexMap::new();
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

/// Transform the IndexMap of user/password pairs as create a htpasswd string.
///
/// # Examples
///
/// ```
/// use indexmap::IndexMap;
/// use htpasswd::stringify;
/// let output = "johndoe:$apr1$hdqQY4oe$6PtEz0XH6ORg.GPKCTpG31
/// janedoe:$apr1$D7qCR.yD$vfKO/2urv89Okpxl8VGpb/
/// ";
/// let mut input = IndexMap::new();
/// input.insert("johndoe", "$apr1$hdqQY4oe$6PtEz0XH6ORg.GPKCTpG31");
/// input.insert("janedoe", "$apr1$D7qCR.yD$vfKO/2urv89Okpxl8VGpb/");
/// assert_eq!(output, stringify(input));
/// ```
pub fn stringify<'a>(map: IndexMap<&str, &str>) -> String {
    let mut string: String = "".to_string();

    for (&username, &pwd_hash) in map.iter() {
        let line = format!("{}:{}\n", username, pwd_hash);
        string.push_str(line.as_str());
    }
    string
}

#[cfg(test)]
mod tests {
    use super::parse;
    use super::stringify;
    use indexmap::IndexMap;

    #[test]
    fn it_parse() {
        let input = "johndoe:$apr1$hdqQY4oe$6PtEz0XH6ORg.GPKCTpG31
janedoe
# comment
janedoe:$apr1$D7qCR.yD$vfKO/2urv89Okpxl8VGpb/
        ";
        let mut output = IndexMap::new();
        output.insert("johndoe", "$apr1$hdqQY4oe$6PtEz0XH6ORg.GPKCTpG31");
        output.insert("janedoe", "$apr1$D7qCR.yD$vfKO/2urv89Okpxl8VGpb/");
        assert_eq!(output, parse(input));
    }

    #[test]
    fn it_stringify() {
        let output = "johndoe:$apr1$hdqQY4oe$6PtEz0XH6ORg.GPKCTpG31
janedoe:$apr1$D7qCR.yD$vfKO/2urv89Okpxl8VGpb/
";
        let mut input = IndexMap::new();
        input.insert("johndoe", "$apr1$hdqQY4oe$6PtEz0XH6ORg.GPKCTpG31");
        input.insert("janedoe", "$apr1$D7qCR.yD$vfKO/2urv89Okpxl8VGpb/");
        assert_eq!(output, stringify(input));
    }
}
