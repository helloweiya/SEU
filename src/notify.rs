use anyhow:: {Context, Result};
pub trait Notify {
    fn send_msg(&self,msg: &str) -> Result<()>;
}
pub struct NotifySender<T>
where
    T: Notify,
{
    sender: T,
}

impl<T:Notify> NotifySender<T> {
    pub fn new(item: T) -> Self {
        NotifySender{
            sender:item
        }
    }
    pub fn send_msg(&self,msg: &str) -> Result<()> {
        self.sender.send_msg(msg).context("send error")?;
        Ok(())
    }
}

