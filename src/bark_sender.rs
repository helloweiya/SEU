use anyhow::Result;
use reqwest::{Error, StatusCode};
use crate::notify::Notify;
pub struct BarkSender{
    url: String,
}
impl BarkSender {
    pub fn new(prefix: &str, uuid: &str) ->Self {
        BarkSender{
            url:format!("{}/{}",prefix,uuid),
        }
    }
    pub fn get_complete_url(&self,msg: &str) -> String {
        format!("{}/{}",self.url, msg)
    }
    fn send(&self,msg: &str) -> Result<()>{
        let url = self.get_complete_url(msg);
        let body = reqwest::blocking::get(url).unwrap();
        if body.status() != StatusCode::OK {
            println!("notify failed!!")
        }
        Ok(())
    }
}
impl Notify for BarkSender {
    fn send_msg(&self,msg: &str) -> Result<()> {
        self.send(msg)?;
        Ok(())
    }
}