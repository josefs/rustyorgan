#![feature(core)]

use std::boxed::FnBox;

#[test]
fn it_works() {
}

pub enum Src<A> {
  Nil,
  Cons(A,Box<FnBox(Snk<A>)>)
}

pub enum Snk<A> {
  Full,
  Cont(Box<FnBox(Src<A>)>)
}

pub type Source<A> = Box<FnBox(Snk<A>)>;
pub type Sink<A>   = Box<FnBox(Src<A>)>;

pub fn map_src<A,B>(f : Box<Fn(A) -> B>, s : Src<A>) -> Src<B> {
  match s {
    Src::Nil => Src::Nil,
    Src::Cons (x,xs) => Src::Cons(f(x),Box::new(move |snk| xs(comap_snk(f,snk))))
  }
}

pub fn comap_snk<A,B>(f : Box<Fn(A) -> B>, s : Snk<B>) -> Snk<A> {
  match s {
    Snk::Full => Snk::Full,
    Snk::Cont(xs) => Snk::Cont(Box::new(move |src| xs(map_src(f,src))))
  }
}

pub fn map_source<A,B>(f : Box<Fn(A) -> B>, s : Source<A>) -> Source<B> {
  Box::new(|snk| match snk {
    Snk::Full => s(Snk::Full),
    Snk::Cont(snk) => s(Snk::Cont(comap_sink(f,snk)))
  })
}

pub fn comap_sink<A,B>(f : Box<Fn(A) -> B>, s : Sink<B>) -> Sink<A> {
  Box::new(|src| match src {
    Src::Nil => s(Src::Nil),
    Src::Cons(x,xs) => s(Src::Cons(f(x),map_source(f,xs)))
  })
}

pub fn enum_from(x : i32) -> Source<i32> {
  Box::new(move |src| match src {
    Snk::Full => return,
    Snk::Cont(snk) => snk(Src::Cons(x,enum_from(x+1)))
  })
}

// pub fn file(

/*
pub fn enum(start : i32, stop : i32) -> Src<i32> {
  if start > stop {
    Src::Nil
  } else {
    Src::Cons(start,Box::new(|snk|
      match snk {
        Snk::Full => 
      }
    ))
  }
}

pub fn enum(snk : Snk<i32>) {
  match snk {
    Full :: 
  }
}
*/
