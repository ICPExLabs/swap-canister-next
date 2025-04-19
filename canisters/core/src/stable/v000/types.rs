use serde::{Deserialize, Serialize};

pub use ic_canister_kit::types::*;

#[allow(unused)]
pub use super::super::{Business, ParsePermission, ScheduleTask};

#[allow(unused)]
pub use super::super::business::*;
#[allow(unused)]
pub use super::business::*;
#[allow(unused)]
pub use super::permission::*;
#[allow(unused)]
pub use super::schedule::schedule_task;

// 初始化参数
#[derive(Debug, Clone, Serialize, Deserialize, candid::CandidType, Default)]
pub struct InitArg {
    pub maintainers: Option<Vec<UserId>>, // init maintainers or deployer
    pub schedule: Option<DurationNanos>,  // init scheduled task or not
}

// 升级参数
#[derive(Debug, Clone, Serialize, Deserialize, candid::CandidType)]
pub struct UpgradeArg {
    pub maintainers: Option<Vec<UserId>>, // add new maintainers of not
    pub schedule: Option<DurationNanos>,  // init scheduled task or not
}

// 框架需要的数据结构
#[derive(Serialize, Deserialize, Default)]
pub struct CanisterKit {
    pub pause: Pause,             // 记录维护状态 // ? 堆内存 序列化
    pub permissions: Permissions, // 记录自身权限 // ? 堆内存 序列化
    pub schedule: Schedule,       // 记录定时任务 // ? 堆内存 序列化
}

// 能序列化的和不能序列化的放在一起
// 其中不能序列化的采用如下注解
// #[serde(skip)] 默认初始化方式
// #[serde(skip, default="init_xxx_data")] 指定初始化方式
// ! 如果使用 ic-stable-structures 提供的稳定内存，不能变更 memory_id 的使用类型，否则会出现各个版本不兼容，数据会被清空
#[derive(Serialize, Deserialize)]
pub struct InnerState {
    pub canister_kit: CanisterKit, // 框架需要的数据 // ? 堆内存 序列化
}

impl Default for InnerState {
    fn default() -> Self {
        ic_cdk::println!("InnerState::default()");
        Self {
            canister_kit: Default::default(),
        }
    }
}

impl InnerState {
    pub fn do_init(&mut self, _arg: InitArg) {
        // maybe do something
    }

    pub fn do_upgrade(&mut self, _arg: UpgradeArg) {
        // maybe do something
    }
}
