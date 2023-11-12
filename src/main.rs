extern crate linked_list;
use linked_list::LinkedList;


fn main() {
    let mut x = LinkedList::new();
    x.push(1);
    x.push(2);
    x.push(3);
    x.push(4);
    x.push(5);

    let mut list = LinkedList::from_iter(x.into_iter());

    let mut list2 = LinkedList::from(&[1, 2, 3, 4, 5]);

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