#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("demo/include/test.hpp");
        type Complex;
        fn new_comp(re: f64, im: f64) -> UniquePtr<Complex>;
        fn get_re(&self) -> f64;
        fn get_im(&self) -> f64;
    }
}

fn main() {
    let x = ffi::new_comp(0.12, 0.54);
    dbg!(x.get_re());
}
