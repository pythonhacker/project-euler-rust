"""
Build a Cargo.toml from the list of *.rs files here

"""

import glob

template="""
[package]
name = "euler-rust"
version = "0.1.0"
authors = ["Anand"]

[dependencies]
num-bigint = "0.2.6"
num-traits = "0.2.11"
{}
"""

bin_template="""
[[bin]]
name = "{}"
path = "{}"
"""

def build_cargo_toml():
    """ Build a Cargo.toml from problem*.rs files in current folder """

    targets = []
    
    for path in sorted(glob.glob("problem*.rs")):
        print("Adding",path)
        path_base = path.rsplit('.')[0]
        bin_target = bin_template.format(path_base, path)
        targets.append(bin_target)

    open("Cargo.toml","w").write(template.format("".join(targets)))

if __name__ == "__main__":
    build_cargo_toml()
