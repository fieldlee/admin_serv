
pub mod index;

/// 登录失败后锁定时间
pub const LOGIN_LOCKED_TIME: usize = 3600;
/// 最多的失败次数
pub const LOGIN_ERROR_MAX :usize = 3;