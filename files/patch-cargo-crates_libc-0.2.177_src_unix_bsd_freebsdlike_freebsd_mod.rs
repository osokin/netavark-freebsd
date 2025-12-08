--- cargo-crates/libc-0.2.177/src/unix/bsd/freebsdlike/freebsd/mod.rs.orig	2025-11-14 20:42:56 UTC
+++ cargo-crates/libc-0.2.177/src/unix/bsd/freebsdlike/freebsd/mod.rs
@@ -1453,6 +1453,14 @@ s_no_extra_traits! {
         pub sdl_data: [c_char; 46],
     }
 
+    pub struct sockaddr_nl {
+        pub nl_len: u8,
+        pub nl_family: crate::sa_family_t,
+        pub nl_pad: u16,
+        pub nl_pid: u32,
+        pub nl_groups: u32,
+    }
+
     pub struct mq_attr {
         pub mq_flags: c_long,
         pub mq_maxmsg: c_long,
