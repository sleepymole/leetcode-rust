#![allow(dead_code)]

struct Codec {
    urls: Vec<String>,
}

impl Codec {
    fn new() -> Self {
        Codec { urls: Vec::new() }
    }

    fn encode(&mut self, long_url: String) -> String {
        self.urls.push(long_url);
        format!("http://tinyurl.com/{}", self.urls.len() - 1)
    }

    fn decode(&self, short_url: String) -> String {
        let i = short_url
            .strip_prefix("http://tinyurl.com/")
            .map(|s| s.parse::<usize>().unwrap_or_default())
            .unwrap_or_default();
        self.urls[i].clone()
    }
}
