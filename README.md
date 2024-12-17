Nebula is an HTTP Server created as a simple project to learn Rust. 
It's an simple tcp listener that listen at some port and receives a container path. 
The container path is just a folder where it start to list dir's.

To test it, in cargo.toml (binary crate) add the nebula package reference:

```
[dependencies]
nebula = { path = "path/to/nebula" }
```

In your program, create a server instance and run it.

```
use nebula::server::{self, Server};

fn main() {
  let mut server = Server::new(String::from("/"), 7070).expect("Invalid path");
  server.start().expect("Connection failed.");
}
```

The path at creation of Server instance can be altered to read the files and dir's
from an specified directory.

If you use a tcp client (E.g browser) to connect with it, you gonna see something like this:
![image](https://github.com/user-attachments/assets/e171faa8-3952-44eb-bf05-d81cf5a664be)

You can click in a dir to see files and dirs inside it. 
Since the objective is just to build the http server, the 'functionality' to see files or nested archives 
in isn't important, but the way how it's build. 

It's just made for fun with the purpose to learn rust programming language
