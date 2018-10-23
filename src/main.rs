use std::collections::HashMap;

macro_rules! new_map {
    ($($key : expr => $val : expr),+) => {
        {
            let mut map = HashMap::new();

            $(
                map.insert($key, $val);
            )*

            map
        }
    };
}

macro_rules! list_comp {
    ($id1 : ident for $id2 : ident in range($start : expr; $end : expr) if $cond : expr) => {{
        let mut vec = Vec::new();

        for num in $start..$end + 1 {
            if $cond(num) {
                vec.push(num);
            }
        }

        vec
    }};

    ($id1 : ident for $id2 : ident in $list : expr ,if $cond : expr) => {{
        let mut vec = Vec::new();

        let it = $list.iter();

        for num in it {
            if $cond(num) {
                vec.push(num);
            }
        }

        vec
    }};
}

macro_rules! print_expr {
    ($e : expr) => {
        println!("{:?} = {:?}", stringify!($e), $e);
    };
}

macro_rules! calc {
    (eval $e: expr) => {
        {
            // let val : usize = $e;
            println!("{} = {}", stringify!($e), $e);
        }



    };

    (eval $e: expr, $(eval $es : expr),+) => {
        {
            calc!{ eval $e}
            calc!{ $(eval $es),+}
        }
    };
}

pub fn main() {
    println!("Print Expressions:");
    {
        print_expr!({
            let x = 5;
            let y = 2;
            x * y
        });
    }

    println!("\nEasy HashMap:");
    {
        let my_map = new_map!{
            "one" => 1,
            "two" => 2,
            "three" => 3
        };
        println!("{:?}", my_map);
    }

    println!("\nList Comprehension:");
    {
        let x = list_comp![x for x in range(1;10) if |x| x % 2 == 0];
        println!("{:?}", x);
        let y = list_comp![y for y in [1, 4, 10, 31, 65] ,if |x| x % 2 != 0];
        println!("{:?}", y);
    }

    println!("\nCalculate Expressions:");
    {
        calc!{
            eval 4 * 5,
            eval 10 - 6,
            eval 4 << 3,
            eval i32::pow(5, 2)
        };
    }
}
