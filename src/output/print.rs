use crate::model::directory_node::DirectoryNode;

pub fn print_tree(node: &DirectoryNode, indent: usize) {
    let indent_str: String = " ".repeat(indent);
    println!("{}{}", indent_str, node.name);

    for file in &node.files {
        println!("{}  {}", indent_str, file.name);
    }

    for subdir in &node.subdirectories {
        print_tree(subdir, indent + 2);
    }
}
