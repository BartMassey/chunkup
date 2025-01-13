pub struct ChunkUp<I: Iterator> {
    size: usize,
    count: usize,
    separator: I::Item,
    source: I,
    saved: Option<I::Item>,
}

impl<I: Iterator + Sized> ChunkUp<I> {
    fn new(source: I, size: usize, separator: I::Item) -> Self {
        assert!(size > 0, "chunk size must be positive");
        ChunkUp { size, count: 0, separator, source, saved: None }
    }
}

impl<I: Iterator + Sized> Iterator for ChunkUp<I>
where I::Item: Clone
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(result) = self.saved.take() {
            self.count += 1;
            return Some(result);
        }
        let next = self.source.next()?;
        if self.count >= self.size.into() {
            self.count = 0;
            self.saved = Some(next);
            return Some(self.separator.clone());
        }
        self.count += 1;
        Some(next)
    }
}

pub trait ChunkUpExt<I: Iterator + Sized> : Iterator + Sized {
    fn chunk_up(self, size: usize, separator: I::Item) -> ChunkUp<Self>;
}

impl<I: Iterator + Sized> ChunkUpExt<I> for I {
    fn chunk_up(self, size: usize, separator: I::Item) -> ChunkUp<Self> {
        ChunkUp::new(self, size, separator)
    }
}

#[test]
fn test_chunk_up() {
    let tests = [
        ("", ""),
        ("x", "x"),
        ("xxxxx", "xxxxx"),
        ("hello_world", "hello _worl d"),
    ];
    for (src, target) in tests {
        let chunked: String = src
            .chars()
            .chunk_up(5, ' ')
            .collect();
        assert_eq!(target, &chunked);
    }
}
