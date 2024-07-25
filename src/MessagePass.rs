use std:: sync::mpsc;
use std:: thread;

fn main(){

    let (sender, receiver) = mpsc::channel();

    thread::spawn(move ||{

        let val = String::from("I was created in the child thread, will be sent to the main thread");

        sender.send(val).unwrap();

    });

    let received = receiver.recv().unwrap();
    println!("I have received this message from the child thread :{}", received)

}