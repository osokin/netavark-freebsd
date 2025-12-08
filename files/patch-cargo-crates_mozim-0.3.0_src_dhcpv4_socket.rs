--- cargo-crates/mozim-0.3.0/src/dhcpv4/socket.rs.orig	2006-07-24 01:21:28 UTC
+++ cargo-crates/mozim-0.3.0/src/dhcpv4/socket.rs
@@ -173,6 +173,7 @@ impl DhcpV4Socket for DhcpRawSocket {
     }
 }
 
+#[cfg(target_os = "linux")]
 fn create_raw_eth_socket() -> Result<OwnedFd, DhcpError> {
     nix::sys::socket::socket(
         AddressFamily::Packet,
@@ -188,6 +189,22 @@ fn create_raw_eth_socket() -> Result<OwnedFd, DhcpErro
     })
 }
 
+#[cfg(not(target_os = "linux"))]
+fn create_raw_eth_socket() -> Result<OwnedFd, DhcpError> {
+    nix::sys::socket::socket(
+        AddressFamily::Inet,
+        SockType::Raw,
+        SockFlag::SOCK_NONBLOCK,
+        None,
+    )
+    .map_err(|e| {
+        DhcpError::new(
+            ErrorKind::Bug,
+            format!("Failed to create raw ethernet socket: {e}"),
+        )
+    })
+}
+
 fn bind_raw_socket(
     fd: RawFd,
     eth_protocol: libc::c_int,
@@ -277,6 +294,7 @@ impl DhcpV4Socket for DhcpUdpV4Socket {
     }
 }
 
+#[cfg(target_os = "linux")]
 fn bind_socket_to_iface(fd: RawFd, iface_name: &str) -> Result<(), DhcpError> {
     let iface_name_cstr = CString::new(iface_name)?;
 
@@ -299,5 +317,12 @@ fn bind_socket_to_iface(fd: RawFd, iface_name: &str) -
             ));
         }
     }
+    Ok(())
+}
+
+#[cfg(not(target_os = "linux"))]
+fn bind_socket_to_iface(fd: RawFd, iface_name: &str) -> Result<(), DhcpError> {
+    let iface_name_cstr = CString::new(iface_name)?;
+
     Ok(())
 }
