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
fn test_chunk_up_str() {
    let tests = [
        (1, "", ""),
        (1, "x", "x"),
        (1, "xx", "x x"),
        (1, "xxx", "x x x"),
        (5, "", ""),
        (5, "x", "x"),
        (5, "xxxxx", "xxxxx"),
        (5, "hello_world", "hello _worl d"),
    ];
    for (n, src, target) in tests {
        let chunked: String = src
            .chars()
            .chunk_up(n, ' ')
            .collect();
        assert_eq!(target, &chunked);
    }
}

#[test]
fn test_chunk_up_trid() {
    #[derive(Clone, Debug, PartialEq)]
    enum Trid { X, S }
    use Trid::*;

    let tests = [
        (1, [].as_ref(), [].as_ref()),
        (1, &[X], &[X]),
        (1, &[X, X], &[X, S, X]),
    ];
    for (n, src, target) in tests {
        let chunked: Vec<Trid> = src
            .iter()
            .cloned()
            .chunk_up(n, S)
            .collect();
        assert_eq!(target, &chunked);
    }
}

#[test]
#[should_panic]
fn test_chunk_up_0() {
    let _ = (1..3).chunk_up(0, 0).nth(0);
}
