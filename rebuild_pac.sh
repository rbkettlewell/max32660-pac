mkdir -p temp/build
svd2rust -i max32660.svd -o temp
form -i temp/lib.rs -o temp/build  && rm temp/lib.rs
cp -r temp/build/* src
rm -rf temp
cargo fmt