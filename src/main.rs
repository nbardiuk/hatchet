use crate::bot::Bot;
use chrono::prelude::*;
use std::env;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::net::TcpStream;
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;

mod bot;

fn main() -> io::Result<()> {
    let password = env::var("OATH_TOKEN").expect("missing OATH_TOKEN in env");
    let bot_username = env::var("BOT_USERNAME").expect("missing BOT_USERNAME in env");
    let channel_name = env::var("CHANNEL_NAME").expect("missing CHANNEL_NAME in env");

    let stream = TcpStream::connect("irc.chat.twitch.tv:6667")?;
    let (output_lines, input_lines) = read_write(stream)?;

    let mut bot = Bot::new(&password, &bot_username, &channel_name);

    loop {
        if let Ok(line) = input_lines.recv_timeout(Duration::from_millis(50)) {
            println!("{} < {}", now(), line);
            bot.handle(line);
        }
        while let Some(line) = bot.next() {
            println!("{} > {}", now(), mask(&line));
            output_lines.send(line).unwrap();
        }
    }
}

fn now() -> String {
    Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true)
}

fn mask(s: &str) -> &str {
    if s.starts_with("PASS") {
        "PASS qwerty1234"
    } else {
        s
    }
}

fn read_write(stream: TcpStream) -> io::Result<(Sender<String>, Receiver<String>)> {
    let receiver = read_loop(stream.try_clone()?);
    let sender = write_loop(stream);
    Ok((sender, receiver))
}

fn write_loop<W: Write + Send + 'static>(mut write: W) -> Sender<String> {
    let (sender, receiver) = channel();
    thread::spawn(move || {
        while let Ok(args) = receiver.recv() {
            write.write_fmt(format_args!("{}\r\n", args)).unwrap(); // FIXME i/o error, reconnect???
        }
    });
    sender
}

fn read_loop<R: Read + Send + 'static>(stream: R) -> Receiver<String> {
    let (sender, receiver) = channel();
    thread::spawn(move || {
        let mut reader = BufReader::new(stream);
        loop {
            let mut line = String::new();
            reader.read_line(&mut line).unwrap(); // FIXME i/o error, reconnect???

            line.pop(); // \n
            line.pop(); // \r
            sender.send(line).unwrap(); // FIXME receiver is dealocated, log and quit
        }
    });
    receiver
}
