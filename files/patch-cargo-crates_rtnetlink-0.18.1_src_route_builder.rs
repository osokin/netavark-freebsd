--- cargo-crates/rtnetlink-0.18.1/src/route/builder.rs.orig	2006-07-24 01:21:28 UTC
+++ cargo-crates/rtnetlink-0.18.1/src/route/builder.rs
@@ -64,7 +64,7 @@ impl<T> RouteMessageBuilder<T> {
     }
 
     /// Sets the output MPLS encapsulation labels.
-    #[cfg(not(target_os = "android"))]
+    #[cfg(not(target_os = "freebsd"))]
     pub fn output_mpls(mut self, labels: Vec<MplsLabel>) -> Self {
         if labels.is_empty() {
             return self;
@@ -446,7 +446,7 @@ impl Default for RouteMessageBuilder<IpAddr> {
     }
 }
 
-#[cfg(not(target_os = "android"))]
+#[cfg(not(target_os = "freebsd"))]
 impl RouteMessageBuilder<MplsLabel> {
     /// Create default RouteMessage with header set to:
     ///  * route: [RouteHeader::RT_TABLE_MAIN]
@@ -463,7 +463,7 @@ impl RouteMessageBuilder<MplsLabel> {
         builder
     }
 
-    #[cfg(not(target_os = "android"))]
+    #[cfg(not(target_os = "freebsd"))]
     /// Sets the destination MPLS label.
     pub fn label(mut self, label: MplsLabel) -> Self {
         self.message.header.address_family = AddressFamily::Mpls;
@@ -482,7 +482,7 @@ impl RouteMessageBuilder<MplsLabel> {
         self
     }
 }
-#[cfg(not(target_os = "android"))]
+#[cfg(not(target_os = "freebsd"))]
 impl Default for RouteMessageBuilder<MplsLabel> {
     fn default() -> Self {
         Self::new()
@@ -530,7 +530,7 @@ impl RouteNextHopBuilder {
             (AddressFamily::Inet, IpAddr::V6(v6)) => {
                 RouteAttribute::Via(RouteVia::Inet6(v6))
             }
-            #[cfg(not(target_os = "android"))]
+            #[cfg(not(target_os = "freebsd"))]
             (AddressFamily::Mpls, _) => RouteAttribute::Via(addr.into()),
             (af, _) => return Err(InvalidRouteMessage::AddressFamily(af)),
         };
@@ -548,7 +548,7 @@ impl RouteNextHopBuilder {
     }
 
     /// Sets the nexthop MPLS encapsulation labels.
-    #[cfg(not(target_os = "android"))]
+    #[cfg(not(target_os = "freebsd"))]
     pub fn mpls(mut self, labels: Vec<MplsLabel>) -> Self {
         if labels.is_empty() {
             return self;
