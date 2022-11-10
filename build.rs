use ethers::prelude::MultiAbigen;

fn main()
{
    std::fs::remove_dir_all("./adapters").unwrap();
    let _ = MultiAbigen::from_json_files("./abi/")
        .unwrap()
        .build()
        .unwrap()
        .write_to_crate("adapters", "0.1.0", "./adapters", false);

    println!("cargo:rerun-if-changed=abi");
}
