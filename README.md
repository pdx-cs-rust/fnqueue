fnqueue: "functional-style" queue
Bart Massey 2023-10-20

This library crate implements a queue (FIFO) using the
"functional style": vectors are used to hold the head and
tail of the queue, with elements moving between them as
needed. This gives amortized *O(1)* insertion and removal.

# License

This work is licensed under the "MIT License". Please see the file
`LICENSE.txt` in this distribution for license terms.
