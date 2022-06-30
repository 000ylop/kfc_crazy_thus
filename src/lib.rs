const N: usize = 3;

pub const SUN_TIP: &str = "周日随便刷哦";
pub const SPLITTER: &str = "、";

pub mod weapon {
    use crate::N;
    pub const MON_THU: [&'static str; N] = ["孤云寒林", "高塔孤王", "远海夷地"];
    pub const TUE_FRI: [&'static str; N] = ["凛风奔狼", "雾海云间", "鸣神御灵"];
    pub const WED_SAT: [&'static str; N] = ["狮牙斗士", "漆黑陨铁", "今昔剧画"];
}

pub mod talent {
    use crate::N;
    pub const MON_THU: [&'static str; N] = ["自由", "繁荣","浮世"];
    pub const TUE_FRI: [&'static str; N] = ["抗争", "勤劳", "风雅"];
    pub const WED_SAT: [&'static str; N] = ["诗文", "黄金", "天光"];
}
