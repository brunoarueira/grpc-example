fn main() {
    tonic_build::compile_protos("proto/calculator.proto").unwrap();
}
