#![windows_subsystem = "windows"]

mod quack;

use windows::ApplicationModel::Activation::{ActivationKind, ShareTargetActivatedEventArgs};
use windows::core::Interface;
use crate::quack::quack;

fn main() -> windows::core::Result<()> {
    let args = windows::ApplicationModel::AppInstance::GetActivatedEventArgs()?;
    let kind = args.Kind()?;

    if kind == ActivationKind::ShareTarget {
        let share_args: ShareTargetActivatedEventArgs = args.cast()?;
        let share_op = share_args.ShareOperation()?;
        share_op.ReportStarted()?;
        unsafe {
            quack();
        }
        share_op.ReportCompleted()?;
    } else {
        unsafe {
            quack();
        }
    }
    Ok(())
}