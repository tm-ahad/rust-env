//!# rust-env
//!
//! rust-env is a package to make managing env
//! a lot easier in rust

use std::env::vars;
use std::fs::{read_to_string, write};

#[allow(dead_code)]
struct SPair(String, String);

/// Using this trait you can extend the behavior
/// of Env struct
/// # Example
/// ```
/// use rust_env::{EnvFrame, Hash};
///
/// struct MyEnv {
///     data: Vec<Hash>,
///      global: Vec<Hash>,
///      path: String
/// }
///
/// impl EnvFrame for MyEnv {
///     //write your code
/// }
///```

pub trait EnvFrame {
    fn marshal(val: Vec<Hash>) -> String;
    fn parse(content: String) -> Vec<Hash>;
    fn new(name: String) -> Env;
    fn get(&self, k: &str) -> Wrapper;
    fn get_debug(self) -> Vec<Hash>;
    fn set(&mut self, k: &str, v: Hash);
    fn debug(self);
    fn upload(path: &str, pairs: Vec<Hash>) -> Env;
    fn global_env(&mut self);
    fn get_local(&self, k: &str) -> Wrapper;
    fn get_global(&self, k: &str) -> Wrapper;
}

#[allow(non_snake_case)]
#[allow(dead_code)]
pub fn Str(a: &str, b: &str) -> Hash {
    Hash::Str(a.to_string(), b.to_string())
}

#[allow(non_snake_case)]
#[allow(dead_code)]
pub fn Vct(a: &str, v: Vec<&str>) -> Hash {
    let mut v_: Vec<String> = vec![];

    for e in v.into_iter() {
        v_.push(e.to_string());
    }

    Hash::Vec(a.to_string(), v_)
}

fn get_d(d: Vec<Hash>, key: String) -> Wrapper {
    for h in d.into_iter() {

        match h {
            Hash ::Str(k, v)
            if k == key => {
                return Wrapper::Str(v)
            },
            Hash ::Vec(k, v)
            if k == key => {
                return Wrapper::Vec(v)
            },
            _ => continue
        }
    }

    return Wrapper::Empty;
}

#[derive(Debug, Clone)]
/// This enum mainly rap two type of data
/// String and `Vec<String>`
/// See docs of `struct Env` of this package for learn more
pub enum Wrapper {
    Str(String),
    Vec(Vec<String>),
    Empty
}

/// This enum is mainly A key-value pair
/// `String` and `Vec<String>`
/// See docs of `struct Env` of this package for learn more
/// Here's The enum looks like
/// # Example
/// ```
/// pub enum Hash {
///     Str(String, String),
///     Vec(String, Vec<String>),
///     Placeholder
/// }
///```

#[derive(Debug, Clone)]
pub enum Hash {
    Str(String, String),
    Vec(String, Vec<String>),
    Placeholder
}

pub struct Env {
    data: Vec<Hash>,
    global: Vec<Hash>,
    path: String
}

impl Env {
    #[allow(dead_code)]
    ///It parse a env string
    ///Here's a example
    /// # Example
    ///```
    /// use rust_env::{Env, Hash};
    ///
    /// let env: Vec<Hash> = Env::parse("PORT=6778\nHOST=127.0.0.1");
    /// ```
    pub fn parse(content: &str) -> Vec<Hash> {
        let s = content.to_string();
        let lines = s.split("\n").collect::<Vec<&str>>();
        let mut res: Vec<Hash> = Vec::new();

        for _lines in lines.iter() {
            let pair_ = _lines.split("=").collect::<Vec<&str>>();
            let raw_value = pair_[1];

            #[allow(unused_assignments)]
                let mut value: Hash = Hash::Placeholder;

            match raw_value.find(";") {
                Some(_) => {
                    let raw = raw_value.split(";").collect::<Vec<&str>>();
                    let mut str_vec: Vec<String> = Vec::new();

                    for r in raw.into_iter() {
                        str_vec.push(r.to_string());
                    }
                    value = Hash::Vec(
                        pair_[0].to_string(),
                        str_vec)
                }
                None => {
                    value = Hash::Str(pair_[0].to_string(),
                                      pair_[1].to_string())
                }
            }

            res.push(value);
        }

        return res;
    }

    /// that will Marshal a piece of data like this
    /// # Example
    /// ```
    /// use rust_env::{Env, Hash, Str};
    ///
    /// let d: Vec<Hash> = vec![
    ///     Str("PORT", "6779")
    /// ];
    ///
    /// assert_eq!(Env::marshal(d), "PORT=6779".to_string())
    /// ```
    pub fn marshal(val: Vec<Hash>) -> String {
        let mut hash = String::new();

        for v in val.into_iter() {
            hash.push_str(match v.clone() {
                Hash::Str(a, _) => a,
                Hash::Vec(a, _) => a,
                _ => String::new()
            }.as_str());
            hash.push('=');
            hash.push_str(match v.clone() {
                Hash::Str(_, b) => b,
                Hash::Vec(_, v) => {
                    let mut s = String::new();

                    for v in v.clone() {
                        s.push_str(v.as_str());
                        s.push(';')
                    }

                    s
                },
                Hash::Placeholder=> String::new()
            }.as_str());
        }

        return hash;
    }

    /// New function create a new Env;
    ///
    /// # Example
    /// ```
    /// use rust_env::Env;
    /// let env = Env::new("./.env");
    ///
    /// //debug
    /// env.debug();
    #[allow(dead_code)]
    pub fn new(name: &str) -> Env {
        let content = read_to_string(name.clone()).expect("Invalid path");
        let local = Env::parse(content.as_str());

        Self {
            data: local,
            path: name.to_string().clone(),
            global: Vec::new()
        }
    }

    #[allow(dead_code)]
    /// It will return the entire env
    /// local and global
    /// # Example
    /// ```
    /// use rust_env::{Env, Hash};
    ///
    /// let env = Env::new("./.env");
    /// let e: Vec<Hash> = env.get_debug();
    /// ```

    pub fn get_debug(self) -> Vec<Hash> { return self.data.clone() }
    #[allow(dead_code)]

    /// it will set a prop to your local env
    /// Here's an example
    /// # Example
    /// ```
    /// use rust_env::{Env, Str, Wrapper};
    ///
    /// let mut env = Env::new("./.env");
    /// env.set(Str("PORT", "6778"));
    /// ```
    pub fn set(&mut self, h: Hash) {

        self.data.push(h.clone());
        let hash = Env::marshal(vec![h]);

        write(&self.path, hash).expect(
            "Invalid path to write")
    }

    /// It's similar to the `set` function
    /// But you'll put raw string as parameter
    /// # Example
    /// ```
    /// use rust_env::Env;
    /// use rust_env::Str;
    ///
    /// let mut env = Env::new("./.env");
    /// //Using set function
    /// env.set(Str("PORT", "6778"));
    ///
    /// //Using raw function
    /// env.raw("PORT=6778");

    pub fn raw(&mut self, e: &str) {
        let mut h = Env::parse(e);
        self.data.append(&mut h);

    }


    #[allow(dead_code)]
    /// It's a function to debug the entire env
    /// # Example
    /// ```
    /// use rust_env::Env;
    /// let env: Env = Env::new("./.env");
    ///
    /// env.debug();
    /// ```

    pub fn debug(&self) { println!("{:?}", self.data) }
    #[allow(dead_code)]
    /// You can # upload config data to a env file.
    /// It's similar to the `new` function
    /// But you can write external data to the
    /// env file
    /// # Example
    /// ```
    /// use rust_env::{Env, Str, Vct};
    ///
    /// let env = Env::upload("./env", vec![
    ///       Str("PORT", "6778"),
    ///       Vct("IP", vec![
    ///          "127",
    ///           "0",
    ///           "0"
    ///       ])
    /// ]);
    /// ```
    pub fn upload(path: &str, pairs: Vec<Hash>) -> Env {
        let mut data: Vec<Hash> = Vec::new();
        let mut hash = String::new();

        for pair in pairs.into_iter() {
            match pair.clone() {
                Hash::Str(a, b) => {
                    data.push(pair.clone());

                    hash.push_str(&*a);
                    hash.push('=');
                    hash.push_str(&*b);
                    hash.push_str("\n");
                },
                Hash::Vec(a, vector) => {
                    data.push(Hash::Vec(a.clone(), vector.clone()));

                    let mut raw_literal = a;
                    raw_literal.push('=');

                    for ve in vector.clone().into_iter() {
                        raw_literal.push_str(&*ve);
                        raw_literal.push_str(";")
                    }

                    hash.push_str(&*raw_literal);
                }
                _ => {}
            }
        }
        write(path, hash).expect(
            "Invalid path to write");

        Self {
            data,
            global: Vec::new(),
            path: path.to_string()
        }
    }

    ///You can upload the global env data on the environment
    ///# Example
    ///```
    /// use rust_env::Env;
    ///
    /// let mut env: Env = Env::upload("./.env", vec![
    ///     //put your local config
    /// ]);
    ///
    /// env.debug();
    /// env.global_env();
    /// env.debug();
    #[allow(dead_code)]
    pub fn global_env(&mut self) {
        for (k, v) in vars() {
            self.global.push(Hash::Str(k, v));
        }
    }

    #[allow(dead_code)]
    /// get_local is similar to `get_hash`
    /// But, You can just gt the local config
    /// Not the global
    /// # Example
    /// ```
    /// use rust_env::Wrapper;
    ///
    /// let port = match get_local("PORT") {
    ///     Wrapper::Str(v) => v,
    ///     e => e
    /// };
    pub fn get_local(&self, k: &str) -> Wrapper {
        get_d(self.data[..].to_vec(), k.to_string())
    }

    #[allow(dead_code)]
    /// get_global is similar to `get_hash`
    /// But, You can just gt the local config
    /// Not the global
    /// # Example
    /// ```
    /// use rust_env::Wrapper;
    ///
    /// let path = match get_global("PATH") {
    ///     Wrapper::Str(v) => v,
    ///     e => e
    /// };
    pub fn get_global(&self, k: &str) -> Wrapper { get_d(self.global[..].to_vec(), k.to_string()) }

    /// It will print the global env
    #[allow(dead_code)]
    pub fn debug_global(&self) { println!("{:?}", self.global) }

    /// It will print the local env
    /// # Example
    /// ```
    /// use rust_env::Env;
    ///
    /// let mut env = Env::new("./.env");
    /// env.global_env();
    ///
    /// //printing just global env
    /// env.debug_global();
    ///
    /// //printing just local env
    /// env.debug_local()
    /// ```
    #[allow(dead_code)]
    pub fn debug_local(&self) { println!("{:?}", self.data) }

    #[allow(dead_code)]
    /// You can get data from the Env
    /// # Example
    /// ```
    /// use rust_env::Wrapper;
    /// let port = match env.get_hash("PORT") {
    ///     Wrapper::Str(d) => d,
    ///     _ => String::new()
    /// };
    ///
    /// let ip = match env.get_hash("IP") {
    ///       Wrapper::Vec(v) => v,
    ///       e => e
    /// };
    /// ```
    /// `get_hash` returns a Wrapper enum
    pub fn get_hash(&mut self, k: &str) -> Wrapper {
        return match get_d(self.global[..].to_vec(), k.to_string()) {
            Wrapper::Empty => get_d(self.data[..].to_vec(), k.to_string()),
            e => e
        };
    }
}
