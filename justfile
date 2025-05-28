test:
    cargo test 
    cargo test -p bsalign
    cargo test -p bsalign_sys

publish:
    cargo publish --registry crates-io -p bsalign_sys
    cargo publish --registry crates-io -p bsalign
