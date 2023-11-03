# Shopkeeper

A file sharing program for trusted networks or separately verified data

## Usage

```bash
shopkeeper serve <path> <interface>
```
Serves a directory on a given interface and port. The default interface is `0.0.0.0:6677`. The default path is the current working directory.

```bash
shopkeeper cat <server> <resource-path>
```
Reads the contents of a file on the server. Directories are identified by a trailing slash, this is required for the request to work if the target is a directory. If resource-path is omitted, the root of the server is listed. Paths don't start with a slash, the path `/` is invalid.

```bash
shopkeeper fetch <server> <node-id>
```
Obtains the text contents of a particular numeric node ID from the server.

## Protocol

This program uses an extension of the Unsecure File Sharing Protocol. A copy of the original RFC is included in this repository. This protocol was a coursework submission at the University of Surrey for Computer Networking (COM2022) in the spring semester of 2022.

## Roadmap

I have several ideas for the protocol and the client. The project is shelved for the moment as I'm focusing on [Orchid](https://github.com/lbfalvy/orchid), but once I find a use case or regain some enthusiasm these are the things I would like to add:

- I would like to add encryption in a way that doesn't influence the performance of the protocol. UFSP is designed to be connectionless and I want to retain that, basing my encryption scheme most likely on a streaming cipher and an initial shared secret

- The client was initially intended to download resources on multiple threads concurrently, this was a pivotal feature of the protocol that set it apart from TCP-based protocols. I ran out of time before I could implement it, but it's still on the roadmap

- The server should cache slices, and it should also precache subsequent slices of requested files

- I would like to introduce a lot of extensibility to the protocol. Requests can have a body but currently this serves no purpose. Feature discovery can be implemented using a second preallocated node ID (most likely -1). Implementors are required to ignore unknown message types, so more operations can be defined using these. Request bodies are limited to 381 bytes, but extension handlers might discard the connectionless nature of the protocol to accept multipart requests using the last slice index field.

## Contributing

Anyone is welcome to submit PRs or fork this repo subject to the terms of the GNU General Public License version 3. Suggestions in issues are also welcome