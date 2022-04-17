# Programming Game AI by Example

* [Book Source](https://github.com/wangchen/Programming-Game-AI-by-Example-src)

## West World (Chapter 2)

* [Alternative Rust State Machine Pattern](https://hoverbear.org/blog/rust-state-machine-pattern/)

### Bevy version

* Bevy Entity replaces the Entity struct
* State exit / enter is handled using Bevy events
  * This was done to allow the systems handling those events to query as needed
  * TODO: I'm pretty sure this is all kinds of bugged and sending events across more entities than it should
* Messaging and regular updates run at the same 800ms as the original version
* State enter / exit / on message run per-frame due to limitations with bevy's event system

## Autonomous (Chapter 3)

* The Actor component replaces the BaseGameEntity
* Lots of incomplete steering behaviors here due to time restrictions

## Soccer (Chapter 4)

* This builds on west-world-bevy and autonomous, including all the problems that show up in those projects
