#!/bin/sh

set -eu

usage() {
    cat <<'EOF'
Usage: ./install.sh [--suid]

Options:
  --suid    Install rttmeter and then enable setuid root on /usr/local/bin/rttmeter
EOF
}

enable_suid=false

case "${1-}" in
    "")
        ;;
    --suid)
        enable_suid=true
        ;;
    -h|--help)
        usage
        exit 0
        ;;
    *)
        usage
        exit 1
        ;;
esac

echo "Building release binary..."
cargo build --release

echo "Installing /usr/local/bin/rttmeter..."
sudo install -m 755 target/release/rttmeter /usr/local/bin/rttmeter

if [ "$enable_suid" = true ]; then
    echo "Enabling setuid root on /usr/local/bin/rttmeter..."
    sudo chown root:wheel /usr/local/bin/rttmeter
    sudo chmod 4755 /usr/local/bin/rttmeter
    echo "Installed with setuid root enabled."
else
    echo "Installed with normal permissions."
fi

ls -l /usr/local/bin/rttmeter
