// iterator pattern allows you to iterate over sequesnce of element regardless of how they are stored (vec, hashmaps)
fn main() {
    println!("Hello, world!");

    {
        // methods that consume the iterator
        let mut v = [1,2,3,4];
        // an immutable ref
        let v0_iter = v.iter(); 
        // a mut ref
        // let v1_iter = v.iter_mut(); 
        // own
        // let v2_iter = v.into_iter();

        for i in v0_iter {
            println!("item: {:?}", i)
        }

    }
    {
        let mut v = [1,2,3,4];
        // an immutable ref
        let mut v_iter = v.into_iter(); 
        assert_eq!(v_iter.next(), Some(1));
        assert_eq!(v_iter.next(), Some(2));
        assert_eq!(v_iter.next(), Some(3));
    }

    {
        let  v = [1,2,3,4];
        // an immutable ref
        let sumsum : i32 = v.iter().sum(); 
        println!("sumsum: {:?}", sumsum);

    }
    {
        // methods that produce other iterators
        // adapter methods
        let  v = [1,2,3,4];
        let mul : Vec<i32>= v.iter().map(|x| x*2).collect();
        println!("mul {:?}", mul);


        let evens : Vec<i32>= v.iter().copied().filter(|x| x % 2 == 0).collect();
        println!("mul {:?}", evens);

    }
}
