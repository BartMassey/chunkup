pub struct ChunkUp<I: Iterator> {
    size: usize,
    count: usize,
    separator: I::Item,
    source: I,
}

impl<I: Iterator + Sized> ChunkUp<I> {
    fn new(source: I, size: usize, separator: I::Item) -> Self {
        ChunkUp { size, count: 0, separator, source }
    }
}

impl<I: Iterator + Sized> Iterator for ChunkUp<I>
where I::Item: Clone
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count > self.size {
            self.count = 0;
            return Some(self.separator.clone());
        }
        self.source.next()
    }
}

pub trait ChunkUpExt<I: Iterator + Sized> : Iterator + Sized {
    fn chunk_up(self, count: usize, separator: I::Item) -> ChunkUp<Self>;
}

impl<I: Iterator + Sized> ChunkUpExt<I> for I {
    fn chunk_up(self, count: usize, separator: I::Item) -> ChunkUp<Self> {
        ChunkUp::new(self, count, separator)
    }
}

#[test]
fn test_chunk_up() {
    let chunked: String = "hello_world"
        .chars()
        .chunk_up(5, ' ')
        .collect();
    assert_eq!(&chunked, "hello _worl d");
}
