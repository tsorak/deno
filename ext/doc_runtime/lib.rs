use deno_core::op;

deno_core::extension!(
  deno_doc_runtime,
  ops = [op_doc_runtime],
  esm = ["doc_runtime.js"]
);

#[op]
fn op_doc_runtime() {
  println!("Hello, world!");
}
