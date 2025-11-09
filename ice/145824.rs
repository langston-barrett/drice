#![cfg_attr(not(test), no_std)]
#![feature(generic_const_exprs)]
#![feature(associated_const_equality)]
#![allow(incomplete_features)]

use core::fmt::{self, Write};

type Pixel = Option<char>;


pub struct Canvas<const WIDTH: usize, const HEIGHT: usize>(pub [[Pixel; WIDTH]; HEIGHT]);

impl<const WIDTH: usize, const HEIGHT: usize> Canvas<WIDTH, HEIGHT> {
    pub fn new(pixel: Pixel) -> Self {
        Self([[pixel; WIDTH]; HEIGHT])
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> Default for Canvas<WIDTH, HEIGHT> {
    fn default() -> Self {
        Self::new(None)
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> fmt::Display for Canvas<WIDTH, HEIGHT> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.0 {
            for pixel in row {
                f.write_char(pixel.unwrap_or(' '))?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

trait Widget {
    const WIDTH: usize;
    const HEIGHT: usize;

    fn render(&mut self) -> Canvas<{ Self::WIDTH }, { Self::HEIGHT }>;
}

pub struct Box<const WIDTH: usize, const HEIGHT: usize, T> {
    border: Pixel,
    inner: T,
}

impl<const WIDTH: usize, const HEIGHT: usize, T> Box<WIDTH, HEIGHT, T> {
    pub fn new(inner: T) -> Self
    where
        T: Widget<WIDTH = { WIDTH - 2 }, HEIGHT = { HEIGHT - 2 }>,
    {
        Self {
            border: Pixel::default(),
            inner,
        }
    }

    fn empty<const X: usize, const Y: usize>() -> Box<X, Y, Empty<{ X - 2 }, { Y - 2 }>> {
        Box::new(Empty)
    }
}

pub struct Empty<const WIDTH: usize, const HEIGHT: usize>;

impl<const WIDTH: usize, const HEIGHT: usize> Widget for Empty<WIDTH, HEIGHT> {
    const WIDTH: usize = WIDTH;
    const HEIGHT: usize = HEIGHT;

    fn render(&mut self) -> Canvas<{ Self::WIDTH }, { Self::HEIGHT }> {
        Canvas::default()
    }
}

pub struct HBox<const WIDTH: usize, const HEIGHT: usize, T> {
    inner: T,
}

impl<const WIDTH: usize, const HEIGHT: usize, T> HBox<WIDTH, HEIGHT, T> {
    fn new(inner: T) -> Self
    where
        T: HContainer<WIDTH, HEIGHT>,
    {
        Self { inner }
    }
}

impl<const WIDTH: usize, const HEIGHT: usize, T> Widget for HBox<WIDTH, HEIGHT, T> {
    const WIDTH: usize = WIDTH;
    const HEIGHT: usize = HEIGHT;

    fn render(&mut self) -> Canvas<{ Self::WIDTH }, { Self::HEIGHT }> {
        let mut canvas = Canvas::default();
        canvas
    }
}


trait HContainer<const WIDTH: usize, const HEIGHT: usize> {}

impl<const WIDTH: usize, const HEIGHT: usize, T, S> HContainer<WIDTH, HEIGHT> for (T, S)
where
    T: Widget,
    S: Widget,
{
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_foo() {
        let w = HBox::<20, 4, _>::new((
            Box::<9, 2, _>::new(Empty::<7, 0>).border(Some('#')),
            Box::<9, 2, _>::new(Empty::<7, 0>).border(Some('#')),
        ));

        println!("{}", w.render());
        panic!();
    }
}