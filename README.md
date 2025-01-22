# secs - Shit Entity Component System
**secs** is an ECS for people who want something with the bare minimum. No complex features, no unnecessary abstractions — just the essentials, no strings attached (but you could do that).

If you’re tired of ECS libraries that make you feel like you’re doing your PhD in systems design, **secs** is here to give you what you need and nothing more

## Why secs?
You could use something like **hecs**, **specs** or **bevy** for your ECS needs, but why chase frills when you could have uncomplicated **secs**? It's simple, lightweight and may (or may not) get the job done. No promises. If you need more, you can always try something else.

***secs** — without the fluff*

## Features
- **Entity Management**: Sure, you can spawn entities with IDs. But are they actually useful? Who knows?
- **Component Storage**: Components are stored in sparse sets — sounds fancy, right? But it’s more of a hope-and-pray situation.
- **Queries**: Querying components? Sure, go ahead. The results might surprise you..
- **Mutation**: Mutating components? You can do that! Just don’t try anything too wild like multiple mutable components. We’ll tackle that someday.. Hopefully.
- **Systems**: Right now, **secs** doesn’t have systems but eventually if shit works out, it might.

## Getting Started
Get **secs**

```bash
cargo add --git https://github.com/wick3dr0se/secs
```

Example: How it’s probably supposed to work.
```rust
use secs::World;

let mut world = World::default();
let entity = world.spawn();

world.attach(entity, MyComponent { /* your data */ });

for (entity, (component,)) in world.query::<(&MyComponent,)>() {
    // maybe get components
}
```

## Contributing
Could always use some of that.