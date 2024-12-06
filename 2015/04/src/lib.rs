use common::Answer;

fn find_md5_prefix(secret: &str, prefix: &str) -> Answer {
    for i in 0..100_000_000 {
        let attempt = format!("{}{i}", secret.trim_end());
        let digest = md5::compute(attempt.as_bytes());

        if format!("{:x}", digest).starts_with(prefix) {
            return i.into();
        }
    }

    ().into()
}

pub fn step1(s: &str) -> Answer {
    find_md5_prefix(s, "00000")
}

pub fn step2(s: &str) -> Answer {
    find_md5_prefix(s, "000000")
}
