struct Node<'a> {
    val: i32,
    next: &'a Node<'a>
}

