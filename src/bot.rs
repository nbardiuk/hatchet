use regex::Regex;

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
            return;
        }

        let re = Regex::new(r":[^ ]+!([^ ]+)@[^ ]+ PRIVMSG #([^ ]+) :!whoami( .*)?").unwrap();
        if re.is_match(&line) {
            if let Some(caps) = re.captures_iter(&line).next() {
                let user = &caps[1];
                let channel = &caps[2];
                let message = format!("PRIVMSG #{} :{}", channel, user);
                self.output.insert(0, message);
            }
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
    fn ping() {
        let mut bot = Bot::new("pass", "bot", "channel");
        while bot.next().is_some() {}

        bot.handle(String::from("PING :tmi.twitch.tv"));

        assert_eq!(bot.next().unwrap(), "PONG :tmi.twitch.tv");
        assert_eq!(bot.next(), None);
    }

    #[test]
    fn whoami() {
        let mut bot = Bot::new("pass", "bot", "channel");
        while bot.next().is_some() {}

        bot.handle(String::from(":nick!user@host PRIVMSG #channel :!whoami"));
        assert_eq!(bot.next().expect("nonempty"), "PRIVMSG #channel :user");
        assert_eq!(bot.next(), None);

        bot.handle(String::from(":n!u@h PRIVMSG #c :!whoami some garbage"));
        assert_eq!(bot.next().unwrap(), "PRIVMSG #c :u");
        assert_eq!(bot.next(), None);

        bot.handle(String::from(":n!u@h PRIVMSG #c :some garbage :!whoami"));
        assert_eq!(bot.next(), None);

        bot.handle(String::from(":n!u@h PRIVMSG #c :some garbage"));
        assert_eq!(bot.next(), None);
    }
}
