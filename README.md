Generating [integer compositions](https://en.wikipedia.org/wiki/Composition_(combinatorics)).

By default, the compositions of each integer are listed in the lexicographic order.

Command line arguments (all of them are optional):

* an integer number *n*: get the compositions for the numbers from 0 to *n* (the default is 6);
* `colex`: reverse each composition after ordering (i. e., use the colexicographic order instead);
* `reverse`: reverse the lexicographic ordering;
* `len+`/`len-`: group the compositions by length and order the groups by increasing/decreasing length;
* `-1`: subtract 1 from each term (this allows listing all tuples of nonnegative integers);
* `flat`: instead of pretty formatting, output all terms in one line, like in the OEIS.

On Windows, compile with `--features clip` to put the "flat" output to the clipboard. This adds dependencies.

The pretty output of `cargo run 4` looks like this:
```
[[]]
[[1]]
[[1, 1], [2]]
[[1, 1, 1], [1, 2], [2, 1], [3]]
[[1, 1, 1, 1], [1, 1, 2], [1, 2, 1], [1, 3], [2, 1, 1], [2, 2], [3, 1], [4]]
```

OEIS sequences for the compositions and the corresponding command line arguments:

* [A228369](https://oeis.org/A228369): (no arguments)
* [A228525](https://oeis.org/A228525): `colex`
* [A066099](https://oeis.org/A066099): `reverse`
* [A108730](https://oeis.org/A108730): `reverse -1`
* [A228351](https://oeis.org/A228351): `reverse colex`
* [A163510](https://oeis.org/A163510): `reverse colex -1`
* [A124734](https://oeis.org/A124734): `len+`
* [A124735](https://oeis.org/A124735): `len+ -1`
* [A337243](https://oeis.org/A337243): `len+ colex`
* [A296774](https://oeis.org/A296774): `len+ reverse`
* [A337259](https://oeis.org/A337259): `len+ reverse colex`
* [A296773](https://oeis.org/A296773): `len-`
* [A337260](https://oeis.org/A337260): `len- colex`
* [A296772](https://oeis.org/A296772): `len- reverse`
* [A108244](https://oeis.org/A108244): `len- reverse colex`

The content of this repository is licensed under [Apache License 2.0](https://www.apache.org/licenses/LICENSE-2.0) or [CC BY 4.0](https://creativecommons.org/licenses/by/4.0/) at your option.
