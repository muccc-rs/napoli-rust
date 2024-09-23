#!/usr/bin/env bash

exec > >(tee -a /home/user/apps/webhook/deploy-napoli/output.log) 2>&1

an_napoli_server=napoli-server-linux-x64
an_napoli_pain=napoli-pain-wasm.tar.gz

# Change to a clean working dir
owd=$(echo $PWD)
date_stamp=$(date +"%F-%H-%M-%S")
wd=$owd/deploy-$date_stamp
mkdir -p $wd && cd $wd

# Download release artifacts from github
url=$(curl -s https://api.github.com/repos/muccc-rs/napoli-rust/releases/latest | jq -r ".assets[] | select(.name | test(\"$an_napoli_pain\")) | .browser_download_url"); wget -q "$url"

# Redeploy napoli-pain
np_webroot=/var/www/virtual/user/pizza.website.com/
tar -xzf $an_napoli_pain
rsync -vaxSHAX --delete dist/ $np_webroot

# Redeploy napoli-server
ns_bin=/home/user/apps/napoli-server/napoli-server

#url=$(curl -s https://api.github.com/repos/git-commit/napoli-rust/releases/latest | jq -r ".assets[] | select(.name | test(\"$an_napoli_server\")) | .browser_download_url"); wget -q "$url"
#mv $an_napoli_server $ns_bin

cd /home/user/dev/napoli-rust
git reset --hard HEAD
git pull

if cargo build --release --bin napoli-server
then
    echo "First build successful"
else
    cargo clean
    if cargo build --release --bin napoli-server
    then
        echo "Second build successful"
    else
        echo "Second build NOT successful"
        exit 1
    fi
fi


supervisorctl stop napoli-server
cp /home/user/dev/napoli-rust/target/release/napoli-server $ns_bin
chmod +x $ns_bin
supervisorctl start napoli-server

# Cleanup
rm -rf $wd
