--- cargo-crates/libc-0.2.177/src/unix/bsd/freebsdlike/mod.rs.orig	2006-07-24 01:21:28 UTC
+++ cargo-crates/libc-0.2.177/src/unix/bsd/freebsdlike/mod.rs
@@ -875,7 +875,21 @@ pub const AF_NETGRAPH: c_int = 32;
 pub const AF_ATM: c_int = 30;
 pub const pseudo_AF_HDRCMPLT: c_int = 31;
 pub const AF_NETGRAPH: c_int = 32;
+pub const AF_NETLINK: c_int = 38;
+pub const NETLINK_PKTINFO: c_int = 3;
+pub const NETLINK_GET_STRICT_CHK: c_int = 12;
+pub const SOL_NETLINK: c_int = 270;
+pub const NETLINK_EXT_ACK: c_int = 11;
+pub const NETLINK_CAP_ACK: c_int = 10;
+pub const NETLINK_LISTEN_ALL_NSID: c_int = 8;
+pub const NETLINK_NO_ENOBUFS: c_int = 5;
+pub const NETLINK_BROADCAST_ERROR: c_int = 4;
+pub const NETLINK_DROP_MEMBERSHIP: c_int = 2;
+pub const NETLINK_ADD_MEMBERSHIP: c_int = 1;
+pub const ARPHRD_ETHER: c_int = 1;
+pub const ETH_ALEN: c_int =6;
 
+pub const PF_NETLINK: c_int = AF_NETLINK;
 pub const PF_UNSPEC: c_int = AF_UNSPEC;
 pub const PF_LOCAL: c_int = AF_LOCAL;
 pub const PF_UNIX: c_int = PF_LOCAL;
