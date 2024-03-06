

default:
	svd2rust -i max32655.svd
	rm -rf src
	form -i lib.rs -o src/ && rm lib.rs
	cargo fmt
