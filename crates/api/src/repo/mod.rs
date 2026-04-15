//! DB アクセスを集約するモジュール。
//!
//! 方針:
//! - ハンドラは SQL を直接書かない。必ず repo を経由する。
//! - ユーザースコープのクエリは関数シグネチャで `user_id` を必須パラメータにする。
//!   これにより "WHERE user_id を書き忘れる" 事故を型で防ぐ。
//! - トランザクションを跨ぐ複合操作は repo 関数内で閉じる（呼び出し側は意識しない）。

pub mod portfolio;
pub mod profile;
pub mod session;
pub mod user;
