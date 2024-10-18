# install rust using recomended command
echo 'Installing Rust...'
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

echo 'Installing build dependencies'
# https://docs.rs/openssl/latest/openssl/#automatic
sudo dnf install pkgconf perl-FindBin perl-IPC-Cmd openssl-devel
source ~/.bashrc

echo 'Building PDE CLI...'
cargo build --release
sudo cp target/release/pde /usr/bin/pde
