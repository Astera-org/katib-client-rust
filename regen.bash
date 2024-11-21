#!/usr/bin/env bash

# This generates Rust code for the Katib API.

set -euo pipefail
set -o xtrace

cd -P -- "$(dirname -- "${BASH_SOURCE[0]}")"

VERSION="0.17.0"

docker run --rm --network=host -u "$(id -u):$(id -g)" -v ${PWD}:/local openapitools/openapi-generator-cli:v5.3.1 \
    generate \
    -i https://raw.githubusercontent.com/kubeflow/katib/refs/tags/v${VERSION}/pkg/apis/v1beta1/swagger.json \
    -g rust \
    -o /local \
    --additional-properties=packageName=katib,packageVersion=${VERSION}

# Delete the client API code because it fails to compile, see https://github.com/OpenAPITools/openapi-generator/issues/20145
rm -r src/apis
sed -i '' '/pub mod apis;/d' "src/lib.rs"
sed -i '' '/extern crate reqwest;/d' "src/lib.rs"
cargo remove reqwest
# Remove empty doc comments
sed -i '' '/^[[:space:]]*\/\/\/[[:space:]]*$/d' src/models/*.rs

# Add missing types
cargo add kubernetes --git https://github.com/Astera-org/kubernetes-client-rust.git --tag v1.31.0
echo "pub(crate) use kubernetes::models::{V1Container, V1HttpGetAction, V1ListMeta, V1ObjectMeta};
pub(crate) use serde_json::Value as V1UnstructuredUnstructured;
pub(crate) use String as V1Time;" >> src/models/mod.rs

# Do not format generated code.
echo 'ignore = ["/"]' > rustfmt.toml

# Remove unnecessary files generated by openapi-generator.
rm -rf .travis.yml swagger.json swagger.json.unprocessed git_push.sh

cargo build

sed -i '' 's/Swagger description for Katib.*/Generated by .\/regen.bash/' README.md