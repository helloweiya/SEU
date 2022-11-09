mod notify;
mod bark_sender;
mod ids_encrypt;
mod login;
mod dayReport;

use tokio;
use notify::NotifySender;
use bark_sender::BarkSender;
const PREFIX: &str = "https://bark.hellowei.top";
const UUID: &str = "5Ar5gkJpLvMbRkAMMSB4UN";

static barker: bark_sender = BarkSender::new(PREFIX, UUID);
static  sender:NotifySender<bark_sender> = NotifySender::new(barker);
#[tokio::main]
async fn main() {
    // println!("Hello, world!");

    // let x = ids_encrypt::encrypt_aes("220201756","dsa");
    // println!("{}",x);
    let client = login::login("220201756","1022LWxy@").await.unwrap();

}

