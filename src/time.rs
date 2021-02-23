

#[derive(Debug, Copy, Clone)]
pub struct SystemTime(u64);

pub const UNIX_EPOCH: SystemTime = SystemTime(0);

impl SystemTime {
    pub fn duration_since(&self, time: SystemTime) -> Result<SystemTime, ()> {
        if self.0 - time.0 > 0 {
            Ok(SystemTime (self.0 - time.0))
        } else {
            Err(())
        }
    }

    pub fn as_secs(&self) -> u64 {
        self.0
    }

    pub fn now() -> SystemTime {
        now()
    }

 }

pub fn now() -> SystemTime {
    SystemTime(1612680532)
}
