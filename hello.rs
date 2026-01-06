fn main() {
    println!("Hellow world!");
    let x = 5 + /* 90 + */ 5;
    println!("is `x` 10 or 100? x = {}", x);

   println!("{0}, this is {1}, {1} this is {0}", "Alice", "Bob"); 

   println!("{subject} <-- subject", subject="foo");

   let y = 69420;
   println!("base 10: {}", y);
   println!("binary: {:b}", y);
   println!("octal: {:o}", y);
   println!("hex: {:x}", y);

   /* Convert a string to bytes */
   let f = "foo";
   let bytes: &[u8] = f.as_bytes(); // borrowed
   println!("binary word: {bytes:?}");

   // pad and 'left-adjust' 
   println!("{number:0>5}",number=1); // 00001
   println!("{number:0<5}",number=1); // 10000
   
   // use passed var's with appending a `$`
   println!("{number:0>width$}", width=5, number=1);

   // will fail if the number of args past don't match what was defined
   // in the string
   println!("my name is {1}, {0} {1}", "james", "bond");

   /* this will allow dangling definitions/code */
   // #[allow(dead_code)]
   struct Structure(i32);
   /* Uncomment below to see errors */
   /* This will not compile because `Structure` does not implement 
    * fmt::Display */
   // println!("This struct `{}` won't print...", Structure(3));

   let number: f64 = 1.0;
   let width: usize = 5;
   println!("{number:>width$}");
}
