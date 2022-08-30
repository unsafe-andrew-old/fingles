# fingles
A functional rewrite of the [schindel](https://github.com/dapper91/schindel/) library, the goal is to make the library more extensible and open. The perfomance is at the first place, i just don't think that such a simple library should have a lot of boilerplate

For the algorithm description refer to the [original repository README](https://github.com/dapper91/schindel/blob/master/README.md)

NOTE: The crate can only be built with Nightly Rust because it's using const generics inference, which is currently only available in the Nightly Rust

TODO:
- Saner hashing(functions-based), multiple hashing algorithm support

Example:

``` rust
#![feature(generic_arg_infer)]
use fingles::*;

const HASH_LEN: usize = 100;
const NGRAM_LEN: usize = 5;

fn main() {
    let original = "\
        “My sight is failing,” she said finally. “Even when I was young I could not have read what was written there. \
        But it appears to me that that wall looks different. Are the Seven Commandments the same as they used to be, \
        Benjamin?” For once Benjamin consented to break his rule, and he read out to her what was written on the wall. \
        There was nothing there now except a single Commandment. It ran:\
        ALL ANIMALS ARE EQUAL BUT SOME ANIMALS ARE MORE EQUAL THAN OTHERS";

    let plagiarism = "\
        “My sight is failing,” she said finally. “When I was young I could not have read what was written there. \
        But it appears to me that that wall looks different. Are the Seven Commandments the same as they used to be” \
        Benjamin read out to her what was written. There was nothing there now except a single Commandment. \
        It ran: ALL ANIMALS ARE EQUAL BUT SOME ANIMALS ARE MORE EQUAL THAN OTHERS";

    let other = "\
        Throughout the spring and summer they worked a sixty-hour week, and in August Napoleon announced that there \
        would be work on Sunday afternoons as well. This work was strictly voluntary, but any animal who absented \
        himself from it would have his rations reduced by half. Even so, it was found necessary to leave certain \
        tasks undone. The harvest was a little less successful than in the previous year, and two fields which \
        should have been sown with roots in the early summer were not sown because the ploughing had not been \
        completed early enough. It was possible to foresee that the coming winter would be a hard one.";

    let original_hash = hash_from_string::<HASH_LEN, NGRAM_LEN>(original.chars());

    let plagiarism_hash = hash_from_string::<HASH_LEN, NGRAM_LEN>(plagiarism.chars());
    println!("plagiarism similarity: {}", compare_hashes::<_, NGRAM_LEN>(&original_hash, &plagiarism_hash));

    let other_hash = hash_from_string::<HASH_LEN, NGRAM_LEN>(other.chars());
    println!("other text similarity: {}", compare_hashes::<_, NGRAM_LEN>(&original_hash, &other_hash));
}
```
