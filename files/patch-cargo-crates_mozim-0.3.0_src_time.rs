--- cargo-crates/mozim-0.3.0/src/time.rs.orig	2025-11-14 21:21:38 UTC
+++ cargo-crates/mozim-0.3.0/src/time.rs
@@ -5,7 +5,7 @@ use nix::sys::{
 use nix::sys::{
     time::TimeSpec,
     timerfd::{
-        ClockId::CLOCK_BOOTTIME, Expiration, TimerFd, TimerFlags,
+        ClockId::CLOCK_UPTIME, Expiration, TimerFd, TimerFlags,
         TimerSetTimeFlags,
     },
 };
@@ -13,7 +13,7 @@ use crate::{DhcpError, ErrorKind};
 
 use crate::{DhcpError, ErrorKind};
 
-/// Timer depend on CLOCK_BOOTTIME so it continue ticks when system
+/// Timer depend on CLOCK_UPTIME so it continue ticks when system
 /// sleeps/hibernates.
 #[derive(Debug)]
 pub(crate) struct DhcpTimer {
@@ -29,7 +29,7 @@ impl DhcpTimer {
     pub(crate) async fn wait(&self) -> Result<(), DhcpError> {
         let remains = self.remains()?;
         if !remains.is_zero() {
-            let fd = TimerFd::new(CLOCK_BOOTTIME, TimerFlags::TFD_NONBLOCK)
+            let fd = TimerFd::new(CLOCK_UPTIME, TimerFlags::TFD_NONBLOCK)
                 .map_err(|e| {
                     let e = DhcpError::new(
                         ErrorKind::Bug,
@@ -72,10 +72,10 @@ fn boot_time_now() -> Result<TimeSpec, DhcpError> {
 }
 
 fn boot_time_now() -> Result<TimeSpec, DhcpError> {
-    nix::time::clock_gettime(nix::time::ClockId::CLOCK_BOOTTIME).map_err(|e| {
+    nix::time::clock_gettime(nix::time::ClockId::CLOCK_UPTIME).map_err(|e| {
         DhcpError::new(
             ErrorKind::Bug,
-            format!("Failed to retrieve CLOCK_BOOTTIME: {e}"),
+            format!("Failed to retrieve CLOCK_UPTIME: {e}"),
         )
     })
 }
