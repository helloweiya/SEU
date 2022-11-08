
mod notify;
mod bark_sender;
use notify::NotifySender;
use bark_sender::BarkSender;
const PREFIX: &str = "https://bark.hellowei.top";
const UUID: &str = "5Ar5gkJpLvMbRkAMMSB4UN";
fn main() {
    println!("Hello, world!");
    let barker = BarkSender::new(PREFIX, UUID);
    let sender:NotifySender<_> = NotifySender::new(barker);
    sender.send_msg("申报成功").expect("TODO: panic message");

}
