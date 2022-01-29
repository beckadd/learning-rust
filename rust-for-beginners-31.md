# Common Collection Datatypes in Rust

Several collection types in Rust

Vec<T> -> stores new items always at end

Good default choice for lists of items; indexable with O(1)

HashMap<K, V> -> equivalent to typed python dict, java hashmap

Good for associating a data object with some key for O(1) lookup performance.

HashSet<T> -> a mathematical set

Good for uniqueness, but no access by index

VecDeque<T> -> good for setting to front *and* back of the collection

A queue

LinkedList<T> -> Not often used; doubly-linked list

We been knew when linked lists are good.

Other collections available.

**SLICES** are references to continuous chunks of memory, written as &[SLICE INDEX]
composed of two parts, a length and datapointer.
