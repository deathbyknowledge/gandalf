
fn main() {
  capnpc::CompilerCommand::new()
    .src_prefix("cereal")
    .file("cereal/page.capnp")
    .default_parent_module(vec!["schema".into()])
    .run().expect("schema compiler command");
}
