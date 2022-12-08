use std::collections::HashMap;
use std::env;
use std::fs;
// use std::rc::Weak;

enum IType {
    File,
    Dir,
}

struct Inode {
    name: String,
    itype: IType,
    stuff: Vec<String>,
    size: u32,
}

impl Inode {
    pub fn cd(self: &mut Self, path: &str, inodes: &mut HashMap<String, Inode>) -> String {
        if path == ".." {
            return self.name.rsplit_once("/").unwrap().0.to_string();
        }
        let subdir = format!("{}/{path}", self.name);
        inodes.entry(subdir.clone()).or_insert(Inode {
            name: subdir.clone(),
            itype: IType::Dir,
            stuff: Vec::new(),
            size: 0,
        });
        self.stuff.push(subdir.clone());
        subdir
    }

    pub fn du(self: &Self, inodes: &HashMap<String, Inode>) -> u32 {
        let mut size = 0;
        for path in self.stuff.iter() {
            let inode = inodes.get(path).unwrap();
            match inode.itype {
                IType::File => { size += inodes.get(path).unwrap().size },
                IType::Dir => { size += inodes.get(path).unwrap().du(inodes) },
            }
        }
        size
    }

    pub fn touch(self: &mut Self, path: &str, size: u32, inodes: &mut HashMap<String, Inode>) {
        let filename = format!("{}/{path}", self.name);
        inodes.insert(filename.clone(), Inode {
            name: filename.clone(),
            itype: IType::File,
            stuff: Vec::new(),
            size: size
        });
        self.stuff.push(filename);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let input = fs::read_to_string(path).unwrap();

    let mut inodes: HashMap<String, Inode> = HashMap::new();
    let mut total_path: Vec<String> = vec![];

    let mut lines = input.split("\r\n");
    loop {
        let line = lines.next();
        if line == None {
            break;
        }
        let line = line.unwrap();
        println!("{line}");
        if line == "$ cd /" {
            inodes.insert("/".to_string(), Inode {
                name: "/".to_string(),
                itype: IType::Dir,
                stuff: vec![],
                size: 0,
            });
            total_path.push("/".to_string());
        } else if line == "$ cd .." {
            total_path.pop();
        } else if line.starts_with("$ cd ") {
            let dirname = line.rsplit_once(" ").unwrap().1;
            let cwd = total_path.last().unwrap();
            let cwd_inode = inodes.get_mut(cwd).unwrap();
            let path = cwd_inode.cd(dirname, &mut inodes);
        }
    }
}
