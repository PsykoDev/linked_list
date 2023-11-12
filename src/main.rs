extern crate linked_list;
use linked_list::LinkedList;


fn main() {
    let mut list = LinkedList::from_iter(vec![8, 5, 1]);

    list.print_memory_layout();
    println!();

    println!("Size of the list: {}", list.get_size());
    println!("Type of the elements: {}", list.get_element_type());

    println!("\nIterating over the list:");
    for item in list.iter() {
        println!("{}", item);
    }

    println!("\nSorting the list:");
    list.sorting();

    println!("\nPopping the last element:");
    let popped = list.pop();
    if let Some(value) = popped {
        println!("Popped: {}", value);
    }

    println!("\nIterating over the list:");
    for item in list.iter() {
        println!("{}", item);
    }

    println!("Size of the list: {}", list.get_size());

    println!("\nClearing the list:");
    list.clear();

    println!("Size of the list: {}", list.get_size());

}