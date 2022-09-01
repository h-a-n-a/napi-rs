use napi::bindgen_prelude::*;

#[napi]
pub fn create_external(size: u32) -> External<u32> {
  External::new(size)
}

#[napi]
pub fn create_external_string(content: String) -> External<String> {
  External::new(content)
}

#[napi]
pub fn get_external(external: External<u32>) -> u32 {
  *external
}

#[napi]
pub fn mutate_external(mut external: External<u32>, new_val: u32) {
  *external = new_val
}

#[derive(Debug)]
#[napi(object)]
pub struct A {
  pub b: B,
}

#[derive(Debug)]
#[napi(object)]
pub struct B {
  pub num: u32,
}

#[napi]
pub fn create_external_val() -> External<A> {
  External::new(A { b: B { num: 123 } })
}

#[napi]
pub fn mutate_external_val(external: External<A>) {
  let b = external.as_ref();

  std::thread::spawn(move || {
    b.num += 123;
    println!("{:#?}", b.num);
  });
}
