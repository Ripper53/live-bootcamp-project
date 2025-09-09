#!/bin/sh

cat > src/auth/constants.rs << EOF
pub const JWT_SECRET: &str = "${JWT_SECRET}";
EOF
