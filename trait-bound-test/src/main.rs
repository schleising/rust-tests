use std::fmt::Display;

trait BoundTest<U>
where
    U: Display,
{
    fn test(&self);
}

struct TestStruct<T, U>
where
    T: BoundTest<U>,
    U: Display,
{
    value: T,
    _marker: std::marker::PhantomData<U>,
}

impl <U> BoundTest<U> for i32
where
    U: Display,
{
    fn test(&self) {
        println!("Testing: {self}");
    }
}

fn print_thing<T, U>(thing: TestStruct<T, U>)
where
    T: BoundTest<U>,
    U: Display,
{
    thing.value.test();
}

fn main()
{
    let test = TestStruct { value: 5, _marker: std::marker::PhantomData::<i32> };
    print_thing(test);
}

