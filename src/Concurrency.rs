use std::thread;
use std::time::Duration;



fn main(){

   let secondary_thread =  thread::spawn(||{

        for i in 1..5{
            println!("Secondary thread Prints {}", i);
            thread::sleep(Duration::from_millis(2));
        }
    });


    for i in 1..5 {

        println!("Main thread prints {}", i );

        thread::sleep(Duration::from_millis(1));
    }

    secondary_thread.join.unwrap();


}