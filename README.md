# rust-print
Project to figure out 3d printing in Rust

# Background
I love using [OpenSCAD](http://openscad.org/), however I'm hitting the limits of what it can do. In particular it really struggles without multithreading. Also without a library like [BOSL2](https://github.com/revarbat/BOSL2) it has even more limitations.

To start my first goal is to take output from OpenSCAD (meaning an STL), subdivide/mutate it and then import it back in. This should be done in a multithreaded, SIMD-aware style.

However that's a lot to bite off, so I'm going to start simple and work my way up.

# Project Is NOT
I think its important to define what a project isn't just as much as what it is. At the moment I'm not trying to make a general CSG CAD package. This is just aiming at solving some accute modeling pain from OpenSCAD. If that changes, I'll adjust this statement.