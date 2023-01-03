# BQN Web Framework (name comprised of bacon pun pending)

This project is primarily a way for me to further my understanding of the BQN
language. BQN is an array programming language, similar to APL or J. My
understanding is that languages like these are used by scientists, wizards, and
people with oversized datasets. I however am a web developer so I decided to do
the web developer thing of building yet another web framework.

## How?

BQN doesn't have any networking built in. Rather than stopping and learning APL
which does, like a sane person, I decided to crack open the CBQN FFI and have
the web server portion and have a function which has an input and output
similar to the ruby rack.

# TODO

- [ ] URL encodeing and decoding
- [ ] Parameter parsing
	- [x] Basic query parameter parsing
	- [ ] Query parameter parsing supporting arrays and objects
	- [ ] JSON Body parsing
- [ ] JSON parsing
- [x] JSON stringifying
- [ ] HTML templating
- [ ] Routing
	- [x] Basic routing
	- [ ] Argument routes
	- [ ] Optional routes? // I've never been a fan of optional routes
	- [ ] Wildcards? // Can be useful at the end of a url. A query param could
				sufice though
- [ ] Persistant storage
	- [ ] Decide between Mongo or Postgres
	- [ ] Use FFI to interact with either database
	- [ ] Build an ORM
		- [ ] Selecting
		- [ ] Inserting
		- [ ] Updating
		- [ ] Deleting
