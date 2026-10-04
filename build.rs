use uniffi::generate_scaffolding;

fn main(){
    generate_scaffolding("./src/lib.udl").unwrap();
}