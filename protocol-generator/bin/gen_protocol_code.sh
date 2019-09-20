#!/bin/bash

current_dir="$( cd "$( dirname "${BASH_SOURCE[0]}" )/.." && pwd )";
cd ${current_dir}

# Fetch proto files
if [[ ! -d proto ]]; then
    git clone git@bitbros.com:/seriesdb-protocol proto;
fi

output_dir="../src/protocol"
mkdir -p ${output_dir}

touch ${output_dir}/mod.rs
echo "
mod seriesdb_protocol;
mod seriesdb_protocol_ext;

pub use seriesdb_protocol::*;
pub use seriesdb_protocol_ext::*;
" > ${output_dir}/mod.rs

# Generate xxx file by protoc
cargo run && mv ${output_dir}/seriesdb.protocol.rs ${output_dir}/seriesdb_protocol.rs

# Generate xxx_ext file
bin/gen_protocol_ext.py \
    --proto_file proto/seriesdb_protocol.proto \
    --enum_type_name MsgType
