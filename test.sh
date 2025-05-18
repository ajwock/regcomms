set -xe

cargo run quantum_flux_sensor.yaml quantum_flux_sensor
pushd ./quantum_flux_sensor
# This makes quantumfluxsensor ignore the parent workspace
printf "\n[workspace]" >> ./Cargo.toml
cargo build
popd
pushd ./test_crate
cargo test
