fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("protos/rate_limiter.proto")?;
    Ok(())
}
