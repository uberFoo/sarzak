# Implementation Details

The implementation is divided into sub-projects.
Generally these are crates, with the exception of Cuckoo, which is a weird beast.
Subsequent sections go into detail on each sub-project.

This entire experiment began years ago, not long after I left _Project Technology_ in 2002.
Since then I have tried to make this a reality numerous times with little success until now.
I blame this on suddenly finding myself having copious spare time...
The most recent impetus was a desire to get back into electronics and programming microcrotrollers.
For some reason, probably because it's what we ostensibly did at PT, this always makes me want to model a solution.
So I did the natural thing, and started building a modeling solution.
I fully expect to blink an LED[^led] before this is all said and done.

The first thing I did was create data structures in Rust to store my model.
This was the genesis of [nut](crates/nut.md).
I then began creating instances of my structures by hand.
That didn't last long, and I decided I could quickly build a tool to draw instances.
Thus was born [cuckoo](crates/cuckoo.md).
After cuckoo was able to produce a model, I modeled the model, of course.
And built an ad-hoc code generator to create Rust code from the model — hence [sarzak](crates/sarzak.md).

The basic workflow is:

1. Draw a model in cuckoo — cuckoo stores the model as a JSON file
2. Nut loads the JSON into Rust data structures
3. The data is mutated until it's suitable for code generation
4. Generate Code

Interestingly, if your model is the _metamodel_ the net result is that you wind up with _3._ above.
In fact, some of the work in _3._ is done using code that was generated from the _drawing_ model.

[^led]: The [hello world](https://www.embeddedrelated.com/showarticle/460.php) of microcontrollers!
