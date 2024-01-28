#![feature(const_mut_refs)]
#![feature(const_refs_to_cell)]

#![feature(adt_const_params)]
#![feature(generic_const_exprs)]

moddef::moddef!(
    flat(pub) mod {
        array_segments_
    },
    mod {
        private
    }
);

#[cfg(test)]
mod tests
{
    use array__ops::ArrayOps;

    use crate::{ArraySegments};

    #[test]
    fn it_works()
    {
        let tuple = ([1u8, 2], [3u8, 4, 5], [6u8, 7]);

        let partition = ArraySegments::<u8, {&[2usize, 3, 2]}>::new(tuple.0.chain(tuple.1).chain(tuple.2));
    
        assert_eq!(Some(tuple.0.as_slice()), partition.get_slice(0));
        assert_eq!(Some(tuple.1.as_slice()), partition.get_slice(1));
        assert_eq!(Some(tuple.2.as_slice()), partition.get_slice(2));

        println!("o = {:?}", ArraySegments::<u8, {&[2usize, 3, 2]}>::each_offset());
        println!("a = {:?}", partition.each_slice());
    }
}