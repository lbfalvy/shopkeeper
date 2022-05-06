This document is an assessment of my implementation in the context of the
coursework.

# Github

My implementation's source code is available at
[https://github.com/lbfalvy/shopkeeper#Roadmap]

# Conformance testing

The protocol fundamentally defines three points of compatibility

- Retrieval of a specific slice from a node
- Retrieval of the complete contents of a node using the above operation
- Enumeration of available files using the above operation on node 0

As implementors and authors of a standard we didn't work completely
independently, rather the development process was dotted with constant
negotiation of the unspecified protocol details and subsequent addition
of these to the RFC document. As a result, all of the above operations
are completely functional across all 3 implementations. We did encounter
some compatibility issues when initially comparing implementations but we
promptly fixed them.

# Proprietary extensions

My implementation relies on a trailing slash in the name to distinguish
subfolders within the root, upon access the contents of these folders
follow the same structure as node 0. My client also supports accessing
files by path, where the case of a single element path (a single file
name) is equivalent to the case described in the standard. As far as I
know, no one else implemented this extension.

I also have a number of ideas for further protocol extensions, described
in the (Roadmap section of the README)[https://github.com/lbfalvy/shopkeeper#Roadmap].

# Designing for Robustness

Being a binary protocol, UFSP leaves almost no room for error that
doesn't also introduce ambiguity. That being said, my client ignores all
lines in directory listings that don't conform to the specification. This
is very intentional to allow for other proprietary extensions, but it
also proves useful with servers that - inexplicably - add prompts
addressed to the user in the directory index.
