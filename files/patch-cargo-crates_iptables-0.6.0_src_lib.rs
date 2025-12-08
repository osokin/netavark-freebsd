--- cargo-crates/iptables-0.6.0/src/lib.rs.orig	2025-11-15 00:03:31 UTC
+++ cargo-crates/iptables-0.6.0/src/lib.rs
@@ -243,6 +243,7 @@ impl IPTables {
     }
 
     /// Inserts `rule` in the `position` to the table/chain if it does not exist.
+    #[cfg(target_os = "linux")]
     pub fn insert_unique(
         &self,
         table: &str,
@@ -282,6 +283,7 @@ impl IPTables {
     }
 
     /// Appends `rule` to the table/chain if it does not exist.
+    #[cfg(target_os = "linux")]
     pub fn append_unique(
         &self,
         table: &str,
@@ -296,6 +298,7 @@ impl IPTables {
     }
 
     /// Appends or replaces `rule` to the table/chain if it does not exist.
+    #[cfg(target_os = "linux")]
     pub fn append_replace(
         &self,
         table: &str,
@@ -316,6 +319,7 @@ impl IPTables {
     }
 
     /// Deletes all repetition of the `rule` from the table/chain.
+    #[cfg(target_os = "linux")]
     pub fn delete_all(&self, table: &str, chain: &str, rule: &str) -> Result<(), Box<dyn Error>> {
         while self.exists(table, chain, rule)? {
             self.delete(table, chain, rule)?;
