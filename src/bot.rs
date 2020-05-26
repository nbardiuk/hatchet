pub struct Bot {
    output: Vec<String>,
}

impl Bot {
    pub fn new(pass: &str, nick: &str, channel: &str) -> Self {
        let output = vec![
            format!("JOIN #{}", channel),
            format!("NICK {}", nick),
            format!("PASS {}", pass),
        ];
        Self { output }
    }

    pub fn handle(&mut self, line: String) {
        if line == "PING :tmi.twitch.tv" {
            self.output.insert(0, "PONG :tmi.twitch.tv".to_string());
        }
    }

    pub fn next(&mut self) -> Option<String> {
        self.output.pop()
    }
}

#[cfg(test)]
mod specs {
    use super::*;

    #[test]
    fn joins() {
        let mut bot = Bot::new("secret", "name", "streamer");

        assert_eq!(bot.next().unwrap(), "PASS secret");
        assert_eq!(bot.next().unwrap(), "NICK name");
        assert_eq!(bot.next().unwrap(), "JOIN #streamer");
        assert_eq!(bot.next(), None);
    }

    #[test]
    fn pongs() {
        let mut bot = Bot::new("pass", "bot", "channel");
        while bot.next().is_some() {}

        bot.handle(String::from("PING :tmi.twitch.tv"));

        assert_eq!(bot.next().unwrap(), "PONG :tmi.twitch.tv");
        assert_eq!(bot.next(), None);
    }
}
