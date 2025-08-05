use anyhow::Result;

/// عملیات‌هایی که باید روی فایل اجرا بشن
pub trait FileChecks {
    fn check_hash(&self) -> Result<String>;
    fn check_size(&self) -> Result<u64>;
    fn check_name(&self) -> Result<()>;
    fn check_type(&self) -> Result<String>;
}
