use array__ops::{slice_ops, ArrayOps};

use super::*;

pub const fn sum_len<const SEGMENT_LENGTHS: &'static [usize]>() -> usize
{
    let mut i = 0;
    let mut len = 0;
    while i < SEGMENT_LENGTHS.len()
    {
        len += SEGMENT_LENGTHS[i];
        i += 1;
    }
    len
}

pub const fn sum_len_eq<const A: &'static [usize], const B: &'static [usize]>() -> bool
{
    sum_len::<{A}>() == sum_len::<{B}>()
}

pub const fn all_len_eq<const A: &'static [usize], const B: &'static [usize]>() -> bool
{
    let len = A.len();
    if len != B.len()
    {
        return false;
    }

    let mut i = 0;
    while i < len
    {
        if A[i] != B[i]
        {
            return false;
        }
        i += 1;
    }

    true
}

pub struct ArraySegments<T, const SEGMENT_LENGTHS: &'static [usize]>([T; sum_len::<{SEGMENT_LENGTHS}>()])
where
    [(); sum_len::<{SEGMENT_LENGTHS}>()]:;

impl<T, const SEGMENT_LENGTHS: &'static [usize]> ArraySegments<T, SEGMENT_LENGTHS>
where
    [(); sum_len::<{SEGMENT_LENGTHS}>()]:,
    [(); SEGMENT_LENGTHS.len()]:
{
    pub const SEGMENTS: usize = SEGMENT_LENGTHS.len();
    pub const SEGMENT_LENGTHS: [usize; SEGMENT_LENGTHS.len()] = *slice_ops::split_array_ref(SEGMENT_LENGTHS).0;
    pub const SERIALIZED_LENGTH: usize = sum_len::<{SEGMENT_LENGTHS}>();
    
    /*pub const fn split_lengths_left(mid: usize) -> &'static [usize]
    {
        Self::PART_LENGTHS.split_at(mid).0
    }
    pub const fn split_lengths_right(mid: usize) -> &'static [usize]
    {
        Self::PART_LENGTHS.split_at(mid).1
    }*/
    pub const fn split_lengths(mid: usize) -> (&'static [usize], &'static [usize])
    {
        Self::SEGMENT_LENGTHS.split_at(mid)
    }

    /*pub fn split_parts<const N: usize>(self) -> (PartitionedArray<T, {Self::split_lengths_left(N)}>, PartitionedArray<T, {Self::split_lengths_right(N)}>)
    where
        [(); sum_len::<{Self::split_lengths_left(N)}>()]:,
        [(); sum_len::<{Self::split_lengths_right(N)}>()]:
    {
        unsafe {
            private::split_transmute(self)
        }
    }*/

    pub const fn new(array: [T; sum_len::<SEGMENT_LENGTHS>()]) -> Self
    {
        Self(array)
    }

    pub const fn serialize_arrays(self) -> [T; sum_len::<{SEGMENT_LENGTHS}>()]
    {
        unsafe {private::transmute_unchecked_size(self)}
    }
    pub const fn as_serialize_arrays(&self) -> &[T; sum_len::<{SEGMENT_LENGTHS}>()]
    {
        &self.0
    }
    pub const fn as_serialize_arrays_mut(&mut self) -> &mut [T; sum_len::<{SEGMENT_LENGTHS}>()]
    {
        &mut self.0
    }
    
    pub const fn as_ptr(&self) -> *const T
    {
        self.0.as_ptr()
    }
    pub const fn as_mut_ptr(&mut self) -> *mut T
    {
        self.0.as_mut_ptr()
    }

    pub fn each_offset() -> [usize; SEGMENT_LENGTHS.len()]
    {
        let mut o = Self::SEGMENT_LENGTHS;
        o.integrate();
        o.sub_assign_each(Self::SEGMENT_LENGTHS);
        o
    }
    pub fn each_ptr(&self) -> [*const T; SEGMENT_LENGTHS.len()]
    {
        let ptr = self.as_ptr();
        
        Self::each_offset()
            .map2(|offset| unsafe {ptr.add(offset)})
    }
    pub fn each_mut_ptr(&mut self) -> [*mut T; SEGMENT_LENGTHS.len()]
    {
        let ptr = self.as_mut_ptr();

        Self::each_offset()
            .map2(|offset| unsafe {ptr.add(offset)})
    }

    pub fn each_slice(&self) -> [&[T]; SEGMENT_LENGTHS.len()]
    {
        self.each_ptr()
            .comap(Self::SEGMENT_LENGTHS, |ptr, len| unsafe {core::slice::from_raw_parts(ptr, len)})
    }
    pub fn each_slice_mut(&mut self) -> [&mut [T]; SEGMENT_LENGTHS.len()]
    {
        self.each_mut_ptr()
            .comap(Self::SEGMENT_LENGTHS, |ptr, len| unsafe {core::slice::from_raw_parts_mut(ptr, len)})
    }

    /*pub fn get_array<const I: usize>(&self) -> &[T; Self::PART_LENGTHS[I]]
    where
        [(); Self::PARTS - I]:
    {
        unsafe {
            &*self.each_ptr()[I].cast()
        }
    }*/

    pub fn get_slice(&self, index: usize) -> Option<&[T]>
    {
        self.each_ptr()
            .zip(Self::SEGMENT_LENGTHS)
            .get(index)
            .map(|&(ptr, len)| unsafe {core::slice::from_raw_parts(ptr, len)})
    }
    pub fn get_slice_mut(&mut self, index: usize) -> Option<&mut [T]>
    {
        self.each_mut_ptr()
            .zip(Self::SEGMENT_LENGTHS)
            .get(index)
            .map(|&(ptr, len)| unsafe {core::slice::from_raw_parts_mut(ptr, len)})
    }

    pub const fn reinterpret_lengths<const S: usize, const P: &'static [usize]>(self) -> ArraySegments<T, {P}>
    where
        [(); sum_len::<{P}>()]:,
        [(); sum_len_eq::<{SEGMENT_LENGTHS}, {P}>() as usize - 1]:
    {
        unsafe {private::transmute_unchecked_size(self)}
    }
    pub const fn reinterpret_lengths_ref<const S: usize, const P: &'static [usize]>(&self) -> &ArraySegments<T, {P}>
    where
        [(); sum_len::<{P}>()]:,
        [(); sum_len_eq::<{SEGMENT_LENGTHS}, {P}>() as usize - 1]:
    {
        unsafe {core::mem::transmute(self)}
    }
    pub const fn reinterpret_lengths_mut<const S: usize, const P: &'static [usize]>(&mut self) -> &mut ArraySegments<T, {P}>
    where
        [(); sum_len::<{P}>()]:,
        [(); sum_len_eq::<{SEGMENT_LENGTHS}, {P}>() as usize - 1]:
    {
        unsafe {core::mem::transmute(self)}
    }
    
    pub const fn reformulate_lengths<const S: usize, const P: &'static [usize]>(self) -> ArraySegments<T, {P}>
    where
        [(); sum_len::<{P}>()]:,
        [(); all_len_eq::<{SEGMENT_LENGTHS}, {P}>() as usize - 1]:
    {
        unsafe {private::transmute_unchecked_size(self)}
    }
    pub const fn reformulate_lengths_ref<const S: usize, const P: &'static [usize]>(&self) -> &ArraySegments<T, {P}>
    where
        [(); sum_len::<{P}>()]:,
        [(); all_len_eq::<{SEGMENT_LENGTHS}, {P}>() as usize - 1]:
    {
        unsafe {core::mem::transmute(self)}
    }
    pub const fn reformulate_lengths_mut<const S: usize, const P: &'static [usize]>(&mut self) -> &mut ArraySegments<T, {P}>
    where
        [(); sum_len::<{P}>()]:,
        [(); all_len_eq::<{SEGMENT_LENGTHS}, {P}>() as usize - 1]:
    {
        unsafe {core::mem::transmute(self)}
    }
}

impl<'a, T, const PART_LENGTHS: &'static [usize]> IntoIterator for &'a ArraySegments<T, PART_LENGTHS>
where
    [(); sum_len::<{PART_LENGTHS}>()]:,
    [(); PART_LENGTHS.len()]:
{
    type Item = &'a [T];
    type IntoIter = <[&'a [T]; PART_LENGTHS.len()] as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter
    {
        self.each_slice().into_iter()
    }
}

impl<'a, T, const PART_LENGTHS: &'static [usize]> IntoIterator for &'a mut ArraySegments<T, PART_LENGTHS>
where
    [(); sum_len::<{PART_LENGTHS}>()]:,
    [(); PART_LENGTHS.len()]:
{
    type Item = &'a mut [T];
    type IntoIter = <[&'a mut [T]; PART_LENGTHS.len()] as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter
    {
        self.each_slice_mut().into_iter()
    }
}