Nebula is an HTTP Server created as a simple project to learn Rust. 

To test it, clones the project with ``git clone`` and use ``cargo run path/to/container/`` 
to use it. The container path is the path where the server goes to search for archives.

After that, in a browser (since we only implemented http protocol), type 127.0.0.1:8080. 
If in your folder you have index.html, automatically going to pop up on your screen. 

At this point, I have only implemented thread workers, but the objective is to implement process_pool or use 
``epoll()`` syscall, like nginx use it.
