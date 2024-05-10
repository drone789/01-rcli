// 引入模块
mod opts;
mod process;

// 使用 use 导入,
// 使用 pub 方便外部直接使用
pub use opts::{Opts, SubCommand};
pub use process::{process_csv, process_genpass};
