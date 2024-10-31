// at the end what will satisfite me the progressing, seeing with your own eye. Eliminating possible future by feet-back loop.
// I find out 100 ways of not working, and be better. I might nail this or I might about to fail. Let's find out.
// keep going by the things are unfolding. (patten maching) Sloving a puzzle, unfolding.

// any task, that align with their own core passion, Linux. Inner & public feetback loop. Inner is the perpose.

// How to reduce unsertenty by doing the thing.

// spiritual development, difficulties must be faced there for learning to accur.
// with will power, good comes out, and evil is exposed. the effects has to be tolorable. consicunses is made easyer for you. You we belong, and you we will return. (Worse sicuation a o son-ga ahca. Indeed Allah is with them as well)
// It is impossible for the one who gives compassion to have none. who's the destresser.

// God wants good.
// God have given to people a blessing called the partial will to demostrate their ability. My good thinking has to be willed or happend. You have to do good.
// (A effects of others will be here)

// use std::marker::Sized;

fn main() {
    println!("Hello, world!");

    // ***************************************************************************************************************
    // ********************************************** FUNCTIONS & CLOUSERS *******************************************
    // ***************************************************************************************************************

    fn str_transformation(stringable_stucture: &str) {
        let mut some_text = stringable_stucture.to_uppercase(); // it's the refence pointer which is auto derefernced
                                                                // let some = *stringable_stucture.to_uppercase(); // it will derefernce, the acual content, sturutre it will invoke the method. // maybe it dobuold derefernced.
        println!("{}", &some_text); // displays the acutal value by auto * derefenceing.
        println!("{:p}", &some_text); // Pointer trait displays the actual address.
                                      // use the `Display` trait: `{}`
                                      // use the `Debug` trait: `?`
                                      // use the `LowerExp` trait: `e`
                                      // use the `UpperExp` trait: `E`
                                      // use the `Octal` trait: `o`
                                      // use the `Pointer` trait: `p`
                                      // use the `Binary` trait: `b`
                                      // use the `LowerHex` trait: `x`
                                      // use the `UpperHex` trait: `X`
        println!("{:<15}", &some_text); // 15 characters left.

        // don't lose owership means doesn't lose underline value.
        // for cahanging underline value derefernce to get the content.
        // some_text.push_str("words"); // when you reference to a string, it's the original string that you're updating.
    }

    // will live long enought or not to referce that. Think of it which scop owns the stucture.

    fn some() -> &'static str {
        "something" // permanently allocating in the static storage. So it will always live long engouht.
    }

    // You can only have 1 mutable refence at a time. to change the acutal value. address has changed. case the size has change. it has to find an empty memoriy location of that size.
    // other ways pointing to invalide refence, dangling reference.

    // A nested fn isn't allowed to access varibles definded in the outer scope fn. it can only access it's paramiters. paramiter type and return type has to be especify.
    // Closure overcome those constraints and can also assigned to a variable & invoke that. (pointer to a closure)
    // first time invoke paramiter type will be define.
    // when logic is simple use closure, for simplecity.
    // captures variables the underline value, ownership moves (not really) and captures that mutable, immutable depending in you're useage.
    let some = |a, _b| a; // type anotation needed until you invoke one.
    some("some".to_string(), 39);

    let mut text_1 = String::from("Hello");
    let mut text_2 = String::from("Worlds"); // can not do imutable things, if it's not mutable in the first place.
    let mut text_update = || {
        text_1.push_str("bar");
        text_1.push_str(&text_2);
    };

    // std::mem::drop(text_2); // drop function acquire ownership of that stucture type.

    text_update(); // inside the closure: ownership moves not really means compiler will create a whole new sturcure type, just like copy or assign.
    println!("{} {}", text_1, text_2); // force move happed when you are inside a thread.

    // The closure will converted into a stucture. That will have external variables:
    // struct text_update {
    //     text_1: &mut String, // TO DO: missing lifetime specifier
    //     text_2: &String, // TO DO: missing lifetime specifier
    // }

    // depending on the useage of external variable, the closure type is defind. FnMut()

    // in a clouser if you try to destroy a variable, that kind of fn only be called once. So the closure type is FnOnce.

    // ***************************************************************************************************************
    // ************************************************** ARRAY/ELEMENTS *********************************************
    // ***************************************************************************************************************
    // operation performing on each element needs clouser, To say what operation to perform. It's more fexibule.
    // each element in a collection.
    let animations_names = vec!["Loong", "hover", "trans", "pop"];

    // filter for true false operation. return type has to true or false and true once will return.
    animations_names.iter().filter(|e| e.starts_with('p'));

    // Map for transforming operation.
    animations_names.iter().map(|e| e.to_uppercase());

    // for_each for doing operation.
    animations_names.iter().for_each(|e| println!("{}", e)); // iter() for itirating over a collection

    // So, Filter the elements, transform them, and use them.
    let upper_animations_name = animations_names // return type anotation
        .iter()
        .filter(|e| e.ends_with("s"))
        .map(|e| e.to_uppercase())
        .collect::<Vec<String>>(); // or return type anotation

    // when you are iterating, not by for loop which can get massy. heard to rap your head around, the operations.

    // ***************************************************************************************************************
    // ************************************************** FUNCTIONALITY IMP ******************************************
    // ***************************************************************************************************************
    // Defined the shape of the sturcture in stack mamory:
    struct Employee {
        name: String,
        salary: u8,
        full_time: bool,
    }

    // passing a whole stucture into a function:
    // - enables you to pass a lot of data into a function as a consive boundle
    // - much better than passing a whole load of individual paramiters
    // - if passing stucture by value: by copy, the location or address transfer to another address then *ownership move and then previce address becomes canceal or gone.
    fn emp_cal_handle(e: Employee) {} // doesn't imp copy trait, it move ownership

    // & is a ponter. we can accass it's value by explicit derefence * or by implicit derefence.
    fn bonase(e: &mut Employee) -> Employee {
        // return value is not mutable.
        let emp_2 = Employee {
            // this can be mut after wars. this menas return by value. because you are returning by value, you can mut in that instance.
            name: "something".to_string(),
            salary: 4800,
            full_time: true,
        };
        emp_2
    }
    let mut emp_1 = Employee {
        name: "something".to_string(),
        salary: 3800,
        full_time: true,
    };
    let mut emp_1_update = bonase(&mut emp_1); // if one is mut everywhere it has to be mut. without & we can not just drop the value.
    println!("{:?}", emp_1);

    // if muliputl arugments, to stablish the same lifetime between argument & ruturn types. fn some<'a>(e1: &'a mut Employee, e2: &'a mut Employee) -> &'a mut Employee {}

    impl Employee {
        fn some(self: &Self) {} // self = interface stucture = Self
        fn some(self: &Employee) {} // self = Employee instance = Employee
                                    // Or
        fn some(&self) {} // self = instance //*** any trait fn it has access to instance to operate on it. it will invoke like this: some(&enstance) // (there is nothing specal about that coupling)
    } // this could be mutable. so you have assece to it's instance (when it's instantiate) and interface stucture.

    // ***************************************************************************************************************
    // ************************************************ SPLIDING/ORGONIATION *****************************************
    // ***************************************************************************************************************

    // crate means the application. Ex: use crate::main::Employee; or let emp: crate:main::Employee;
    // in main.rs: mod filename // mod for importing folders and files. First look for folder then file. mod is for importing here this file only.
    // Inside that, inner stucutre could import more files and folders. But for here only. only to this file
    // pub is to expose it one level up. (okay, if anyone imports it, what are things they can assecss)
    // so, if someone is importing this current file. inside this file, mod for import a file and that imported file, I can pub to upper level to also import that.
    // the way it expose I'll use it like this. Ex: use crate::mytypes::point::Point

    // For simplicity: one file, one stucutre in one module and it's implementation.
    //
    // ***************************************************************************************************************
    // ***************************************************** METHODS *************************************************
    // ***************************************************************************************************************
    // associate or static fn doesn't have accass to self, intance. we didn't give the accass case we are not gonna operate on it rather it define functionality concerning or invoke functionality to the whole sturcture. Ex:
    let init_text = String::from("hellow worlds"); // gives you a whole stucture back.

    // extra data that is shared by whole file or in stucuture implementation: const data/variable, static data/variable that can change.Both lifetime is valid throught out the scop.

    // something that will give you an external type that your file doesn't have. and you wanna match on that something. then you have to import that external type.

    // if you invoke a method which is defined in a trait then you also import or define that trait in current file.

    // a stucture muliple traits is like multiple inharatence of factionalityes. means a stucture has mulitpule implemntation blocks.

    // To pass or return muliple stuctures in a fn or a vec or a hashmap. &dyn trait_name. stuctures that have same operation porformed, same functionality, same behabiar. // and they're fn will call when the program is running, not at compile time.
    // when muliple things could be passed in or return then passed at runtime or return it at runtime. // (it's called dynamically dispatched, dynamically send off, push through, discharge)
    // build functionality over functionlity which is to build cumulative interface, kro-mo-bor-tho-man interface, increasing day by day.Extents over extents, inherits over inherits form other thing. trait over traits
    trait AnotherTrait {}
    trait SomeTrait: AnotherTrait {
        // SomeTrait is inheriting funtionality form AnotherTrait which could have inhertence form other traits.
        // incorporte functionality of AnotherTrait into here and all the other traits inhartence that AnotherTrait has to offer. // Like a adapter API - TODO see later.

        // So I wanna implement this functinality, How to you do it? How do you benifit form this?

        // display, drop, clone trait you are implemaneting to a instance, of a stucture. By trait implementation it has access to the stucture or instance that it will operate on. To give the functionality to it. where you want to perform this functionality or operation.
        // must override functionality has external helper stuctures and it's mathod that you can import. To facilitate, to make possiable to implment that paticular functionality.
        // and to imp a trait to the whole custom stucture that has it's fileds, which has that trait mathoed build in. You just need to wrap with those trait mathod and constract the whole stucture. to have that trait to the whole stucture.

        // with ginaric factionality traits:
        // can't predict outside, argument ginaric stucture that has that fuctionality in order to use that mathod or behavier. Ex:
        fn display_arry<T: std::fmt::Display>(arr: &[T]) {
            // constraint the mathods that can use in here. // this fuctionalitys this can use.
            for e in arr {
                println!("{}", e); // e.fmt()
            }
        }
        // you can use derive to give non-overwrite functionality on a stucture and if each filed implements that functionality and super level inharant trait functionality has to derive as well. // which trait requare which?

        // for inserting costom stucture type into hashmap. Hash trait has to imp for hash algorith to be porfom on that stucture.
    }
    struct Employe<T> {
        // a ginaric type
        name: String,
        salary: u8,
        full_time: T,
    }

    let mut emp_1 = Employe::<i8> {
        // to specify the ginarice structure type. // original stucture doesn't specify so I have to specify.
        // but it can gess.
        name: "something".to_string(),
        salary: 3800,
        full_time: 1,
    };

    fn any_array<T>(arr: &[T]) {
        // std::mem::size_of()
        let size = std::mem::size_of::<T>();
    }

    any_array::<i32>(&[34, 293, 2, 209]);

    // ***************************************************************************************************************
    // *************************************************** PARALEL COMPUTING *****************************************
    // ***************************************************************************************************************
    // to spawn a thread:
    std::thread::spawn(|| {
        std::thread::current().id();
        // std::thread::sleep(DurationType);
    }); // after spawning main thread will immediately continue to execute in paralel.

    // for waiting, you will get something:
    // let thread_handle: JoinHandle = std::thread::spawn();
    // match thread_handle.join() { // join waits to finish sussecfully or error.
    //     Ok(value) => ;
    //     Err(e) => ;
    // }

    // for all the threads to wait and get:
    // let mut threads_handles: Vec<std::thread::JoinHandle::<_>> = vec![]; // _ means ginaric stucture
    // threads_handles.push(thread_handle);
    // for thread in threads_handles {
    //     let value = thread.join().unwrap()
    //     // now instart into a vec, hashmap anything.
    // }

    // capture data in a thread closure by only moving. (because if main thread finshed first, orignal value doesn't exit, so borrowing or pointing would be dangling reference)
    // by |move| it forecfull take ownership for as long as the thread needs it. what if this thread outlives main/any.

    // Why this? don't know. doesn't need wait and match, more simpler.
    // For transmitting/sending and receiving data:
    //     ----
    //        | tx
    //  _____________
    // | | channel   |
    // |_|___________|
    //        | rx
    //     ----
    // By creating a channel it return a sender and receiver:
    let (sender, receiver) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        // doesn't need wait and match.
        sender.send(String::from("Hi")).unwrap();
    });
    let received_data = receiver.recv().unwrap(); // could be many. send & recive instantly and updated.

    for data in receiver {
        println!("{}", data); // send and recive. send and recive instantly. Completely in sync. Thay are working concurrently.
    }

    // Box<T> box rapper stucture type knows size at compile time becaus it lives on the stack, a pointer. and T lives on the heap which can be infinitly growable and infinte size. Ex: linked list which is reference to it's self, next element, infinitely nested inside. Go to the end by finding null at the end and create add. They call themselves to step to the next node.

    // move or ownership move means origal value moved. address changed. after that refencing or pointing will be invalid.

    // Rc reference counter for single threaded, by increasing counter a type is allocated on the heap. (basical like anything: A pointer on the stack, refenceing to a heap. by inceasing, shared pointer. points to the same thing. and gives shared ownership) (and owns the data, How?)
    // by cloning count increased, by out of scope lifetime end count is decrased. // it implements clone trait.
    // Not allowed sending bettewn threads or mutable refenceing. for mutated RefCell inside Rc and std::sync::Arc for atomic reference counter.

    let fooo = std::rc::Rc::new(vec![1.0, 2.0, 3.0]);
    let a = fooo.clone();
    let b = std::rc::Rc::clone(&fooo); // same thing as clone. // (but Rc::clone(&from) syntax is the most idiomatic because it conveys more explicitly the meaning of the code. // a and b both point to the same memory location as foo.)
    drop(a);

    let foo = std::sync::Arc::new(vec![1.0, 2.0, 3.0]);
    let a = foo.clone();
    let b = std::sync::Arc::clone(&foo); // same thing as clone. it can send bettwen threads. it auto perform lock.

    // Sharing a mutable iniger refenence: // update has ordering hasal
    let val = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(5));
    let vall = std::sync::Arc::clone(&val);

    // The downgrade method can be used to create a non-owning Weak pointer. A Weak pointer can be upgraded to an Rc, but this will return None if the value stored in the allocation has already been dropped.
    // To observe it. Not actual use. it's used in binary try.
    let my_weak = std::rc::Rc::downgrade(&fooo);
    let my_weak = std::sync::Arc::downgrade(&foo);

    // ganaral safety: when you derefence a pointer/refence, it's potentially dangerous.
    let mut x = 29;
    let p = &mut x;
    let p: *mut i32 = &mut x; // expiciply storing address of x in p.

    // ***************************************************************************************************************
    // ***************************************************** PARSING *************************************************
    // ***************************************************************************************************************
    // rust code to JSON: serizlized:
    // Creating JSON by serializing data structures:
    use serde::{Deserialize, Serialize};
    use serde_json::Result;

    #[derive(Serialize, Deserialize)]
    struct Address {
        street: String,
        city: String,
    }

    let address = Address {
        street: "10 Downing Street".to_owned(),
        city: "London".to_owned(),
    };

    // by enstend to string:
    // Serialize rust code, to a JSON string that you can print, write to a file, or send to an HTTP server.
    // we could serialize rust code to a JSON string, JSON byte vector and data structure as JSON into the I/O stream. // by to_string, to_vec, to_writer
    let j = serde_json::to_string(&address).unwrap();
    println!("{}", j);

    // by enstand to writing the writer:
    // fist rust code serialized into JSON then writing to writer:
    // first given data structure convert as JSON then into the I/O stream in one go:
    serde_json::to_writer(writer, &address); // takes ownership. can not use writer for later writing: writer.write_all(b"\n")?;
    serde_json::to_writer(&mut *writer, &address);

    // Note: [ // mutable reference to reborrow/reused again. // deref kora acual valuer mutable reference create korse. Maybe    // by mutable refenence ar fola compiler creates another reference hoe ja value ka move kora na. // case mutable refence is still a refence. ]

    // -------------------------------------------------------

    // JSON to rust code: deserialized:
    // Parsing JSON as strongly typed data structures:
    // use serde::{Deserialize, Serialize};
    // use serde_json::Result;

    fn typed_example() {
        // I can have JSON string, JSON bytes, I/O stram of JSON (that comes form the user). // by from_str, from_slice, from_reader
        let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

        #[derive(Serialize, Deserialize)]
        struct Person {
            name: String,
            age: u8,
            phones: Vec<String>,
        }

        // by JSON file reading & storing it as String then, String to rust code stucture:
        let p: Person = serde_json::from_str(data).unwrap(); // Read the JSON contents as an instance of `Person`. type anotation needed.

        // by JSON file reading & reader to rust code stucture:
        let file = std::fs::File::open("test.json").unwrap();
        let reader = std::io::BufReader::new(file);
        let p: Person = serde_json::from_reader(reader).unwrap(); // type anotation needed. importent case Read the JSON contents of the file as an instance of `Person`.

        // by JSON reading from socket connection:
        use std::net::{TcpListener, TcpStream};
        let listener = TcpListener::bind("127.0.0.1:4000").unwrap();

        for stream in listener.incoming() {
            let mut de = serde_json::Deserializer::from_reader(stream.unwrap());
            let p = Person::deserialize(&mut de)?;
            Ok(p)
        }
    }

    // Note: Maybe for extra feature you can create serializer or deserializer and use it. How? don't know yet.
    // rust code output can serialized with the serializer, a destination to send rust code and convert to JSON. A serializer is a writer.
    // JSON input can deserialized with the deserializer, a resiver to resived JSON and converts. A deserializer is a reader.
}

// ***************************************************************************************************************
// ***************************************************** WRITING FUNTIONS ****************************************
// ***************************************************************************************************************
// fn takes external converted stucture. That stucture type has to espasicy. Just the name and the ginaric stucture type has to say.
// what stucture type is allowed in <>, it says in the Implementations of that transformed external stucutre docs.
// normal imp block what trait it implement, how can I create one.
// TODO: https://docs.rs/serde_json/latest/serde_json/struct.Serializer.html
// TODO: https://docs.rs/serde_json/latest/serde_json/struct.Deserializer.html
