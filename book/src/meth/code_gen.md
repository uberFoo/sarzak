# Code Generation

## The Meta-Model

The meta-model, or metamodel, as I sometimes spell it, is a model of the model.
Put another way, the result of generating code on the meta-model should be exactly what you started with.

The meta-model contains objects called, *Object*, *Attribute*, and *Relationship*.
These are the elements that cuckoo draws.
These are the types that a model compiler operates upon.

Making changes to the meta-model, and regenerating it, is equivalent, in it's end result, to hand editing Rust code.
The difference is that instead of hand editing code, you are editing an abstract representation of code.
Changing the model compiler would allow you to generate code in a different language, for example.
Models are also reusable, much in the way that classes were supposed to be reusable.

## Model Compilers

Think compiler for models.
Rather than generating assembly language, or machine code, a model compiler generates source code.

One of these turns JSON into code; currently Rust.
The model compiler in Sarzak 1.0 is an ad-hoc compiler that is really only suited to generating domain code.

Work is well underway on a replacement.
It's called grace, and it's currently generating domain code, and some application code.
Relationship navigation is still to do, while the test generation framework is nearly complete.

### Options / Coloring

### Domain Code

### Application Code

## Domain Extrusion

I'm throwing this in here, because I don't know where else to put it.
It's a process that really has limited application, and yet it's used extensively in nut, and some in sarzak.
The process comes into play when you have a domain model with existing instances, and you want to change the *shape* of the data.
Perhaps this falls under "shape", but it's also used to synthesize new types from existing types.

This method relies on a sinngle trait.

```Rust
pub trait Extrude<T, C> {
    fn extrude(input: T, context: &mut C) -> Self;
}
```

The `Extrude` trait is pretty simple, but fairly powerful.
The key is in teh `context` generic parameter.
What this is depends on what the `extrude` method needs to accomplish.

For example, in the extrusion from a nut domain into a sarzak domain, the context looks like this.

```Rust
struct Context<'a> {
    sarzak: &'a SarzakObjectStore,
    drawing: &'a mut DrawingObjectStore,
    id: Option<Uuid>,
}
```

It is during this phase of extrusion that we are converting our objects from one store to another, so it makes sense that we'd have both in the context.
The `to` store is mutable, because that is where the new instances are written.

This is the definition of `Object` that I started with (ooa_0).

```Rust
pub struct Object {
    pub id: Uuid,
    pub key_letter: String,
    pub name: AttributeName,
    pub description: String,
    pub attributes: HashMap<Uuid, Attribute>,
}
```

This is the first change that I introduced (ooa_1).

```Rust
pub struct Object {
    pub id: Uuid,
    pub key_letter: String,
    pub name: AttributeName,
    pub description: String,
    pub attributes: HashMap<String, Attribute>,
    pub rels: HashMap<String, RelPointer>,
    pub is_referrer: bool,
}
```

For fun, here is the `Object` definition that is generated from the sarzak model.
There is an extrude implementation for it as well.

```Rust
/// An anchor, or anchor point, is the location where an arrow from a relationship attached to an object.
///
/// Rather than storing the `x` and `y` coordinates of where the anchor attaches,
/// we are related to an [Edge], which is related to a box, which is related to
/// the [Object] to which we are attached. This of course completes the circuit
/// from the [Relationship] for which we are drawing the lines in the first place.
///
/// Anchor also contains a direction, so that we know the orientation to draw the
/// arrows. Finally, there is an offset, which is a point that describes the offset
/// from the anchor for the relationship phrase.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Anchor {
    /// pub id: `Uuid`,
    pub id: Uuid,
    /// pub edge: `Edge`,
    pub edge: Uuid,
    /// pub location: `Point`,
    pub location: Uuid,
    /// pub offset: `Point`,
    pub offset: Uuid,
}
```
