// Copyright (c) Microsoft Corporation
// License: MIT OR Apache-2.0

//! Build script for the `sample-kmdf-driver` crate.

fn main() -> Result<(), wdk_build::ConfigError> {
    wdk_build::Config::from_env_auto()?.configure_binary_build()
}
