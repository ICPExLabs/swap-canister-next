use super::super::business::*;
use super::types::*;

impl Business for InnerState {}

#[allow(clippy::panic)] // ? allow rollback
#[allow(clippy::unwrap_used)] // ? allow rollback
#[allow(clippy::expect_used)] // ? allow rollback
impl MutableBusiness for InnerState {}
